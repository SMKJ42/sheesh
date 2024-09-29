use std::error;

use crate::harness::{
    sqlite::{SqliteHarnessSession, SqliteHarnessToken},
    DbHarnessSession, DbHarnessToken,
};

use super::{
    auth_token::{
        AuthToken, AuthTokenError, AuthTokenErrorKind, AuthTokenManager, AuthTokenManagerConfig,
        TokenManagerError, TokenTtl,
    },
    id::{DefaultIdGenerator, IdGenerator},
};

// Session naming convention may be a bit misleading. it is really to handle the refresh token on the auth server iteself...
// the client server will also have a session entity representing the user's session within the application
pub struct SessionManagerConfig<T>
where
    T: IdGenerator,
{
    id_generator: T,
    token_manager_config: AuthTokenManagerConfig<T>,
    ttl: i64,
}

impl Default for SessionManagerConfig<DefaultIdGenerator> {
    fn default() -> Self {
        return Self {
            id_generator: DefaultIdGenerator {},
            token_manager_config: AuthTokenManagerConfig::default(),
            ttl: 240,
        };
    }
}

impl<T> SessionManagerConfig<T>
where
    T: IdGenerator + Copy,
{
    pub fn init<V: DbHarnessSession, Y: DbHarnessToken>(
        &self,
        session_harness: V,
        token_harness: Y,
    ) -> SessionManager<T, V, Y> {
        return SessionManager {
            id_generator: self.id_generator,
            harness: session_harness,
            ttl: self.ttl,
            token_manager: self.token_manager_config.init(token_harness),
        };
    }
}

pub struct SessionManager<T, V, X>
where
    T: IdGenerator,
    V: DbHarnessSession,
    X: DbHarnessToken,
{
    id_generator: T,
    token_manager: AuthTokenManager<T, X>,
    harness: V,
    ttl: i64,
}

impl SessionManager<DefaultIdGenerator, SqliteHarnessSession, SqliteHarnessToken> {}

impl<T, X> SessionManager<T, SqliteHarnessSession, X>
where
    T: IdGenerator,
    X: DbHarnessToken,
{
}

impl<T, V, X> SessionManager<T, V, X>
where
    T: IdGenerator,
    V: DbHarnessSession,
    X: DbHarnessToken,
{
    pub fn new_session(
        &self,
        user_id: i64,
    ) -> Result<(Session, String, String), Box<dyn error::Error>> {
        let id = self.id_generator.new_u64();

        let (auth_token, auth_secret) = self.token_manager.next_token(user_id, TokenTtl::Access)?;

        let (refresh_token, refresh_secret) = self
            .token_manager
            .next_token(user_id, TokenTtl::Refresh(self.ttl))?;

        let session = Session {
            // shift to bytes then into i64 (DO NOT CAST, we want to preserve the bit values)
            id: i64::from_be_bytes(id.to_be_bytes()),
            user_id,
            refresh_token: Some(refresh_token.id()),
            auth_token: Some(auth_token.id()),
        };
        self.harness.insert(&session)?;

        return Ok((session, auth_secret, refresh_secret));
    }

    pub fn read_session(&self, id: i64) -> Result<Session, Box<dyn error::Error>> {
        return self.harness.read(id);
    }

    pub fn create_new_access_token(
        &self,
        session: &mut Session,
        user_id: i64,
    ) -> Result<String, Box<dyn error::Error>> {
        // cleanup old token
        if session.auth_token.is_some() {
            self.token_manager
                .delete_access_token(session.auth_token.unwrap())?;
        }

        // create a new token, the 'token_manager' harness will handle database insertion.
        let (new_token, auth_token_secret) =
            self.token_manager.next_token(user_id, TokenTtl::Access)?;

        session.auth_token = Some(new_token.id());

        //update the session with the new token id.
        self.harness.update(session)?;

        return Ok(auth_token_secret);
    }

    pub fn create_new_refresh_token(
        &self,
        mut session: Session,
        token_str: String,
        user_id: i64,
    ) -> Result<String, TokenManagerError> {
        let refresh_token_res: Result<Option<AuthToken>, Box<dyn error::Error>>;

        // if the session has a None value in the session token, there is no 'old' refresh token to check.
        match session.refresh_token {
            Some(token_id) => {
                refresh_token_res = self.token_manager.get_refresh_token(token_id);
            }
            None => {
                return Err(AuthTokenError::new(AuthTokenErrorKind::NotAuthorized).into());
            }
        }

        /*
         *   this is a nasty block of code, apologies.
         */

        // check for harness error.
        match refresh_token_res {
            // check to ensure we obtained a token.
            Ok(refresh_token) => {
                match &refresh_token {
                    // if we obtained a token, validate it.
                    Some(token) => {
                        match self
                            .token_manager
                            .verify_token(token.clone(), user_id, token_str)
                        {
                            Ok(_) => {
                                // The provided token is valid, we can continue...
                            }

                            Err(err) => {
                                match err.kind {
                                    AuthTokenErrorKind::Expired | AuthTokenErrorKind::Invalid => {
                                        // if we hit this area, someone has accessed an expired or invalidated refresh token.
                                        // When this happens, we want to invalidate the session and return an error.
                                        self.set_token_ids_none(session)?;

                                        // Change the error to reflect the new state of the session.
                                        return Err(AuthTokenError::new(
                                            AuthTokenErrorKind::NotAuthorized,
                                        )
                                        .into());
                                    }
                                    _ => {
                                        // propogate the wildcard error
                                        return Err(err.into());
                                    }
                                }
                            }
                        }
                    }

                    // No token found in the database, return an error...
                    None => {
                        return Err(AuthTokenError::new(AuthTokenErrorKind::NotAuthorized).into());
                    }
                }
            }

            // the harness failed to fetch the provided refresh token, return an error...
            Err(err) => return Err(TokenManagerError::Harness(err)),
        }

        // create the new token
        let (new_token, refresh_token_secret) = self
            .token_manager
            .next_token(user_id, TokenTtl::Refresh(self.ttl))?;

        // save the token to the session
        session.refresh_token = Some(new_token.id());
        match self.harness.update(&session) {
            Ok(()) => return Ok(refresh_token_secret),
            Err(err) => return Err(TokenManagerError::Harness(err)),
        }
    }

    pub fn set_token_ids_none(&self, mut session: Session) -> Result<(), TokenManagerError> {
        session.auth_token = None;
        session.refresh_token = None;
        match self.harness.update(&session) {
            Err(err) => return Err(TokenManagerError::Harness(err)),
            Ok(()) => Ok(()),
        }
    }

    /// currently the equivalent to Self::set_token_ids_none(), but there are plans to change sessions to
    /// have their own lifetime seperate from the refresh_tokens.
    pub fn invalidate_session(&self, session: Session) -> Result<(), TokenManagerError> {
        return self.set_token_ids_none(session);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Session {
    id: i64,
    user_id: i64,
    refresh_token: Option<i64>,
    auth_token: Option<i64>,
}

impl Session {
    pub fn id(&self) -> i64 {
        return self.id;
    }

    pub fn user_id(&self) -> i64 {
        return self.user_id;
    }

    pub fn refresh_token(&self) -> Option<i64> {
        return self.refresh_token;
    }

    pub fn auth_token(&self) -> Option<i64> {
        return self.auth_token;
    }
}
