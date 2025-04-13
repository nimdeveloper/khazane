use std::collections::HashMap;

use crate::core::{
    database::value_ref_to_type,
    error::{custom_error, Error},
    repository::Model,
};
use duckdb::{types::ValueRef, Statement};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WarehouseColor {
    pub key: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Warehouse {
    pub id: String,
    pub name: String,
    pub shorthand: String,
    pub color: Option<WarehouseColor>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
impl Warehouse {
    pub const MODEL_QUERY_PREFIX: &'static str = "warehouse_";
    pub fn get_columns() -> [&'static str; 4] {
        [
            "id",
            "name",
            "shorthand",
            "color_code",
            "color_key",
            "created_at",
            "updated_at",
        ]
    }

    pub fn from_row(row: &duckdb::Row, stmt: &Statement) -> Self {
        let mut color = None;
        let color_code = row
            .get::<_, String>(stmt.column_index("color_code")?)
            .map_err(Error::from);
        let color_key = row
            .get::<_, String>(stmt.column_index("color_key")?)
            .map_err(Error::from);
        if let Ok(color_code) = color_code {
            if let Ok(color_key) = color_key {
                color = Some(WarehouseColor {
                    key: color_key,
                    code: color_code,
                });
            }
        }
        Warehouse {
            id: row.get(stmt.column_index("id")?)?,
            name: row.get(stmt.column_index("name")?)?,
            shorthand: row.get(stmt.column_index("shorthand")?)?,
            color,
            created_at: row.get(stmt.column_index("created_at")?)?,
            updated_at: row.get(stmt.column_index("updated_at")?)?,
        }
    }
    pub fn from_map(m: HashMap<String, ValueRef>) -> Result<Self, Error> {
        let mut color = None;

        if Self::get_columns().iter().any(|&e| !m.contains_key(e)) {
            return Err(custom_error(
                "Failed to construct MeasurementUnit from HasMap! Some keys missing!",
            ));
        }
        let id: Result<String> = value_ref_to_type(m.get("id").unwrap()).map_err(Error::from);
        if let Err(e) = id {
            return Err(e);
        }
        let id = id.unwrap();

        let color_code: Result<String> =
            value_ref_to_type(m.get("color_code").unwrap()).map_err(Error::from);
        let color_key: Result<String> =
            value_ref_to_type(m.get("color_key").unwrap()).map_err(Error::from);
        if let Ok(color_code) = color_code {
            if let Ok(color_key) = color_key {
                color = Some(WarehouseColor {
                    key: color_key,
                    code: color_code,
                });
            }
        }

        let name: Result<String> = value_ref_to_type(m.get("name").unwrap()).map_err(Error::from);
        if let Err(e) = name {
            return Err(e);
        }
        let name = name.unwrap();

        let shorthand: Result<String> =
            value_ref_to_type(m.get("shorthand").unwrap()).map_err(Error::from);
        if let Err(e) = shorthand {
            return Err(e);
        }
        let shorthand = shorthand.unwrap();

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

        Ok(Warehouse {
            id,
            name,
            shorthand,
            color,
            created_at: Some(created_at),
            updated_at: Some(updated_at),
        })
    }
}

impl Model for Warehouse {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_table_name() -> &'static str {
        "warehouse"
    }
}
