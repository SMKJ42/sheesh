use std::{
    error,
    fmt::{self, Display},
};

use crate::{
    harness::{sqlite::SqliteDiskOpToken, DiskOp},
    hash::{DefaultHashGenerator, HashGenerator},
};

use super::id::{DefaultIdGenerator, IdGenerator};
use chrono::{offset::LocalResult, DateTime, TimeDelta, Utc};

pub struct AuthTokenManagerConfig<T, U>
where
    T: IdGenerator,
    U: HashGenerator,
{
    ttl: i64,
    id_generator: T,
    hash_generator: U,
}

impl AuthTokenManagerConfig<DefaultIdGenerator, DefaultHashGenerator> {
    pub fn new_default() -> Self {
        return Self {
            ttl: 30,
            id_generator: DefaultIdGenerator {},
            hash_generator: DefaultHashGenerator {},
        };
    }
}

impl<T, U> AuthTokenManagerConfig<T, U>
where
    T: IdGenerator + Copy,
    U: HashGenerator + Copy,
{
    pub fn init<V: DiskOp>(&self, connection: V) -> AuthTokenManager<T, U, V> {
        AuthTokenManager {
            ttl: self.ttl,
            id_generator: self.id_generator,
            hash_generator: self.hash_generator,
            connection,
        }
    }
}

pub struct AuthTokenManager<T, U, V>
where
    T: IdGenerator,
    U: HashGenerator,
    V: DiskOp,
{
    ttl: i64,
    id_generator: T,
    hash_generator: U,
    pub connection: V,
}

impl<T, U> AuthTokenManager<T, U, SqliteDiskOpToken>
where
    T: IdGenerator,
    U: HashGenerator,
{
}

impl<T, U, V> AuthTokenManager<T, U, V>
where
    T: IdGenerator,
    U: HashGenerator,
    V: DiskOp,
{
    pub fn next_token(&self, session_id: u64) -> Result<AuthToken, Box<dyn error::Error>> {
        let id = self.id_generator.new_u64();

        let token = AuthToken::new(id, session_id, self.ttl)?;
        self.connection.insert(&token)?;

        return Ok(token);
    }
}

pub struct AuthToken {
    id: u64,
    session_id: u64,
    expires: DateTime<Utc>,
    is_valid: bool,
}

impl AuthToken {
    /// AuthToken::new() should only be called from AuthTokenManager.
    fn new(id: u64, session_id: u64, ttl: i64) -> Result<Self, Box<dyn error::Error>> {
        let now = Utc::now();
        let time_delta = TimeDelta::minutes(ttl);

        let (new_time, rem) = now.time().overflowing_add_signed(time_delta);
        let rem = chrono::Days::new(rem as u64);
        let now_add_day = now.checked_add_days(rem);

        match now_add_day {
            Some(some_now_add_day) => {
                let expires = some_now_add_day.with_time(new_time);
                match expires {
                    LocalResult::Single(expires) => {
                        return Ok(AuthToken {
                            expires,
                            id,
                            session_id,
                            is_valid: true,
                        })
                    }
                    LocalResult::Ambiguous(expires, _) => {
                        return Ok(AuthToken {
                            expires,
                            id,
                            session_id,
                            is_valid: true,
                        })
                    }
                    LocalResult::None => {
                        return Err(Box::new(AuthTokenError::new(
                            AuthTokenErrorKind::InvalidExpiration,
                        )))
                    }
                }
            }
            None => {
                return Err(Box::new(AuthTokenError::new(
                    AuthTokenErrorKind::InvalidExpiration,
                )))
            }
        }
    }

    pub fn is_valid(&mut self) -> bool {
        // if token has been manually invalidated, or if the token is expired return false else return true;
        if self.is_valid == false {
            return false;
        } else {
            return self.expires < Utc::now();
        }
    }

    pub fn invalidate(&mut self) {
        self.is_valid = false;
    }

    pub fn id(&self) -> u64 {
        return self.id;
    }
}

#[derive(Debug)]
pub struct AuthTokenError {
    kind: AuthTokenErrorKind,
}

impl AuthTokenError {
    fn new(kind: AuthTokenErrorKind) -> Self {
        return Self { kind };
    }
}

impl Display for AuthTokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl error::Error for AuthTokenError {}

#[derive(Debug)]
pub enum AuthTokenErrorKind {
    InvalidExpiration,
}

impl Display for AuthTokenErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Self::InvalidExpiration => write!(f, "InvalidExpiration"),
        };
    }
}
