use std::error;

use crate::db::{
    sqlite::{SqliteDiskOpSession, SqliteDiskOpToken},
    DiskOp,
};

use super::{
    auth_token::{AuthToken, AuthTokenGenerator},
    hash::{DefaultHashGenerator, HashGenerator},
    id::{DefaultIdGenerator, IdGenerator},
};

pub struct SessionManager<T, U, V, X>
where
    T: IdGenerator,
    U: HashGenerator,
    V: DiskOp,
    X: DiskOp,
{
    id_generator: T,
    token_generator: AuthTokenGenerator<T, U, X>,
    db_harness: V,
}

impl
    SessionManager<DefaultIdGenerator, DefaultHashGenerator, SqliteDiskOpSession, SqliteDiskOpToken>
{
    pub fn init_default() -> Self {
        return Self {
            id_generator: DefaultIdGenerator {},
            token_generator: AuthTokenGenerator::init_default(),
            db_harness: SqliteDiskOpSession {},
        };
    }
}

impl<T, U, X> SessionManager<T, U, SqliteDiskOpSession, X>
where
    T: IdGenerator,
    U: HashGenerator,
    X: DiskOp,
{
    pub fn init(id_generator: T, token_generator: AuthTokenGenerator<T, U, X>) -> Self {
        return Self {
            id_generator,
            token_generator,
            db_harness: SqliteDiskOpSession {},
        };
    }
}

impl<T, U, V, X> SessionManager<T, U, V, X>
where
    T: IdGenerator,
    U: HashGenerator,
    V: DiskOp,
    X: DiskOp,
{
    pub fn new_session(&self, user_id: u64) -> Result<(Session, AuthToken), Box<dyn error::Error>> {
        let id = self.id_generator.new_u64();
        let new_token = self.token_generator.next_token(id)?;

        return Ok((
            Session {
                id,
                user_id,
                current_token_id: new_token.id(),
            },
            new_token,
        ));
    }

    pub fn refresh_session_token(
        &self,
        session: &mut Session,
    ) -> Result<AuthToken, Box<dyn error::Error>> {
        let new_token = self.token_generator.next_token(session.id)?;
        session.current_token_id = new_token.id();

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
