use std::collections::HashMap;

use crate::core::{
    database::value_ref_to_type,
    error::{custom_error, Error},
    repository::Model,
};
use duckdb::{params, types::ValueRef, Statement};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Person {
    pub id: String,
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
    fn get_columns() {}
    fn load_as_related() {}

    pub fn get_columns() -> [&'static str; 8] {
        [
            "id",
            "first_name",
            "last_name",
            "national_code",
            "phone",
            "email",
            "address",
            "created_at",
            "updated_at",
        ]
    }

    pub fn get_insert_query(&self) -> (String, _) {
        if self.id == -1 {
            return (
                format!(
                    "INSERT INTO {} (first_name,last_name,national_code,phone,email,address,created_at,updated_at) VALUES (?,?,?,?,?,?,?,?) RETURNING id", Self::TABLE_NAME
                ),
                params![&self.first_name, &self.last_name, &self.national_code, &self.phone, &self.email, &self.address, &self.created_at, &self.updated_at],
            );
        }
        // todo: add Result for cleaner code
        return ("".to_string(), params![]);
    }
    pub fn get_update_query(&self) -> (String, _) {
        if self.id == -1 {
            return (
                format!("UPDATE {} SET first_name = ? , last_name = ? , national_code = ? , phone = ? , email = ? , address = ? , updated_at = ? WHERE id = ?", Self::TABLE_NAME),
                params![&self.first_name, &self.last_name, &self.national_code, &self.phone, &self.email, &self.address, &self.updated_at, &self.id],
            );
        }
        // todo: add Result for cleaner code
        return ("".to_string(), params![]);
    }

    pub fn from_row(row: &duckdb::Row, stmt: &Statement) -> Self {
        Person {
            id: row.get(stmt.column_index("id")?)?,
            first_name: row.get(stmt.column_index("first_name")?)?,
            last_name: row.get(stmt.column_index("last_name")?)?,
            national_code: row.get(stmt.column_index("national_code")?)?,
            phone: row.get(stmt.column_index("phone")?)?,
            email: row.get(stmt.column_index("email")?)?,
            address: row.get(stmt.column_index("address")?)?,
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

        let id: Result<String> = value_ref_to_type(m.get("id").unwrap()).map_err(Error::from);
        if let Err(e) = id {
            return Err(e);
        }
        let id = id.unwrap();

        let first_name: Result<String> =
            value_ref_to_type(m.get("first_name").unwrap()).map_err(Error::from);
        if let Err(e) = first_name {
            return Err(e);
        }
        let first_name = first_name.unwrap();

        let last_name: Result<String> =
            value_ref_to_type(m.get("last_name").unwrap()).map_err(Error::from);
        if let Err(e) = last_name {
            return Err(e);
        }
        let last_name = last_name.unwrap();

        let national_code: Result<Option<String>> =
            value_ref_to_type(m.get("national_code").unwrap()).map_err(Error::from);
        if let Err(e) = national_code {
            return Err(e);
        }
        let national_code = national_code.unwrap();

        let phone: Result<Option<String>> =
            value_ref_to_type(m.get("phone").unwrap()).map_err(Error::from);
        if let Err(e) = phone {
            return Err(e);
        }
        let phone = phone.unwrap();

        let email: Result<Option<String>> =
            value_ref_to_type(m.get("email").unwrap()).map_err(Error::from);
        if let Err(e) = email {
            return Err(e);
        }
        let email = email.unwrap();

        let address: Result<Option<String>> =
            value_ref_to_type(m.get("address").unwrap()).map_err(Error::from);
        if let Err(e) = address {
            return Err(e);
        }
        let address = address.unwrap();

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
}
impl Model for Person {
    const MODEL_QUERY_PREFIX: String = String::from("person_");
    const TABLE_NAME: String = String::from("person");
    fn get_id(&self) -> String {
        self.id.clone()
    }
}
