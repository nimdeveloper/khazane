use crate::core::{
    database::value_ref_to_type,
    error::{custom_error, Error, Result},
    repository::{DuckDbRepository, Model},
    selector::Selector,
};
use chrono::Utc;
use duckdb::{params, types::ValueRef, Statement, ToSql};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub id: i64,
    pub name: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl Location {
    pub fn select() -> Selector {
        let mut selector = Selector::default();
        selector.select(Self::get_columns().to_vec(), Self::TABLE_NAME);
        selector
    }
    // pub fn get_insert_query<'a>(&'a self) -> (String, &'a [&dyn ToSql]) {
    //     if self.id == -1 {
    //         let params: &'a Vec<dyn ToSql> = [
    //             &self.name.into(),
    //             &self.created_at.into(),
    //             &self.updated_at.into(),
    //         ]
    //         .into();
    //         return (
    //             format!(
    //                 "INSERT INTO location (name,created_at,updated_at) VALUES (?,?,?) RETURNING id"
    //             ),
    //             &params,
    //         );
    //     }
    //     // todo: add Result for cleaner code
    //     return ("".to_string(), params![]);
    // }
    // pub fn get_update_query(&self) -> (String, &[&dyn ToSql]) {
    //     if self.id == -1 {
    //         return (
    //             format!("UPDATE location SET name=? , updated_at=? WHERE id = ?"),
    //             params![&self.name, &self.updated_at],
    //         );
    //     }
    //     // todo: add Result for cleaner code
    //     return ("".to_string(), params![&self.name, &self.updated_at]);
    // }

    pub fn get_columns() -> [String; 4] {
        [
            "id".to_string(),
            "name".to_string(),
            "created_at".to_string(),
            "updated_at".to_string(),
        ]
    }
    pub fn from_row(row: &duckdb::Row, stmt: &Statement) -> Result<Self> {
        Ok(Location {
            id: row.get(stmt.column_index("id")?)?,
            name: row.get(stmt.column_index("name")?)?,
            created_at: row.get(stmt.column_index("created_at")?)?,
            updated_at: row.get(stmt.column_index("updated_at")?)?,
        })
    }
    pub fn from_map(m: HashMap<String, ValueRef>) -> Result<Self> {
        if Self::get_columns().iter().any(|e| !m.contains_key(e)) {
            return Err(custom_error(
                "Failed to construct MeasurementUnit from HasMap! Some keys missing!",
            ));
        }
        let id: i64 = value_ref_to_type(m.get("id").unwrap()).map_err(Error::from)?;

        let name: String = value_ref_to_type(m.get("name").unwrap()).map_err(Error::from)?;

        let created_at: chrono::NaiveDateTime =
            value_ref_to_type(m.get("created_at").unwrap()).map_err(Error::from)?;

        let updated_at: chrono::NaiveDateTime =
            value_ref_to_type(m.get("updated_at").unwrap()).map_err(Error::from)?;

        Ok(Location {
            id,
            name,
            created_at: Some(created_at),
            updated_at: Some(updated_at),
        })
    }
}

impl Model for Location {
    const TABLE_NAME: &str = "location";
    fn get_id(&self) -> String {
        self.id.to_string().clone()
    }
}
