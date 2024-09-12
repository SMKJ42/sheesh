use std::error;

use chrono::{DateTime, Utc};

use crate::{
    harness::{
        sqlite::{SqliteDiskOpSession, SqliteDiskOpToken},
        DiskOp, IntoValues,
    },
    hash::{DefaultHashGenerator, HashGenerator},
};

use super::{
    auth_token::{AuthToken, AuthTokenManager, AuthTokenManagerConfig},
    get_token_expiry,
    id::{DefaultIdGenerator, IdGenerator},
};

pub struct SessionManagerConfig<T, U>
where
    T: IdGenerator,
    U: HashGenerator,
{
    id_generator: T,
    token_generator_config: AuthTokenManagerConfig<T, U>,
    fields: Vec<String>,
    ttl: i64,
}

impl SessionManagerConfig<DefaultIdGenerator, DefaultHashGenerator> {
    pub fn new_default() -> Self {
        return Self {
            id_generator: DefaultIdGenerator {},
            token_generator_config: AuthTokenManagerConfig::new_default(),
            ttl: 120,
            fields: vec![
                "id".to_string(),
                "user_id".to_string(),
                "refresh_token".to_string(),
                "auth_token".to_string(),
                "expires".to_string(),
            ],
        };
    }

    pub fn with_fields(&mut self, fields: Vec<String>) {
        self.fields.extend(fields);
    }
}

impl<T, U> SessionManagerConfig<T, U>
where
    T: IdGenerator + Copy,
    U: HashGenerator + Copy,
{
    pub fn init<V: DiskOp, X: DiskOp>(
        &self,
        session_harness: V,
        token_harness: X,
    ) -> SessionManager<T, U, V, X> {
        return SessionManager {
            id_generator: self.id_generator,
            token_generator: self.token_generator_config.init(token_harness),
            harness: session_harness,
            fields: self.fields.to_owned(),
            ttl: self.ttl,
        };
    }
}

pub struct SessionManager<T, U, V, X>
where
    T: IdGenerator,
    U: HashGenerator,
    V: DiskOp,
    X: DiskOp,
{
    id_generator: T,
    token_generator: AuthTokenManager<T, U, X>,
    fields: Vec<String>,
    harness: V,
    ttl: i64,
}

impl
    SessionManager<DefaultIdGenerator, DefaultHashGenerator, SqliteDiskOpSession, SqliteDiskOpToken>
{
}

impl<T, U, X> SessionManager<T, U, SqliteDiskOpSession, X>
where
    T: IdGenerator,
    U: HashGenerator,
    X: DiskOp,
{
}

impl<T, U, V, X> SessionManager<T, U, V, X>
where
    T: IdGenerator,
    U: HashGenerator,
    V: DiskOp,
    X: DiskOp,
{
    pub fn new_session(
        &self,
        user_id: u64,
    ) -> Result<(Session, AuthToken, AuthToken), Box<dyn error::Error>> {
        let id = self.id_generator.new_u64();

        let auth_token = self.token_generator.next_token()?;
        let refresh_token = self.token_generator.next_token()?;

        let expires = get_token_expiry(self.ttl)?;

        let session = Session {
            id,
            user_id,
            refresh_token: Some(refresh_token.id()),
            auth_token: Some(auth_token.id()),
            expires,
        };
        self.harness.insert(&session, &self.fields)?;

        return Ok((session, auth_token, refresh_token));
    }

    pub fn refresh_session_token(
        &self,
        session: &mut Session,
    ) -> Result<AuthToken, Box<dyn error::Error>> {
        let new_token = self.token_generator.next_token()?;

        unimplemented!();

        return Ok(new_token);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Session {
    id: u64,
    user_id: u64,
    refresh_token: Option<u64>,
    auth_token: Option<u64>,
    expires: DateTime<Utc>,
}

impl IntoValues for Session {
    fn into_values(&self) -> Vec<String> {
        return vec![
            self.id.to_string(),
            self.user_id.to_string(),
            self.refresh_token.unwrap_or(0).to_string(),
            self.auth_token.unwrap_or(0).to_string(),
            self.expires.to_string(),
        ];
    }
}

/// Session
///
/// Only has getters to prevent accidental overwrites.
impl Session {
    pub fn id(&self) -> u64 {
        return self.id;
    }

    pub fn user_id(&self) -> u64 {
        return self.user_id;
    }
}
