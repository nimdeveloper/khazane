use crate::core::{
    error::{custom_error, Result},
    repository::Model,
    selector::{DbTranslateBox, Selector},
};
use chrono::Utc;
use duckdb::{params_from_iter, Connection, ToSql};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, rc::Rc};

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub id: i64,
    pub name: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl Location {
    pub fn new(name: String) -> Self {
        Location {
            id: 0,
            name,
            updated_at: None,
            created_at: None,
        }
    }
    pub fn select() -> Selector {
        let mut selector = Selector::default();
        selector.select(Self::get_columns().to_vec(), Self::TABLE_NAME);
        selector
    }

    pub fn get_columns() -> [String; 4] {
        [
            "id".to_string(),
            "name".to_string(),
            "created_at".to_string(),
            "updated_at".to_string(),
        ]
    }
    pub fn from_row(row: &duckdb::Row, translator: &DbTranslateBox) -> Result<Self> {
        Ok(Location {
            id: row.get(translator.field("id")?)?,
            name: row.get(translator.field("name")?)?,
            created_at: row.get(translator.field("created_at")?)?,
            updated_at: row.get(translator.field("updated_at")?)?,
        })
    }
    // pub fn from_map(m: HashMap<String, ValueRef>) -> Result<Self> {
    //     if Self::get_columns().iter().any(|e| !m.contains_key(e)) {
    //         return Err(custom_error(
    //             "Failed to construct MeasurementUnit from HasMap! Some keys missing!",
    //         ));
    //     }
    //     let id: i64 = value_ref_to_type(m.get("id").unwrap()).map_err(Error::from)?;

    //     let name: String = value_ref_to_type(m.get("name").unwrap()).map_err(Error::from)?;

    //     let created_at: chrono::NaiveDateTime =
    //         value_ref_to_type(m.get("created_at").unwrap()).map_err(Error::from)?;

    //     let updated_at: chrono::NaiveDateTime =
    //         value_ref_to_type(m.get("updated_at").unwrap()).map_err(Error::from)?;

    //     Ok(Location {
    //         id,
    //         name,
    //         created_at: Some(created_at),
    //         updated_at: Some(updated_at),
    //     })
    // }
    pub fn save<'a>(&mut self, connection: &'a Connection) -> Result<()> {
        let mut params: Vec<Rc<dyn ToSql>> = Vec::new();
        let query;
        let now = Utc::now();

        if self.id > 0 {
            self.updated_at = Some(now.naive_utc());
            query = format!(
                "UPDATE {} SET name=? , updated_at=? WHERE id = ?",
                Self::TABLE_NAME
            );
            params.push(Rc::new(self.name.clone()));
            params.push(Rc::new(self.updated_at.clone()));
            params.push(Rc::new(self.id.clone()));
        } else {
            self.created_at = Some(now.naive_utc());
            self.updated_at = Some(now.naive_utc());
            params.push(Rc::new(self.name.clone()));
            params.push(Rc::new(self.created_at.clone()));
            params.push(Rc::new(self.updated_at.clone()));
            query = format!(
                "INSERT INTO {} (name,created_at,updated_at) VALUES (?,?,?) RETURNING id",
                Self::TABLE_NAME
            );
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
                None => Err(custom_error("failed to insert at 'location'!")),
            };
        }
    }
}

impl Model for Location {
    const TABLE_NAME: &str = "location";
    fn get_id(&self) -> String {
        self.id.to_string().clone()
    }
}
