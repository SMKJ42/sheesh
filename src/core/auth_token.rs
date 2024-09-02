use std::{
    error,
    fmt::{self, Display},
    hash::Hash,
};

use super::{
    id::{DefaultIdGenerator, IdGenerator},
    token::{DefaultHashGenerator, HashGenerator},
};
use chrono::{offset::LocalResult, DateTime, TimeDelta, Utc};

pub struct AuthTokenGenerator<T, U>
where
    T: IdGenerator,
    U: HashGenerator,
{
    ttl: i64,
    id_generator: T,
    hash_generator: U,
}

impl<T, U> AuthTokenGenerator<T, U>
where
    T: IdGenerator,
    U: HashGenerator,
{
    pub fn init(id_generator: T, hash_generator: U, ttl: i64) -> Self {
        Self {
            ttl,
            id_generator,
            hash_generator,
        }
    }

    pub fn default() -> AuthTokenGenerator<DefaultIdGenerator, DefaultHashGenerator> {
        AuthTokenGenerator {
            ttl: 8,
            id_generator: DefaultIdGenerator::init(),
            hash_generator: DefaultHashGenerator::init(),
        }
    }

    pub fn next_token(&self, session_id: u64) -> Result<AuthToken, Box<dyn error::Error>> {
        let id = self.id_generator.new_u64();
        // we dont want to force a reattempt, leave implementation up to the developer.
        return AuthToken::new(id, session_id, self.ttl);
    }
}

pub struct AuthToken {
    pub id: u64,
    pub session_id: u64,
    pub expires: DateTime<Utc>,
    pub is_valid: bool,
}

impl AuthToken {
    /// AuthToken::new() should only be called from AuthTokenGenerator.
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
                            expires: expires,
                            id,
                            session_id,
                            is_valid: true,
                        })
                    }
                    LocalResult::Ambiguous(expires, _) => {
                        return Ok(AuthToken {
                            expires: expires,
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
