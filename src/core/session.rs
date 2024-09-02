use std::error;

use crate::{
    auth_token::{AuthToken, AuthTokenGenerator},
    id::IdGenerator,
};

pub struct SessionManager<T>
where
    T: IdGenerator,
{
    id_generator: T,
    token_generator: AuthTokenGenerator<T>,
}

impl<T> SessionManager<T>
where
    T: IdGenerator,
{
    pub fn init(id_generator: T, token_generator: T, ttl: i64) -> Self {
        return Self {
            id_generator,
            token_generator: AuthTokenGenerator::init(ttl, token_generator),
        };
    }

    pub fn new_session(&self, user_id: u64) -> Result<(Session, AuthToken), Box<dyn error::Error>> {
        let id = self.id_generator.new_u64();
        let new_token = self.token_generator.next_token(id)?;

        return Ok((
            Session {
                id,
                user_id,
                current_token_id: new_token.id,
            },
            new_token,
        ));
    }

    /// This function will create a new token, but it is the developer's job to invalidate the old token.

    // while the package can invalidate the token for the developer, it would not enforce persisting
    // the token's invalid state within the database. (the token would only be invalid in mem, not in the fs)
    pub fn refresh_session_token(
        &self,
        session: &mut Session,
    ) -> Result<AuthToken, Box<dyn error::Error>> {
        let new_token = self.token_generator.next_token(session.id)?;
        session.current_token_id = new_token.id;

        return Ok(new_token);
    }
}

pub struct Session {
    id: u64,
    user_id: u64,
    current_token_id: u64,
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

    pub fn current_token_id(&self) -> u64 {
        return self.current_token_id;
    }
}
