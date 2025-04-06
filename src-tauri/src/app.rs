use crate::core::database;
use crate::core::error::{custom_error, Result};
use duckdb::Connection;

#[derive(Default)]
pub struct AppData {
    pub db_path: Option<String>,
    pub migrations_applied: bool,
}

impl AppData {
    // Get a new connection using the db_path
    pub fn get_connection(&self) -> Result<Connection> {
        match &self.db_path {
            Some(path) => database::get_connection(path),
            None => Err(custom_error("Database path not initialized")),
        }
    }
}
