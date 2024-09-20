use std::{error, fmt::Display, time::Instant};

use crate::harness::{sqlite::SqliteDiskOpToken, DiskOpToken};

use super::{
    get_token_expiry,
    id::{DefaultIdGenerator, IdGenerator},
    DEFAULT_HASH_FN, DEFAULT_RNG_STR_FN, DEFAULT_VERIFY_TOKEN_FN,
};
use chrono::{DateTime, Utc};

pub struct AuthTokenManagerConfig<'a, T>
where
    T: IdGenerator,
{
    ttl: i64,
    id_generator: T,
    salt_fn: &'a dyn Fn() -> String,
    token_fn: &'a dyn Fn() -> String,
    hash_fn: &'a dyn Fn(&str, &str) -> Result<String, Box<dyn error::Error>>,
    verify_token_fn: &'a dyn Fn(&str, &str) -> Result<(), Box<dyn error::Error>>,
}

impl<'a> AuthTokenManagerConfig<'a, DefaultIdGenerator> {
    pub fn default() -> Self {
        return Self {
            ttl: 30,
            id_generator: DefaultIdGenerator {},
            salt_fn: DEFAULT_RNG_STR_FN,
            token_fn: DEFAULT_RNG_STR_FN,
            hash_fn: DEFAULT_HASH_FN,
            verify_token_fn: DEFAULT_VERIFY_TOKEN_FN,
        };
    }
}

impl<'a, T> AuthTokenManagerConfig<'a, T>
where
    T: IdGenerator + Copy,
{
    pub fn init<V: DiskOpToken>(&self, harness: V) -> AuthTokenManager<'a, T, V> {
        AuthTokenManager {
            ttl: self.ttl,
            id_generator: self.id_generator,
            harness,
            salt_fn: self.salt_fn,
            token_fn: self.token_fn,
            hash_fn: self.hash_fn,
            verify_token_fn: self.verify_token_fn,
        }
    }
}

pub struct AuthTokenManager<'a, T, V>
where
    T: IdGenerator,
    V: DiskOpToken,
{
    ttl: i64,
    id_generator: T,
    salt_fn: &'a dyn Fn() -> String,
    token_fn: &'a dyn Fn() -> String,
    hash_fn: &'a dyn Fn(&str, &str) -> Result<String, Box<dyn error::Error>>,
    verify_token_fn: &'a dyn Fn(&str, &str) -> Result<(), Box<dyn error::Error>>,
    pub harness: V,
}

impl<'a, T> AuthTokenManager<'a, T, SqliteDiskOpToken> where T: IdGenerator {}

impl<'a, T, V> AuthTokenManager<'a, T, V>
where
    T: IdGenerator,
    V: DiskOpToken,
{
    pub fn next_token(&self) -> Result<(AuthToken, String), Box<dyn error::Error>> {
        let id = self.id_generator.new_u64();

        let salt_fn = self.salt_fn;
        let salt = salt_fn();

        let token_fn = self.token_fn;
        let token = token_fn();

        let hash_fn = self.hash_fn;
        let secret = hash_fn(&token, &salt)?;

        let auth_token = AuthToken::new(id, salt, secret.clone(), self.ttl)?;
        self.harness.insert(&auth_token)?;

        return Ok((auth_token, secret));
    }

    pub fn verify_token(
        &self,
        token: AuthToken,
        token_str: String,
    ) -> Result<(), Box<dyn error::Error>> {
        let verify_fn = self.verify_token_fn;
        verify_fn(&token_str, &token.secret)?;
        if token.is_expired() {
            return Err(Box::new(AuthTokenError::new(
                "Token has Expired.".to_string(),
            )));
        }

        return Ok(());
    }
}

#[derive(Clone, Debug)]
pub struct AuthToken {
    id: u64,
    salt: String,
    secret: String,
    expires: DateTime<Utc>,
}

impl AuthToken {
    /// AuthToken::new() should only be called from AuthTokenManager.
    fn new(id: u64, salt: String, secret: String, ttl: i64) -> Result<Self, Box<dyn error::Error>> {
        let expires = get_token_expiry(ttl)?;

        return Ok(AuthToken {
            id,
            salt,
            secret,
            expires,
        });
    }

    pub fn id(&self) -> u64 {
        return self.id;
    }

    pub fn from_values(values: Vec<String>) -> Self {
        unimplemented!()
    }
    pub fn into_values(&self) -> Vec<String> {
        unimplemented!()
    }

    pub fn is_expired(&self) -> bool {
        return Utc::now() > self.expires;
    }

    pub fn salt(&self) -> String {
        return self.salt.to_owned();
    }
}

#[derive(Debug)]
pub struct AuthTokenError {
    message: String,
}

impl AuthTokenError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for AuthTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AuthTokenError: {}", self.message)
    }
}

impl error::Error for AuthTokenError {}
