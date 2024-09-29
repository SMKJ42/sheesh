use std::{error, fmt::Display};

use crate::harness::{sqlite::SqliteHarnessUser, DbHarnessSession, DbHarnessToken, DbHarnessUser};

use super::{
    auth_token::AuthTokenError,
    default_hash_fn, default_rng_salt_fn, default_verify_token_fn,
    id::{DefaultIdGenerator, IdGenerator},
    session::{Session, SessionManager},
};

pub struct UserManagerConfig<T>
where
    T: IdGenerator,
{
    id_generator: T,
    salt_fn: fn() -> String,
    hash_fn: fn(&str, &str) -> Result<String, AuthTokenError>,
    verify_pass_fn: fn(&str, &str) -> Result<(), AuthTokenError>,
}

impl UserManagerConfig<DefaultIdGenerator> {
    pub fn default() -> Self {
        Self {
            id_generator: DefaultIdGenerator {},
            salt_fn: default_rng_salt_fn,
            hash_fn: default_hash_fn,
            verify_pass_fn: default_verify_token_fn,
        }
    }
}

impl<T> UserManagerConfig<T>
where
    T: IdGenerator + Copy,
{
    pub fn init<V: DbHarnessUser>(&self, harness: V) -> UserManager<T, V> {
        UserManager {
            id_generator: self.id_generator,
            hash_fn: self.hash_fn,
            verify_pass_fn: self.verify_pass_fn,
            salt_fn: self.salt_fn,
            harness,
        }
    }

    pub fn with_id_gen<X: IdGenerator + Copy>(&self, id_generator: X) -> UserManagerConfig<X> {
        return UserManagerConfig {
            id_generator,
            salt_fn: self.salt_fn,
            verify_pass_fn: self.verify_pass_fn,
            hash_fn: self.hash_fn,
        };
    }
}

pub struct UserManager<T, V>
where
    T: IdGenerator,
    V: DbHarnessUser,
{
    id_generator: T,
    harness: V,
    salt_fn: fn() -> String,
    hash_fn: fn(&str, &str) -> Result<String, AuthTokenError>,
    verify_pass_fn: fn(&str, &str) -> Result<(), AuthTokenError>,
}

impl UserManager<DefaultIdGenerator, SqliteHarnessUser> {}

impl<T, V> UserManager<T, V>
where
    T: IdGenerator,
    V: DbHarnessUser,
{
    pub fn create_user<Pu, Pr>(
        &self,
        username: String,
        pwd: String,
        role: Role,
        public: Option<Pu>,
        private: Option<Pr>,
    ) -> Result<User<Pu, Pr>, Box<dyn error::Error>>
    where
        Pu: PublicUserMeta,
        Pr: PrivateUserMeta,
    {
        let id = self.id_generator.new_u64();

        let salt = (self.salt_fn)();
        let secret = (self.hash_fn)(&pwd, &salt)?;

        let user = User::new(
            i64::from_be_bytes(id.to_be_bytes()),
            username,
            secret,
            role,
            public,
            private,
        )?;

        self.harness.insert(&user)?;
        return Ok(user);
    }

    pub fn login<Pu, Pr, Id, Sh, Th>(
        &self,
        session_manager: SessionManager<Id, Sh, Th>,
        user_id: i64,
        pwd: &str,
    ) -> Result<(Session, String, String), UserManagerError>
    where
        Id: IdGenerator,
        Sh: DbHarnessSession,
        Th: DbHarnessToken,
        Pu: PublicUserMeta,
        Pr: PrivateUserMeta,
    {
        let user_res = self.get_user(user_id);
        let user: User<Pu, Pr>;

        match user_res {
            // harness error occured, propogate the err.
            Err(err) => return Err(err.into()),
            Ok(user_opt) => match user_opt {
                // user not found
                None => return Err(UserManagerError::new(UserManagerErrorKind::UserNotFound)),
                Some(q_user) => {
                    // assign user, continue to verify password
                    user = q_user;
                }
            },
        }

        match self.verify_pwd(user, pwd) {
            // Error validating the user, propogate the Error.
            Err(err) => return Err(err.into()),
            Ok(_) =>
            // I need a way to access a session...
            {
                let sess_res = session_manager.new_session(user_id);
                match sess_res {
                    Ok(res) => return Ok(res),
                    Err(err) => return Err(err.into()),
                }
            }
        };
    }

    pub fn verify_pwd<Pu, Pr>(&self, user: User<Pu, Pr>, pwd: &str) -> Result<(), AuthTokenError>
    where
        Pu: PublicUserMeta,
        Pr: PrivateUserMeta,
    {
        (self.verify_pass_fn)(pwd, &user.secret)
    }

    pub fn update_user<Pu, Pr>(&self, user: User<Pu, Pr>) -> Result<usize, Box<dyn error::Error>>
    where
        Pu: PublicUserMeta,
        Pr: PrivateUserMeta,
    {
        return self.harness.update(&user);
    }

    pub fn get_user<Pu, Pr>(&self, id: i64) -> Result<Option<User<Pu, Pr>>, Box<dyn error::Error>>
    where
        Pu: PublicUserMeta,
        Pr: PrivateUserMeta,
    {
        return self.harness.read(id);
    }

    pub fn delete_user<Pu, Pr>(&self, id: i64) -> Result<(), Box<dyn error::Error>>
    where
        Pu: PublicUserMeta,
        Pr: PrivateUserMeta,
    {
        return self.harness.delete(id);
    }
}

#[derive(Clone)]
pub struct User<Pu, Pr>
where
    Pu: PublicUserMeta,
    Pr: PrivateUserMeta,
{
    pub id: i64,
    pub session_id: Option<i64>,
    pub username: String,
    pub secret: String,
    pub ban: bool,
    pub groups: Groups,
    pub role: Role,
    pub public: Option<Pu>,
    pub private: Option<Pr>,
}

impl<Pu, Pr> User<Pu, Pr>
where
    Pu: PublicUserMeta,
    Pr: PrivateUserMeta,
{
    pub fn new(
        id: i64,
        username: String,
        secret: String,
        role: Role,
        public: Option<Pu>,
        private: Option<Pr>,
    ) -> Result<Self, Box<dyn error::Error>> {
        return Ok(Self {
            id,
            username,
            secret,
            ban: false,
            session_id: None,
            groups: Groups::new(),
            role,
            public,
            private,
        });
    }

    pub fn session_id(&self) -> Option<i64> {
        return self.session_id;
    }
}

impl<Pu: PublicUserMeta, Pr: PrivateUserMeta> User<Pu, Pr> {
    pub fn remove_group(&mut self, group: Group) {
        let idx = self.groups.position(group);

        match idx {
            Some(idx) => {
                self.groups.remove(idx);
            }
            None => {}
        }
    }

    pub fn add_group(&mut self, group: Group) {
        if self.groups.contains(group.clone()) {
        } else {
            self.groups.push(group)
        }
    }
}

#[derive(Clone, Debug)]
pub struct Role {
    pub name: String,
}

impl Role {
    pub fn from_string(name: String) -> Self {
        return Self { name };
    }

    pub fn from_str(name: &str) -> Self {
        return Self {
            name: name.to_owned(),
        };
    }

    pub fn as_str(&self) -> &str {
        return self.name.as_str();
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Group {
    name: String,
}

impl Group {
    pub fn from_string(name: String) -> Self {
        return Self { name };
    }

    pub fn from_str(name: &str) -> Self {
        return Self {
            name: name.to_owned(),
        };
    }

    pub fn as_str(&self) -> &str {
        return self.name.as_str();
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Groups {
    groups: Vec<Group>,
    pub string: String,
}

impl Groups {
    pub fn new() -> Self {
        return Self {
            groups: Vec::new(),
            string: String::new(),
        };
    }
}

impl Groups {
    pub fn contains(&self, group: Group) -> bool {
        for c_group in &self.groups {
            if *c_group == group {
                return true;
            }
        }
        return false;
    }

    pub fn remove(&mut self, idx: usize) {
        let mut str_groups: Vec<&str> = self.string.split(',').collect();
        self.groups.remove(idx);
        str_groups.remove(idx);
        self.string = str_groups.join(",");
    }

    pub fn push(&mut self, group: Group) {
        self.groups.push(group.clone());
        self.string.push_str(group.as_str())
    }

    pub fn position(&self, group: Group) -> Option<usize> {
        return self.groups.iter().position(|x| *x == group);
    }
}

pub trait PublicUserMeta {}

pub trait PrivateUserMeta {}

#[derive(Debug)]
pub enum UserManagerErrorKind {
    Harness(Box<dyn error::Error>),
    Token(AuthTokenError),
    UserNotFound,
}

#[derive(Debug)]
pub struct UserManagerError {
    kind: UserManagerErrorKind,
}

impl Display for UserManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self);
    }
}

impl From<Box<dyn error::Error>> for UserManagerError {
    fn from(value: Box<dyn error::Error>) -> Self {
        return UserManagerError::new(UserManagerErrorKind::Harness(value));
    }
}

impl From<AuthTokenError> for UserManagerError {
    fn from(value: AuthTokenError) -> Self {
        return UserManagerError::new(UserManagerErrorKind::Token(value));
    }
}

impl UserManagerError {
    pub fn new(kind: UserManagerErrorKind) -> Self {
        return Self { kind };
    }
}

impl error::Error for UserManagerError {}
