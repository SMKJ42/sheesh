use std::{
    error,
    fmt::{self, Display},
};

use crate::{
    harness::{sqlite::SqliteDiskOpToken, DiskOp, IntoValues},
    hash::{DefaultHashGenerator, HashGenerator},
};

use super::{
    get_token_expiry,
    id::{DefaultIdGenerator, IdGenerator},
};
use chrono::{DateTime, Utc};

pub struct AuthTokenManagerConfig<T, U>
where
    T: IdGenerator,
    U: HashGenerator,
{
    ttl: i64,
    id_generator: T,
    hash_generator: U,
    fields: Vec<String>,
}

impl AuthTokenManagerConfig<DefaultIdGenerator, DefaultHashGenerator> {
    pub fn new_default() -> Self {
        return Self {
            ttl: 30,
            id_generator: DefaultIdGenerator {},
            hash_generator: DefaultHashGenerator {},
            fields: vec!["id".to_string(), "token".to_string(), "expires".to_string()],
        };
    }
}

impl<T, U> AuthTokenManagerConfig<T, U>
where
    T: IdGenerator + Copy,
    U: HashGenerator + Copy,
{
    pub fn init<V: DiskOp>(&self, harness: V) -> AuthTokenManager<T, U, V> {
        AuthTokenManager {
            ttl: self.ttl,
            id_generator: self.id_generator,
            hash_generator: self.hash_generator,
            fields: self.fields.to_owned(),
            harness,
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
    fields: Vec<String>,
    pub harness: V,
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
    pub fn next_token(&self) -> Result<AuthToken, Box<dyn error::Error>> {
        let id = self.id_generator.new_u64();
        let token = self.id_generator.new_u128().to_string();

        let token = AuthToken::new(id, self.ttl, token)?;
        self.harness.insert(&token, &self.fields)?;

        return Ok(token);
    }
}

#[derive(Clone, Debug)]
pub struct AuthToken {
    id: u64,
    token: String,
    expires: DateTime<Utc>,
}

impl IntoValues for AuthToken {
    fn into_values(&self) -> Vec<String> {
        return vec![
            self.id.to_string(),
            self.token.clone(),
            self.expires.to_string(),
        ];
    }
}

impl AuthToken {
    /// AuthToken::new() should only be called from AuthTokenManager.
    fn new(id: u64, ttl: i64, token: String) -> Result<Self, Box<dyn error::Error>> {
        let expires = get_token_expiry(ttl)?;

        return Ok(AuthToken { expires, id, token });
    }

    pub fn id(&self) -> u64 {
        return self.id;
    }
}

// #[derive(Debug)]
// pub struct AuthTokenError {
//     kind: AuthTokenErrorKind,
// }

// impl AuthTokenError {
//     fn new(kind: AuthTokenErrorKind) -> Self {
//         return Self { kind };
//     }
// }

// impl Display for AuthTokenError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.kind)
//     }
// }

// impl error::Error for AuthTokenError {}

// #[derive(Debug)]
// pub enum AuthTokenErrorKind {
//     InvalidExpiration,
// }

// impl Display for AuthTokenErrorKind {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         return match self {
//             Self::InvalidExpiration => write!(f, "InvalidExpiration"),
//         };
//     }
// }
