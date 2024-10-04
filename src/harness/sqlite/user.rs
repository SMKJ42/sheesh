use std::{error, result};

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::{
    harness::DbHarnessUser,
    user::{PrivateUserMeta, PublicUserMeta, User},
};

pub struct SqliteHarnessUser<'a> {
    connection: Pool<SqliteConnectionManager>,
    private_cols: Vec<&'a str>,
    public_cols: Vec<&'a str>,
}

impl<'a> SqliteHarnessUser<'a> {
    pub fn new(connection: Pool<SqliteConnectionManager>) -> Self {
        Self {
            connection,
            private_cols: Vec::new(),
            public_cols: Vec::new(),
        }
    }

    fn with_public_cols(mut self, cols: Vec<&'a str>) -> Self {
        self.public_cols = cols;
        return self;
    }

    fn with_private_cols(mut self, cols: Vec<&'a str>) -> Self {
        self.private_cols = cols;
        return self;
    }
}

impl<'a> DbHarnessUser for SqliteHarnessUser<'a> {
    fn delete(&self, id: i64) -> Result<(), Box<dyn error::Error>> {
        self.connection
            .get()?
            .prepare("DELETE FROM users WHERE id = ?")?
            .execute([id])?;
        return Ok(());
    }
    fn insert<Pu, Pr>(&self, user: &User<Pu, Pr>) -> Result<(), Box<dyn error::Error>>
    where
        Pu: PublicUserMeta,
        Pr: PrivateUserMeta,
    {
        // let fields = repeat_fields(self.params());
        // let vars = repeat_vars(self.params().len());

        // self.connection
        //     .get()?
        //     .prepare(format!("INSERT INTO users ({}) VALUES ({})", fields, vars).as_str())?
        //     .execute(user.into_sqlite_params().as_slice())?;

        todo!();
    }
    fn read<Pu, Pr>(&self, id: i64) -> Result<Option<User<Pu, Pr>>, Box<dyn error::Error>>
    where
        Pu: PublicUserMeta,
        Pr: PrivateUserMeta,
    {
        let conn = self.connection.get()?;
        let mut statement = conn.prepare("SELECT * FROM users WHERE id = ?")?;
        let mut result = statement.query([id])?;
        todo!();
        // match result.next()? {
        //     Some(row) => {
        //         return User::<Pu, Pr>::from_sqlite_row(row);
        //     }
        //     None => return Ok(None),
        // }
    }
    fn update<Pu, Pr>(&self, user: &User<Pu, Pr>) -> Result<usize, Box<dyn error::Error>>
    where
        Pu: PublicUserMeta,
        Pr: PrivateUserMeta,
    {
        let conn = self.connection.get()?;
        let mut statement = conn.prepare("UPDATE users SET XXXXXXXX WHERE id = ?")?;
        todo!();

        // let res = statement.execute(user.into_sqlite_params().as_slice());

        // match res {
        // Ok(res) => Ok(res),
        // Err(err) => Err(err.into()),
        // }
    }

    fn create_table(
        &self,
        sql_string: Option<String>,
    ) -> result::Result<(), Box<dyn error::Error>> {
        let default_stmt = format!(
            "CREATE TABLE IF NOT EXISTS \"users\" (
                id INTEGER PRIMARY KEY,
                session_id INTEGER,
                username STRING NOT NULL UNIQUE,
                secret STRING NOT NULL,
                ban TINYINT NOT NULL,
                groups STRING NOT NULL,
                role STRING NOT NULL,
                FOREIGN KEY(session_id) REFERENCES sessions(id)
            "
        );

        match sql_string {
            Some(stmt_extension) => {
                self.connection
                    .get()?
                    .prepare(format!("{},{});", default_stmt, stmt_extension).as_str())?
                    .execute([])?;
            }
            None => {
                self.connection
                    .get()?
                    .prepare(format!("{});", default_stmt).as_str())?
                    .execute([])?;
            }
        };

        return Ok(());
    }

    fn ban(&self) -> result::Result<(), Box<dyn error::Error>> {
        todo!()
    }

    fn insert_group(&self) -> result::Result<(), Box<dyn error::Error>> {
        todo!()
    }

    fn remove_group(&self) -> result::Result<(), Box<dyn error::Error>> {
        todo!()
    }

    fn update_private(&self) -> result::Result<(), Box<dyn error::Error>> {
        todo!()
    }

    fn update_public(&self) -> result::Result<(), Box<dyn error::Error>> {
        todo!()
    }

    fn write_role(&self) -> result::Result<(), Box<dyn error::Error>> {
        todo!()
    }
}
