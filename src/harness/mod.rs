pub mod sqlite;
// mod postgresql;
// mod mysql;

use std::error;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use sqlite::{SqliteDiskOpSession, SqliteDiskOpToken, SqliteDiskOpUser};

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
    fn read<T: IntoValues>(&self, id: i64) -> Result<(), Box<dyn error::Error>>;
    fn update<T: IntoValues>(
        &self,
        item: &T,
        cols: &Vec<String>,
    ) -> Result<(), Box<dyn error::Error>>;
    fn insert<T: IntoValues>(
        &self,
        item: &T,
        cols: &Vec<String>,
    ) -> Result<(), Box<dyn error::Error>>;
    fn delete<T: IntoValues>(&self, id: i64) -> Result<(), Box<dyn error::Error>>;
    fn create_table(&self, sql_string: Option<String>) -> Result<(), Box<dyn error::Error>>;
}
/// IntoValues should turn the values into a string form to be inserted into a SQL statement
pub trait IntoValues {
    fn into_values(&self) -> Vec<String>;
}

// TODO: This should be for extinsibility of the tables in the SQL database... ?
// pub trait IntoCols {
//     fn into_cols(&self) -> Vec<String>;
// }

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

pub struct DiskOpManager<T, U, V>
where
    T: DiskOp,
    U: DiskOp,
    V: DiskOp,
{
    pub user: T,
    pub session: U,
    pub token: V,
}

impl<T, U, V> DiskOpManager<T, U, V>
where
    T: DiskOp,
    U: DiskOp,
    V: DiskOp,
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

impl DiskOpManager<SqliteDiskOpUser, SqliteDiskOpSession, SqliteDiskOpToken> {
    pub fn new_sqlite(
        pool: Pool<SqliteConnectionManager>,
    ) -> DiskOpManager<SqliteDiskOpUser, SqliteDiskOpSession, SqliteDiskOpToken> {
        return DiskOpManager {
            user: SqliteDiskOpUser::new(pool.clone()),
            session: SqliteDiskOpSession::new(pool.clone()),
            token: SqliteDiskOpToken::new(pool),
        };
    }
}
