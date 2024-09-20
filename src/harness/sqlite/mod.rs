use std::{error, result};

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Error, Result};

use crate::{
    auth_token::AuthToken,
    harness::{repeat_fields, repeat_vars},
    session::Session,
    user::{Group, PrivateUserMeta, PublicUserMeta, Role, User},
};

use super::{DiskOpManager, DiskOpSession, DiskOpToken, DiskOpUser};

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

pub struct SqliteDiskOpUser {
    connection: Pool<SqliteConnectionManager>,
    cols: Vec<String>,
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
        Self {
            connection,
            cols: vec![
                "username".to_string(),
                "session_id".to_string(),
                "secret".to_string(),
                "salt".to_string(),
                "id".to_string(),
                "role".to_string(),
                "groups".to_string(),
                "ban".to_string(),
            ],
        }
    }

    pub fn add_cols(&mut self, cols: Vec<String>) {
        self.cols.extend(cols);
    }
}

impl DiskOpUser for SqliteDiskOpUser {
    fn delete(&self, id: i64) -> Result<(), Box<dyn error::Error>> {
        self.connection
            .get()?
            .prepare("DELETE FROM users WHERE id = ?")?
            .execute([id])?;
        return Ok(());
    }
    fn insert<R, G, Pu, Pr>(&self, user: &User<R, G, Pu, Pr>) -> Result<(), Box<dyn error::Error>>
    where
        R: Role,
        G: Group,
        Pu: PublicUserMeta,
        Pr: PrivateUserMeta,
    {
        let fields = repeat_fields(self.cols.to_vec());
        let vars = repeat_vars(self.cols.len());

        self.connection
            .get()?
            .prepare(format!("INSERT INTO users ({}) VALUES ({})", fields, vars).as_str())?
            .execute(rusqlite::params_from_iter(user.into_values()))?;

        Ok(())
    }
    fn read<R, G, Pu, Pr>(&self, id: i64) -> Result<User<R, G, Pu, Pr>, Box<dyn error::Error>>
    where
        R: Role,
        G: Group,
        Pu: PublicUserMeta,
        Pr: PrivateUserMeta,
    {
        // let conn = self.connection.get()?;
        // let mut statement = conn.prepare("SELECT * FROM users WHERE id = ?")?;
        // let mut out = statement.query([id])?;
        // let id = out.next()?;
        // let user_name = out.next()?;
        // let secret = out.next()?;
        // let salt = out.next()?;
        // let session_id = out.next()?;
        // let role = out.next()?;
        // let groups = out.next()?;
        // let ban = out.next()?;

        unimplemented!()
        // TODO: get other values from the db for custom public and private data.
    }
    fn update<R, G, Pu, Pr>(&self, user: &User<R, G, Pu, Pr>) -> Result<(), Box<dyn error::Error>>
    where
        R: Role,
        G: Group,
        Pu: PublicUserMeta,
        Pr: PrivateUserMeta,
    {
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

    fn ban(&self) -> result::Result<(), Box<dyn error::Error>> {
        unimplemented!()
    }

    fn insert_group(&self) -> result::Result<(), Box<dyn error::Error>> {
        unimplemented!()
    }

    fn remove_group(&self) -> result::Result<(), Box<dyn error::Error>> {
        unimplemented!()
    }

    fn update_private(&self) -> result::Result<(), Box<dyn error::Error>> {
        unimplemented!()
    }

    fn update_public(&self) -> result::Result<(), Box<dyn error::Error>> {
        unimplemented!()
    }

    fn write_role(&self) -> result::Result<(), Box<dyn error::Error>> {
        unimplemented!()
    }
}

pub struct SqliteDiskOpSession {
    connection: Pool<SqliteConnectionManager>,
    cols: Vec<String>,
}

impl SqliteDiskOpSession {
    pub fn new(connection: Pool<SqliteConnectionManager>) -> Self {
        Self {
            connection,
            cols: vec![
                "id".to_string(),
                "user_id".to_string(),
                "refresh_token".to_string(),
                "auth_token".to_string(),
                "expires".to_string(),
            ],
        }
    }

    pub fn add_cols(&mut self, cols: Vec<String>) {
        self.cols.extend(cols);
    }
}

impl DiskOpSession for SqliteDiskOpSession {
    fn delete(&self, id: i64) -> Result<(), Box<dyn error::Error>> {
        self.connection
            .get()?
            .prepare("DELETE FROM sessions WHERE id = ?")?
            .execute([id])?;
        return Ok(());
    }
    fn insert(&self, session: &Session) -> Result<(), Box<dyn error::Error>> {
        let fields = repeat_fields(self.cols.to_vec());
        let vars = repeat_vars(self.cols.len());

        self.connection
            .get()?
            .prepare(format!("INSERT INTO sessions ({}) VALUES ({})", fields, vars).as_str())?
            .execute(rusqlite::params_from_iter(session.into_values()))?;

        Ok(())
    }
    fn read(&self, id: i64) -> Result<Session, Box<dyn error::Error>> {
        self.connection
            .get()?
            .prepare("SELECT * FROM sessions WHERE id = ?")?
            .execute([id])?;
        unimplemented!()
    }
    fn update(&self, session: &Session) -> Result<(), Box<dyn error::Error>> {
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
    cols: Vec<String>,
}

impl SqliteDiskOpToken {
    pub fn new(connection: Pool<SqliteConnectionManager>) -> Self {
        Self {
            connection,
            cols: vec!["id".to_string(), "token".to_string(), "expires".to_string()],
        }
    }

    pub fn add_cols(&mut self, cols: Vec<String>) {
        self.cols.extend(cols);
    }
}

impl DiskOpToken for SqliteDiskOpToken {
    fn delete(&self, id: i64) -> Result<(), Box<dyn error::Error>> {
        self.connection
            .get()?
            .prepare("DELETE FROM tokens WHERE id = ?")?
            .execute([id])?;
        return Ok(());
    }

    fn insert(&self, token: &AuthToken) -> Result<(), Box<dyn error::Error>> {
        let fields = repeat_fields(self.cols.to_vec());
        let vars = repeat_vars(self.cols.len());

        self.connection
            .get()?
            .prepare(format!("INSERT INTO tokens ({}) VALUES ({})", fields, vars).as_str())?
            .execute(rusqlite::params_from_iter(token.into_values()))?;

        Ok(())
    }
    fn read(&self, id: i64) -> Result<AuthToken, Box<dyn error::Error>> {
        self.connection
            .get()?
            .prepare("SELECT * FROM tokens WHERE id = ?")?
            .execute([id])?;

        unimplemented!()
    }
    fn update(&self, token: &AuthToken) -> Result<(), Box<dyn error::Error>> {
        let values = token.into_values();
        // TODO: come up with a better error...
        if values.len() != self.cols.len() {
            return Err(Box::new(Error::InvalidColumnIndex(self.cols.len() - 1)));
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
