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
    fn read<T: IntoRow>(&self, item: &T, cols: &Vec<String>) -> Result<(), Box<dyn error::Error>>;
    fn update<T: IntoRow>(&self, item: &T, cols: &Vec<String>)
        -> Result<(), Box<dyn error::Error>>;
    fn insert<T: IntoRow>(&self, item: &T, cols: &Vec<String>)
        -> Result<(), Box<dyn error::Error>>;
    fn delete<T: IntoRow>(&self, item: &T, cols: &Vec<String>)
        -> Result<(), Box<dyn error::Error>>;
    fn create_table(&self, sql_string: Option<String>) -> Result<(), Box<dyn error::Error>>;
}

pub trait IntoRow {
    fn into_row(&self) -> Vec<String>;
}
pub trait IntoCols {}

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
