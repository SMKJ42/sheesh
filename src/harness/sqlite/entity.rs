use rusqlite::{
    types::{FromSql, FromSqlResult, ToSqlOutput, Type, Value},
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
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}

impl FromSql for Groups {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> FromSqlResult<Self> {
        let groups_str = value.as_str()?;
        let groups: Vec<&str> = groups_str.split(",").map(|x| x.trim()).collect();

        let mut out = Groups::new();

        for group in groups {
            out.add_group(Group::from_str(group));
        }

        return Ok(out);
    }
}

impl ToSql for Groups {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let string = self.to_string();
        return Ok(ToSqlOutput::Owned(Value::Text(string)));
    }
}
