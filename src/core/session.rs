use std::error;

use chrono::{DateTime, Utc};

use crate::harness::{
    sqlite::{SqliteDiskOpSession, SqliteDiskOpToken},
    DiskOpSession, DiskOpToken,
};

use super::{
    auth_token::{AuthTokenManager, AuthTokenManagerConfig},
    get_token_expiry,
    id::{DefaultIdGenerator, IdGenerator},
};

pub struct SessionManagerConfig<'a, T>
where
    T: IdGenerator,
{
    id_generator: T,
    token_generator_config: AuthTokenManagerConfig<'a, T>,
    ttl: i64,
}

impl<'a> Default for SessionManagerConfig<'a, DefaultIdGenerator> {
    fn default() -> Self {
        return Self {
            id_generator: DefaultIdGenerator {},
            token_generator_config: AuthTokenManagerConfig::default(),
            ttl: 120,
        };
    }
}

impl<'a, T> SessionManagerConfig<'a, T>
where
    T: IdGenerator + Copy,
{
    pub fn init<V: DiskOpSession, Y: DiskOpToken>(
        &self,
        session_harness: V,
        token_harness: Y,
    ) -> SessionManager<'a, T, V, Y> {
        return SessionManager {
            id_generator: self.id_generator,
            harness: session_harness,
            ttl: self.ttl,
            token_generator: self.token_generator_config.init(token_harness),
        };
    }
}

pub struct SessionManager<'a, T, V, X>
where
    T: IdGenerator,
    V: DiskOpSession,
    X: DiskOpToken,
{
    id_generator: T,
    token_generator: AuthTokenManager<'a, T, X>,
    harness: V,
    ttl: i64,
}

impl<'a> SessionManager<'a, DefaultIdGenerator, SqliteDiskOpSession, SqliteDiskOpToken> {}

impl<'a, T, X> SessionManager<'a, T, SqliteDiskOpSession, X>
where
    T: IdGenerator,
    X: DiskOpToken,
{
}

/// Takes in a user_id, and returns the session and associated secrets ->
/// (session: Session, auth_secret: String, refresh_secret: String)  
impl<'a, T, V, X> SessionManager<'a, T, V, X>
where
    T: IdGenerator,
    V: DiskOpSession,
    X: DiskOpToken,
{
    pub fn new_session(
        &self,
        user_id: u64,
    ) -> Result<(Session, String, String), Box<dyn error::Error>> {
        let id = self.id_generator.new_u64();

        let (auth_token, auth_secret) = self.token_generator.next_token()?;
        let (refresh_token, refresh_secret) = self.token_generator.next_token()?;

        let expires = get_token_expiry(self.ttl)?;

        let session = Session {
            id,
            user_id,
            refresh_token: Some(refresh_token.id()),
            auth_token: Some(auth_token.id()),
            expires,
        };
        self.harness.insert(&session)?;

        return Ok((session, auth_secret, refresh_secret));
    }

    /// Updates the sessions with a new session token ->
    /// secret: String
    pub fn issue_new_auth_token(
        &self,
        session: &mut Session,
    ) -> Result<String, Box<dyn error::Error>> {
        let (new_token, auth_token_secret) = self.token_generator.next_token()?;

        session.auth_token = Some(new_token.id());

        todo!("insert new token into db, cleanup old token, return the secret")
    }

    pub fn issue_new_refresh_token(
        &self,
        session: &mut Session,
    ) -> Result<String, Box<dyn error::Error>> {
        let (new_token, refresh_token_secret) = self.token_generator.next_token()?;

        session.refresh_token = Some(new_token.id());

        todo!("insert new token into db, cleanup old token, return the secret")
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

impl Session {
    pub fn id(&self) -> u64 {
        return self.id;
    }

    pub fn user_id(&self) -> u64 {
        return self.user_id;
    }

    pub fn refresh_token(&self) -> Option<u64> {
        return self.refresh_token;
    }

    pub fn auth_token(&self) -> Option<u64> {
        return self.auth_token;
    }

    pub fn expires(&self) -> DateTime<Utc> {
        return self.expires;
    }

    pub fn from_values(values: Vec<String>) -> Self {
        unimplemented!()
    }

    pub fn into_values(&self) -> Vec<String> {
        unimplemented!()
    }
}
