use rusqlite::{
    types::{FromSql, FromSqlResult, ToSqlOutput, Type},
    ToSql,
};

use crate::user::{Group, Groups, Role};

impl ToSql for Role {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        return Ok(ToSqlOutput::Owned(rusqlite::types::Value::Text(
            self.name.clone(),
        )));
    }
}

impl FromSql for Role {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value.data_type() {
            Type::Text => FromSqlResult::Ok(Self::from_str(value.as_str()?)),
            _ => {
                panic!()
            }
        }
    }
}

impl FromSql for Group {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> FromSqlResult<Self> {
        todo!()
    }
}

impl ToSql for Group {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        todo!();
    }
}

impl FromSql for Groups {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> FromSqlResult<Self> {
        todo!()
    }
}

impl ToSql for Groups {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        todo!();
    }
}
