#![allow(dead_code)]
use std::{collections::HashMap, rc::Rc};

use crate::core::{
    database::value_ref_to_type,
    error::{custom_error, Error, Result},
    repository::Model,
    selector::{DbTranslateBox, Selector},
};
use chrono::Utc;
use duckdb::{params_from_iter, types::ValueRef, Connection, ToSql};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WarehouseColor {
    pub key: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Warehouse {
    pub id: i64,
    pub name: String,
    pub shorthand: String,
    pub color: Option<WarehouseColor>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
impl Warehouse {
    pub fn new(name: String, shorthand: String, color: Option<WarehouseColor>) -> Self {
        Warehouse {
            id: 0,
            name,
            shorthand,
            color,
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        }
    }
    pub fn select() -> Selector {
        let mut selector = Selector::default();
        selector.select(Self::get_columns().to_vec(), Self::TABLE_NAME);
        selector
    }
    pub fn get_columns() -> [String; 7] {
        [
            "id".to_string(),
            "name".to_string(),
            "shorthand".to_string(),
            "color_code".to_string(),
            "color_key".to_string(),
            "created_at".to_string(),
            "updated_at".to_string(),
        ]
    }

    pub fn from_row(row: &duckdb::Row, translator: &DbTranslateBox) -> Result<Self> {
        let mut color = None;
        let color_code: Result<Option<String>> = row
            .get(translator.field("color_code")?)
            .map_err(Error::from);
        let color_key: Result<Option<String>> =
            row.get(translator.field("color_key")?).map_err(Error::from);
        if let Ok(Some(color_code)) = color_code {
            if let Ok(Some(color_key)) = color_key {
                color = Some(WarehouseColor {
                    key: color_key,
                    code: color_code,
                });
            }
        }
        Ok(Warehouse {
            id: row.get(translator.field("id")?)?,
            name: row.get(translator.field("name")?)?,
            shorthand: row.get(translator.field("shorthand")?)?,
            color,
            created_at: row.get(translator.field("created_at")?)?,
            updated_at: row.get(translator.field("updated_at")?)?,
        })
    }
    pub fn from_map(m: HashMap<String, ValueRef>) -> Result<Self> {
        let mut color = None;

        // if Self::get_columns().iter().any(|&e| !m.contains_key(e)) {
        //     return Err(custom_error(
        //         "Failed to construct MeasurementUnit from HasMap! Some keys missing!",
        //     ));
        // }
        let id: i64 = value_ref_to_type(m.get("id").unwrap()).map_err(Error::from)?;

        let color_code: Result<String> = value_ref_to_type(m.get("color_code").unwrap());
        let color_key: Result<String> = value_ref_to_type(m.get("color_key").unwrap());
        if let Ok(color_code) = color_code {
            if let Ok(color_key) = color_key {
                color = Some(WarehouseColor {
                    key: color_key,
                    code: color_code,
                });
            }
        }
        let name: String = value_ref_to_type(m.get("name").unwrap()).map_err(Error::from)?;
        let shorthand: String =
            value_ref_to_type(m.get("shorthand").unwrap()).map_err(Error::from)?;
        let created_at: chrono::NaiveDateTime =
            value_ref_to_type(m.get("created_at").unwrap()).map_err(Error::from)?;
        let updated_at: chrono::NaiveDateTime =
            value_ref_to_type(m.get("updated_at").unwrap()).map_err(Error::from)?;

        Ok(Warehouse {
            id,
            name,
            shorthand,
            color,
            created_at: Some(created_at),
            updated_at: Some(updated_at),
        })
    }
    pub fn save<'a>(&mut self, connection: &'a Connection) -> Result<()> {
        let mut params: Vec<Rc<dyn ToSql>> = Vec::new();
        let query;
        let now = Utc::now();

        if self.id > 0 {
            self.updated_at = Some(now.naive_utc());
            query = format!(
                "UPDATE {} SET name = ? , shorthand = ? , color_code = ? , color_key = ? , updated_at = ? WHERE id = ?",
                Self::TABLE_NAME
            );
            params.push(Rc::new(self.name.clone()));
            params.push(Rc::new(self.shorthand.clone()));
            if let Some(color) = &self.color {
                params.push(Rc::new(color.code.clone()));
                params.push(Rc::new(color.key.clone()));
            } else {
                params.push(Rc::new(None::<String>));
                params.push(Rc::new(None::<String>));
            }
            params.push(Rc::new(self.updated_at.clone()));
            params.push(Rc::new(self.id.clone()));
        } else {
            self.updated_at = Some(now.naive_utc());
            self.created_at = Some(now.naive_utc());
            query = format!(
                "INSERT INTO {} (name,shorthand,color_code,color_key,updated_at,created_at) VALUES (?,?,?,?,?,?) RETURNING id",
                Self::TABLE_NAME
            );
            params.push(Rc::new(self.name.clone()));
            params.push(Rc::new(self.shorthand.clone()));
            if let Some(color) = &self.color {
                params.push(Rc::new(color.code.clone()));
                params.push(Rc::new(color.key.clone()));
            } else {
                params.push(Rc::new(None::<String>));
                params.push(Rc::new(None::<String>));
            }
            params.push(Rc::new(self.updated_at.clone()));
            params.push(Rc::new(self.created_at.clone()));
        }
        if self.id > 0 {
            // Update in the database
            let mut stmt = connection.prepare(&query)?;
            let changes = stmt.execute(params_from_iter(params.into_iter()))?;
            if changes <= 0 {
                todo!("Add warning?")
            }
            return Ok(());
        } else {
            let mut stmt = connection.prepare(&query)?;
            stmt.execute(params_from_iter(params.into_iter()))?;
            return match stmt.raw_query().next()? {
                Some(row) => {
                    let id = row.get(stmt.column_index("id")?)?;
                    self.id = id;
                    return Ok(());
                }
                None => Err(custom_error("failed to insert at 'warehouse'!")),
            };
        }
    }
}

impl Model for Warehouse {
    const TABLE_NAME: &str = "warehouse";
    fn get_id(&self) -> String {
        format!("{}", self.id)
    }
}
