use std::{error, result};

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Error, Result};

use crate::harness::{repeat_fields, repeat_vars};

use super::{DiskOp, IntoValues};

pub struct SqliteDiskOpUser {
    connection: Pool<SqliteConnectionManager>,
}

// impl DiskOpUser for SqliteDiskOpUser {
//     fn ban(&self) -> Result<(), Box<dyn error::Error>> {
//         unimplemented!()
//     }
//     fn update_public(&self) -> Result<(), Box<dyn error::Error>> {
//         unimplemented!()
//     }
//     fn insert_group(&self) -> Result<(), Box<dyn error::Error>> {
//         unimplemented!()
//     }
//     fn remove_group(&self) -> Result<(), Box<dyn error::Error>> {
//         unimplemented!()
//     }
//     fn update_private(&self) -> Result<(), Box<dyn error::Error>> {
//         unimplemented!()
//     }
//     fn write_role(&self) -> Result<(), Box<dyn error::Error>> {
//         unimplemented!()
//     }
// }

impl SqliteDiskOpUser {
    pub fn new(connection: Pool<SqliteConnectionManager>) -> Self {
        Self { connection }
    }
}

impl DiskOp for SqliteDiskOpUser {
    fn delete<User>(&self, id: i64) -> Result<(), Box<dyn error::Error>> {
        self.connection
            .get()?
            .prepare("DELETE FROM users WHERE id = ?")?
            .execute([id])?;
        return Ok(());
    }
    fn insert<User: IntoValues>(
        &self,
        user: &User,
        cols: &Vec<String>,
    ) -> Result<(), Box<dyn error::Error>> {
        let fields = repeat_fields(cols.to_vec());
        let vars = repeat_vars(cols.len());

        self.connection
            .get()?
            .prepare(format!("INSERT INTO users ({}) VALUES ({})", fields, vars).as_str())?
            .execute(rusqlite::params_from_iter(user.into_values()))?;

        Ok(())
    }
    fn read<User>(&self, id: i64) -> Result<(), Box<dyn error::Error>> {
        self.connection
            .get()?
            .prepare("SELECT * FROM users WHERE id = ?")?
            .execute([id])?;
        return Ok(());
    }
    fn update<User>(&self, user: &User, cols: &Vec<String>) -> Result<(), Box<dyn error::Error>> {
        unimplemented!()
    }

    fn create_table(
        &self,
        sql_string: Option<String>,
    ) -> result::Result<(), Box<dyn error::Error>> {
        let default_vals = format!(
            "CREATE TABLE IF NOT EXISTS \"users\" (
            id STRING,
            username STRING,
            secret STRING,
            salt STRING,
            session_id BIGINT,
            role STRING,
            groups STRING,
            ban TINYINT,
            FOREIGN KEY(session_id) REFERENCES session(id)
            "
        );

        match sql_string {
            Some(string) => {
                self.connection
                    .get()?
                    .prepare(format!("{},{})", default_vals, string).as_str())?
                    .execute([])?;
            }
            None => {
                self.connection
                    .get()?
                    .prepare(format!("{})", default_vals).as_str())?
                    .execute([])?;
            }
        };

        return Ok(());
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
    fn delete<Session>(&self, id: i64) -> Result<(), Box<dyn error::Error>> {
        self.connection
            .get()?
            .prepare("DELETE FROM sessions WHERE id = ?")?
            .execute([id])?;
        return Ok(());
    }
    fn insert<Session: IntoValues>(
        &self,
        session: &Session,
        cols: &Vec<String>,
    ) -> Result<(), Box<dyn error::Error>> {
        let fields = repeat_fields(cols.to_vec());
        let vars = repeat_vars(cols.len());

        self.connection
            .get()?
            .prepare(format!("INSERT INTO sessions ({}) VALUES ({})", fields, vars).as_str())?
            .execute(rusqlite::params_from_iter(session.into_values()))?;

        Ok(())
    }
    fn read<Session>(&self, id: i64) -> Result<(), Box<dyn error::Error>> {
        self.connection
            .get()?
            .prepare("SELECT * FROM sessions WHERE id = ?")?
            .execute([id])?;
        return Ok(());
    }
    fn update<Session>(
        &self,
        session: &Session,
        cols: &Vec<String>,
    ) -> Result<(), Box<dyn error::Error>> {
        unimplemented!()
    }

    fn create_table(&self, sql_string: Option<String>) -> Result<(), Box<dyn error::Error>> {
        let default_vals = format!(
            "CREATE TABLE IF NOT EXISTS \"sessions\" (
            id BIGINT PRIMARY KEY,
            user_id BIGINT,
            refresh_token BIGINT,
            auth_token BIGINT,
            expires DATETIME,
            FOREIGN KEY(user_id) REFERENCES user(id),
            FOREIGN KEY(refresh_token) REFERENCES token(id)
            "
        );

        match sql_string {
            Some(string) => {
                self.connection
                    .get()?
                    .prepare(format!("{},{})", default_vals, string).as_str())?
                    .execute([])?;
            }
            None => {
                self.connection
                    .get()?
                    .prepare(format!("{})", default_vals).as_str())?
                    .execute([])?;
            }
        };

        return Ok(());
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
    fn delete<Token: IntoValues>(&self, id: i64) -> Result<(), Box<dyn error::Error>> {
        self.connection
            .get()?
            .prepare("DELETE FROM tokens WHERE id = ?")?
            .execute([id])?;
        return Ok(());
    }

    fn insert<Token: IntoValues>(
        &self,
        token: &Token,
        cols: &Vec<String>,
    ) -> Result<(), Box<dyn error::Error>> {
        let fields = repeat_fields(cols.to_vec());
        let vars = repeat_vars(cols.len());

        self.connection
            .get()?
            .prepare(format!("INSERT INTO tokens ({}) VALUES ({})", fields, vars).as_str())?
            .execute(rusqlite::params_from_iter(token.into_values()))?;

        Ok(())
    }
    fn read<Token: IntoValues>(&self, id: i64) -> Result<(), Box<dyn error::Error>> {
        self.connection
            .get()?
            .prepare("SELECT * FROM tokens WHERE id = ?")?
            .execute([id])?;

        Ok(())
    }
    fn update<Token: IntoValues>(
        &self,
        token: &Token,
        cols: &Vec<String>,
    ) -> Result<(), Box<dyn error::Error>> {
        let values = token.into_values();
        // TODO: come up with a better error...
        if values.len() != cols.len() {
            return Err(Box::new(Error::InvalidColumnIndex(cols.len() - 1)));
        }

        unimplemented!()
    }

    fn create_table(&self, sql_string: Option<String>) -> Result<(), Box<dyn error::Error>> {
        let default_vals = format!(
            "CREATE TABLE IF NOT EXISTS \"tokens\" (
            id BIGINT PRIMARY KEY,
            token STRING,
            expires DATETIME
            "
        );

        match sql_string {
            Some(string) => {
                self.connection
                    .get()?
                    .prepare(format!("{},{})", default_vals, string).as_str())?
                    .execute([])?;
            }
            None => {
                self.connection
                    .get()?
                    .prepare(format!("{})", default_vals).as_str())?
                    .execute([])?;
            }
        };

        return Ok(());
    }
}
