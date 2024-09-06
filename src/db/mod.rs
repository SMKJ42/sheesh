pub mod sqlite;
// mod postgresql;
// mod mysql;

use std::error;

pub enum Db {
    MySql,
    Postgresql,
    Sqlite,
}

pub struct Harness<T, U>
where
    T: DiskOp,
    U: DiskOp,
{
    pub user: T,
    pub session: U,
    pub token: U,
}

impl<T, U> Harness<T, U>
where
    T: DiskOp,
    U: DiskOp,
{
    pub fn sqlite() -> Self {
        unimplemented!()
    }
    // pub fn mysql() -> Self {
    //     unimplemented!()
    // }
    // pub fn postgresql() -> Self {
    // unimplemented!()
    // }
}

pub trait DiskOp {
    fn read(&self) -> Result<(), Box<dyn error::Error>>;
    fn update(&self) -> Result<(), Box<dyn error::Error>>;
    fn insert(&self) -> Result<(), Box<dyn error::Error>>;
    fn delete(&self) -> Result<(), Box<dyn error::Error>>;
}

pub trait DiskOpUser {
    fn write_role(&self) -> Result<(), Box<dyn error::Error>>;
    fn insert_group(&self) -> Result<(), Box<dyn error::Error>>;
    fn remove_group(&self) -> Result<(), Box<dyn error::Error>>;
    fn update_public(&self) -> Result<(), Box<dyn error::Error>>;
    fn update_private(&self) -> Result<(), Box<dyn error::Error>>;
    fn ban(&self) -> Result<(), Box<dyn error::Error>>;
}

pub trait DiskOpSession {
    // TODO:
}

pub trait DiskOpToken {
    // TODO:
}
