pub mod mysql;
pub mod postgresql;
pub mod sqlite;

use std::error;

use crate::{
    auth_token::AuthToken,
    session::Session,
    user::{Group, PrivateUserMeta, PublicUserMeta, Role, User},
};

pub enum Db {
    MySql,
    Postgresql,
    Sqlite,
}

pub trait DiskOpUser {
    fn read<R, G, Pu, Pr>(&self, id: i64) -> Result<User<R, G, Pu, Pr>, Box<dyn error::Error>>
    where
        R: Role,
        G: Group,
        Pu: PublicUserMeta,
        Pr: PrivateUserMeta;

    fn update<R, G, Pu, Pr>(&self, item: &User<R, G, Pu, Pr>) -> Result<(), Box<dyn error::Error>>
    where
        R: Role,
        G: Group,
        Pu: PublicUserMeta,
        Pr: PrivateUserMeta;

    fn insert<R, G, Pu, Pr>(&self, item: &User<R, G, Pu, Pr>) -> Result<(), Box<dyn error::Error>>
    where
        R: Role,
        G: Group,
        Pu: PublicUserMeta,
        Pr: PrivateUserMeta;

    fn delete(&self, id: i64) -> Result<(), Box<dyn error::Error>>;
    fn create_table(&self, sql_string: Option<String>) -> Result<(), Box<dyn error::Error>>;
    fn write_role(&self) -> Result<(), Box<dyn error::Error>>;
    fn insert_group(&self) -> Result<(), Box<dyn error::Error>>;
    fn remove_group(&self) -> Result<(), Box<dyn error::Error>>;
    fn update_public(&self) -> Result<(), Box<dyn error::Error>>;
    fn update_private(&self) -> Result<(), Box<dyn error::Error>>;
    fn ban(&self) -> Result<(), Box<dyn error::Error>>;
}

pub trait DiskOpSession {
    fn read(&self, id: i64) -> Result<Session, Box<dyn error::Error>>;
    fn update(&self, item: &Session) -> Result<(), Box<dyn error::Error>>;
    fn insert(&self, item: &Session) -> Result<(), Box<dyn error::Error>>;
    fn delete(&self, id: i64) -> Result<(), Box<dyn error::Error>>;
    fn create_table(&self, sql_string: Option<String>) -> Result<(), Box<dyn error::Error>>;
}

pub trait DiskOpToken {
    fn read(&self, id: i64) -> Result<AuthToken, Box<dyn error::Error>>;
    fn update(&self, item: &AuthToken) -> Result<(), Box<dyn error::Error>>;
    fn insert(&self, item: &AuthToken) -> Result<(), Box<dyn error::Error>>;
    fn delete(&self, id: i64) -> Result<(), Box<dyn error::Error>>;
    fn create_table(&self, sql_string: Option<String>) -> Result<(), Box<dyn error::Error>>;
}

pub fn repeat_vars(count: usize) -> String {
    assert_ne!(count, 0);
    let mut s = "?,".repeat(count);
    // Remove trailing comma
    s.pop();
    s
}

pub fn repeat_fields(cols: Vec<String>) -> String {
    let mut fields = String::new();
    for i in 0..cols.len() - 1 {
        fields.extend([&cols[i], ", "]);
    }
    fields += &cols[cols.len() - 1];

    return fields;
}

pub struct DiskOpManager<T, U, V>
where
    T: DiskOpUser,
    U: DiskOpSession,
    V: DiskOpToken,
{
    pub user: T,
    pub session: U,
    pub token: V,
}

impl<T, U, V> DiskOpManager<T, U, V>
where
    T: DiskOpUser,
    U: DiskOpSession,
    V: DiskOpToken,
{
    pub fn new_custom(user: T, session: U, token: V) -> Self {
        return Self {
            user,
            session,
            token,
        };
    }

    pub fn init_tables(&self) -> Result<(), Box<dyn error::Error>> {
        self.token.create_table(None)?;
        self.session.create_table(None)?;
        self.user.create_table(None)?;

        return Ok(());
    }
}
