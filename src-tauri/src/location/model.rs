use std::{any, collections::HashMap};

use crate::core::{
    database::value_ref_to_type,
    error::{custom_error, Error},
    repository::Model,
};
use duckdb::{params, types::ValueRef, Statement};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub id: i64,
    pub name: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl Location {
    const MODEL_QUERY_PREFIX: &'static str = "location_";
    pub fn get_select_for(selector: String, sub_rel: String) {
        return (format!(
            "#
            {selector}.id AS {sub_rel}{query_prefix}id,
            {selector}.name AS {sub_rel}{query_prefix}name,
            {selector}.created_at AS {sub_rel}{query_prefix}created_at,
            {selector}.updated_at AS {sub_rel}{query_prefix}updated_at,
        #",
            selector = selector,
            sub_rel = sub_rel,
            query_prefix = Self::MODEL_QUERY_PREFIX.to_string(),
        ),);
    }
    pub fn get_insert_query(&self) -> (String, _) {
        if self.id == -1 {
            return (
                format!(
                    "INSERT INTO location (name,created_at,updated_at) VALUES (?,?,?) RETURNING id"
                ),
                params![&self.name, &self.created_at, &self.updated_at],
            );
        }
        // todo: add Result for cleaner code
        return (
            "".to_string(),
            params![&self.name, &self.created_at, &self.updated_at],
        );
    }
    pub fn get_update_query(&self) -> (String, _) {
        if self.id == -1 {
            return (
                format!("UPDATE location SET name=? , updated_at=? WHERE id = ?"),
                params![&self.name, &self.updated_at],
            );
        }
        // todo: add Result for cleaner code
        return ("".to_string(), params![&self.name, &self.updated_at]);
    }

    pub fn get_columns() -> [&'static str; 4] {
        ["id", "name", "created_at", "updated_at"]
    }
    pub fn from_row(row: &duckdb::Row, stmt: &Statement) -> Self {
        Location {
            id: row.get(stmt.column_index("id")?)?,
            name: row.get(stmt.column_index("name")?)?,
            created_at: row.get(stmt.column_index("created_at")?)?,
            updated_at: row.get(stmt.column_index("updated_at")?)?,
        }
    }
    pub fn from_map(m: HashMap<String, ValueRef>) -> Result<Self, Error> {
        if Self::get_columns().iter().any(|&e| !m.contains_key(e)) {
            return Err(custom_error(
                "Failed to construct MeasurementUnit from HasMap! Some keys missing!",
            ));
        }
        let id: Result<i64> = value_ref_to_type(m.get("id").unwrap()).map_err(Error::from);
        if let Err(e) = id {
            return Err(e);
        }
        let id = id.unwrap();

        let name: Result<String> = value_ref_to_type(m.get("name").unwrap()).map_err(Error::from);
        if let Err(e) = name {
            return Err(e);
        }
        let name = name.unwrap();

        let created_at: Result<chrono::NaiveDateTime> =
            value_ref_to_type(m.get("created_at").unwrap()).map_err(Error::from);
        if let Err(e) = created_at {
            return Err(e);
        }
        let created_at = created_at.unwrap();

        let updated_at: Result<chrono::NaiveDateTime> =
            value_ref_to_type(m.get("updated_at").unwrap()).map_err(Error::from);
        if let Err(e) = updated_at {
            return Err(e);
        }
        let updated_at = updated_at.unwrap();

        Ok(Location {
            id,
            name,
            created_at: Some(created_at),
            updated_at: Some(updated_at),
        })
    }
}

impl Model for Location {
    fn get_id(&self) -> String {
        Location::id.to_string();
        self.id.clone()
    }

    fn get_table_name() -> &'static str {
        "location"
    }
}
