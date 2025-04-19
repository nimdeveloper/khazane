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

#[derive(Debug, Deserialize, Serialize)]
pub struct Person {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub national_code: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl Person {
    pub fn new(
        first_name: String,
        last_name: String,
        national_code: Option<String>,
        phone: Option<String>,
        email: Option<String>,
        address: Option<String>,
    ) -> Self {
        Person {
            id: 0,
            first_name,
            last_name,
            national_code,
            phone,
            email,
            address,
            updated_at: None,
            created_at: None,
        }
    }
    pub fn select() -> Selector {
        let mut selector = Selector::default();
        selector.select(Self::get_columns().to_vec(), Self::TABLE_NAME);
        selector
    }

    fn get_columns() -> [String; 9] {
        [
            "id".to_string(),
            "first_name".to_string(),
            "last_name".to_string(),
            "national_code".to_string(),
            "phone".to_string(),
            "email".to_string(),
            "address".to_string(),
            "created_at".to_string(),
            "updated_at".to_string(),
        ]
    }
    fn load_as_related() {}

    pub fn from_row(row: &duckdb::Row, translator: &DbTranslateBox) -> Result<Self> {
        Ok(Person {
            id: row.get(translator.field("id")?)?,
            first_name: row.get(translator.field("first_name")?)?,
            last_name: row.get(translator.field("last_name")?)?,
            national_code: row.get(translator.field("national_code")?)?,
            phone: row.get(translator.field("phone")?)?,
            email: row.get(translator.field("email")?)?,
            address: row.get(translator.field("address")?)?,
            created_at: row.get(translator.field("created_at")?)?,
            updated_at: row.get(translator.field("updated_at")?)?,
        })
    }
    pub fn from_map(m: HashMap<String, ValueRef>) -> Result<Self> {
        // if Self::get_columns().iter().any(|&e| !m.contains_key(e)) {
        //     return Err(custom_error(
        //         "Failed to construct Person from HasMap! Some keys missing!",
        //     ));
        // }
        let id: i64 = value_ref_to_type(m.get("id").unwrap()).map_err(Error::from)?;
        let first_name: String =
            value_ref_to_type(m.get("first_name").unwrap()).map_err(Error::from)?;
        let last_name: String =
            value_ref_to_type(m.get("last_name").unwrap()).map_err(Error::from)?;
        let national_code: Option<String> =
            value_ref_to_type(m.get("national_code").unwrap()).map_err(Error::from)?;
        let phone: Option<String> =
            value_ref_to_type(m.get("phone").unwrap()).map_err(Error::from)?;
        let email: Option<String> =
            value_ref_to_type(m.get("email").unwrap()).map_err(Error::from)?;
        let address: Option<String> =
            value_ref_to_type(m.get("address").unwrap()).map_err(Error::from)?;
        let created_at: chrono::NaiveDateTime =
            value_ref_to_type(m.get("created_at").unwrap()).map_err(Error::from)?;
        let updated_at: chrono::NaiveDateTime =
            value_ref_to_type(m.get("updated_at").unwrap()).map_err(Error::from)?;
        Ok(Person {
            id,
            first_name,
            last_name,
            national_code,
            address,
            phone,
            email,
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
            query = format!("UPDATE {} SET first_name = ? , last_name = ? , national_code = ? , phone = ? , email = ? , address = ? , updated_at = ? WHERE id = ?", Self::TABLE_NAME);
            params.push(Rc::new(self.first_name.clone()));
            params.push(Rc::new(self.last_name.clone()));
            params.push(Rc::new(self.national_code.clone()));
            params.push(Rc::new(self.phone.clone()));
            params.push(Rc::new(self.email.clone()));
            params.push(Rc::new(self.address.clone()));
            params.push(Rc::new(self.updated_at.clone()));
            params.push(Rc::new(self.id.clone()));
        } else {
            self.created_at = Some(now.naive_utc());
            self.updated_at = Some(now.naive_utc());
            query = format!("INSERT INTO {} (first_name,last_name,national_code,phone,email,address,created_at,updated_at) VALUES (?,?,?,?,?,?,?,?) RETURNING id", Self::TABLE_NAME);
            params.push(Rc::new(self.first_name.clone()));
            params.push(Rc::new(self.last_name.clone()));
            params.push(Rc::new(self.national_code.clone()));
            params.push(Rc::new(self.phone.clone()));
            params.push(Rc::new(self.email.clone()));
            params.push(Rc::new(self.address.clone()));
            params.push(Rc::new(self.created_at.clone()));
            params.push(Rc::new(self.updated_at.clone()));
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
                None => Err(custom_error("failed to insert at 'person'!")),
            };
        }
    }
}
impl Model for Person {
    const TABLE_NAME: &str = "person";
    fn get_id(&self) -> String {
        format!("{}", self.id)
    }
}
