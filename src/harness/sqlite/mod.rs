use std::fmt::Debug;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Result;

use crate::harness::{repeat_fields, repeat_vars};

use super::{DiskOp, IntoRow};

pub struct SqliteDiskOpUser {
    connection: Pool<SqliteConnectionManager>,
}

// impl DiskOpUser for SqliteDiskOpUser {
//     fn ban(&self) -> Result<(), Box<dyn std::error::Error>> {
//         unimplemented!()
//     }
//     fn update_public(&self) -> Result<(), Box<dyn std::error::Error>> {
//         unimplemented!()
//     }
//     fn insert_group(&self) -> Result<(), Box<dyn std::error::Error>> {
//         unimplemented!()
//     }
//     fn remove_group(&self) -> Result<(), Box<dyn std::error::Error>> {
//         unimplemented!()
//     }
//     fn update_private(&self) -> Result<(), Box<dyn std::error::Error>> {
//         unimplemented!()
//     }
//     fn write_role(&self) -> Result<(), Box<dyn std::error::Error>> {
//         unimplemented!()
//     }
// }

impl SqliteDiskOpUser {
    pub fn new(connection: Pool<SqliteConnectionManager>) -> Self {
        Self { connection }
    }
}

impl DiskOp for SqliteDiskOpUser {
    fn delete<User>(
        &self,
        user: User,
        cols: &Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
    fn insert<User>(
        &self,
        user: User,
        cols: &Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
    fn read<User>(&self, user: User, cols: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
    fn update<User>(
        &self,
        user: User,
        cols: &Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }

    fn create_table(
        &self,
        sql_string: Option<String>,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
}

pub struct SqliteDiskOpSession {
    connection: Pool<SqliteConnectionManager>,
}

impl SqliteDiskOpSession {
    pub fn new(connection: Pool<SqliteConnectionManager>) -> Self {
        Self { connection }
    }
}

impl DiskOp for SqliteDiskOpSession {
    fn delete<Session>(
        &self,
        session: Session,
        cols: &Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
    fn insert<Session>(
        &self,
        session: Session,
        cols: &Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
    fn read<Session>(
        &self,
        session: Session,
        cols: &Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
    fn update<Session>(
        &self,
        session: Session,
        cols: &Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }

    fn create_table(
        &self,
        sql_string: Option<String>,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
}

pub struct SqliteDiskOpToken {
    connection: Pool<SqliteConnectionManager>,
}

impl SqliteDiskOpToken {
    pub fn new(connection: Pool<SqliteConnectionManager>) -> Self {
        Self { connection }
    }
}

impl DiskOp for SqliteDiskOpToken {
    fn delete<Token: IntoRow + Debug>(
        &self,
        token: Token,
        cols: &Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }

    fn insert<Token: IntoRow + Debug>(
        &self,
        token: Token,
        cols: &Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let vars = repeat_vars(cols.len());
        let fields = repeat_fields(cols.to_vec());

        self.connection
            .get()?
            .prepare(format!("INSERT INTO tokens ({}) VALUES ({})", fields, vars).as_str())?
            .execute(rusqlite::params_from_iter(token.into_row()))?;

        Ok(())
    }
    fn read<Token: IntoRow + Debug>(
        &self,
        token: Token,
        cols: &Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
    fn update<Token: IntoRow + Debug>(
        &self,
        token: Token,
        cols: &Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }

    fn create_table(&self, sql_string: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: finish initializing the table, will panic as of now
        let default_vals = format!(
            "CREATE TABLE IF NOT EXISTS \"tokens\" (
            id INTEGER PRIMARY KEY

            token: String
            user_id: String
            expires: DateTime
            "
        );

        match sql_string {
            Some(string) => {
                self.connection
                    .get()?
                    .prepare(format!("{},{})", default_vals, string).as_str())?;
            }
            None => {
                self.connection
                    .get()?
                    .prepare(format!("{})", default_vals).as_str())?;
            }
        };

        return Ok(());
    }
}
