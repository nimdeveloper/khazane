#![allow(dead_code)]
use async_trait::async_trait;
use chrono::Utc;
use duckdb::params;
use duckdb::Connection;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::marker::PhantomData;
use tauri::async_runtime::Mutex;
use uuid::Uuid;

use crate::app::AppData;
use crate::core::database;
use crate::core::error::{custom_error, Error, Result};

/// Base model trait that all models must implement
pub trait Model: Serialize + DeserializeOwned + Send + Sync {
    const MODEL_QUERY_PREFIX: String;
    const TABLE_NAME: String;
    fn get_id(&self) -> String;
}

/// Timestamps for models
#[derive(Debug, Serialize, Deserialize)]
pub struct Timestamps {
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

/// Repository trait for database operations
#[async_trait]
pub trait Repository<T: Model> {
    async fn find_all(&self) -> Result<Vec<T>>;
    async fn find_by_id(&self, id: &str) -> Result<Option<T>>;
    async fn create<D: Serialize + Send>(&self, item: D) -> Result<T>;
    async fn update<D: Serialize + Send>(&self, id: &str, item: D) -> Result<T>;
    async fn delete(&self, id: &str) -> Result<bool>;

    // New method to ensure the table exists
    async fn ensure_table_exists(&self) -> Result<()>;
}

/// DuckDB implementation of the repository
pub struct DuckDbRepository<T: Model> {
    db_path: String,
    _marker: PhantomData<T>,
}

impl<T: Model> DuckDbRepository<T> {
    pub fn new(db_path: String) -> Self {
        Self {
            db_path,
            _marker: PhantomData,
        }
    }
    pub fn get_connection(&self) -> Result<Connection> {
        database::get_connection(self.db_path.as_str())
    }
}

#[async_trait]
impl<T: Model + 'static> Repository<T> for DuckDbRepository<T> {
    async fn ensure_table_exists(&self) -> Result<()> {
        let conn = database::get_connection(self.db_path.as_str())?;
        let table_name = T::get_table_name();

        if !database::table_exists(&conn, table_name)? {
            // Create the table if it doesn't exist
            let sql = format!("CREATE TABLE IF NOT EXISTS {} (data JSON)", table_name);
            database::execute(&conn, &sql)?;
            println!("Created table: {}", table_name);
        }

        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<bool> {
        self.ensure_table_exists().await?;
        let conn = database::get_connection(self.db_path.as_str())?;
        let table_name = T::get_table_name();
        let sql = format!("DELETE FROM {} WHERE data->>'id' = ?", table_name);

        let mut stmt = conn.prepare(&sql)?;
        let count = stmt.execute([id]).map_err(Error::from)?;

        Ok(count > 0)
    }

    async fn find_all(&self) -> Result<Vec<T>> {
        self.ensure_table_exists().await?;
        let conn = database::get_connection(self.db_path.as_str())?;
        let table_name = T::get_table_name();
        let sql = format!("SELECT data FROM {}", table_name);

        let mut stmt = conn.prepare(&sql)?;
        let mut rows = stmt.query([]).map_err(Error::from)?;

        let mut items = Vec::new();
        while let Some(row) = rows.next().map_err(Error::from)? {
            let json_str: String = row.get(0).map_err(Error::from)?;
            let item: T = serde_json::from_str(&json_str)
                .map_err(|e| custom_error(format!("Failed to deserialize: {}", e)))?;
            items.push(item);
        }

        Ok(items)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<T>> {
        self.ensure_table_exists().await?;
        let conn = database::get_connection(self.db_path.as_str())?;
        let table_name = T::get_table_name();
        let sql = format!("SELECT data FROM {} WHERE data->>'id' = ?", table_name);

        let mut stmt = conn.prepare(&sql)?;
        let mut rows = stmt.query([id]).map_err(Error::from)?;

        if let Some(row) = rows.next().map_err(Error::from)? {
            let json_str: String = row.get(0).map_err(Error::from)?;
            let item: T = serde_json::from_str(&json_str)
                .map_err(|e| custom_error(format!("Failed to deserialize: {}", e)))?;
            Ok(Some(item))
        } else {
            Ok(None)
        }
    }

    async fn create<D: Serialize + Send>(&self, item: D) -> Result<T> {
        self.ensure_table_exists().await?;
        let conn = database::get_connection(self.db_path.as_str())?;
        let table_name = T::get_table_name();

        // Generate UUID for the new entity
        let id = Uuid::new_v4().to_string();

        // Add timestamps
        let now = Utc::now();

        // Create a combination of the item with ID and timestamps
        let mut json_value = serde_json::to_value(item)
            .map_err(|e| custom_error(format!("Failed to serialize: {}", e)))?;

        if let serde_json::Value::Object(ref mut map) = json_value {
            map.insert("id".to_string(), serde_json::Value::String(id));
            map.insert(
                "created_at".to_string(),
                serde_json::Value::String(now.to_rfc3339()),
            );
            map.insert(
                "updated_at".to_string(),
                serde_json::Value::String(now.to_rfc3339()),
            );
        }

        let json_str = serde_json::to_string(&json_value)
            .map_err(|e| custom_error(format!("Failed to serialize to string: {}", e)))?;

        // Insert the entity into the table
        let sql = format!("INSERT INTO {} (data) VALUES (?)", table_name);
        let mut stmt = conn.prepare(&sql)?;
        stmt.execute([json_str.clone()]).map_err(Error::from)?;

        // Deserialize back to the model type
        let item: T = serde_json::from_str(&json_str)
            .map_err(|e| custom_error(format!("Failed to deserialize created item: {}", e)))?;

        Ok(item)
    }

    async fn update<D: Serialize + Send>(&self, id: &str, item: D) -> Result<T> {
        self.ensure_table_exists().await?;
        let conn = database::get_connection(self.db_path.as_str())?;
        let table_name = T::get_table_name();

        // Get existing record to preserve created_at
        let sql = format!("SELECT data FROM {} WHERE data->>'id' = ?", table_name);
        let mut stmt = conn.prepare(&sql)?;
        let mut rows = stmt.query([id]).map_err(Error::from)?;

        let existing_json = if let Some(row) = rows.next().map_err(Error::from)? {
            let json_str: String = row.get(0).map_err(Error::from)?;
            serde_json::from_str::<serde_json::Value>(&json_str)
                .map_err(|e| custom_error(format!("Failed to parse existing JSON: {}", e)))?
        } else {
            return Err(custom_error(format!("Record with id {} not found", id)));
        };

        // Extract created_at from existing record
        let created_at = if let serde_json::Value::Object(ref map) = existing_json {
            map.get("created_at").cloned()
        } else {
            None
        };

        // Add id, created_at, and updated_at to the update
        let mut json_value = serde_json::to_value(item)
            .map_err(|e| custom_error(format!("Failed to serialize: {}", e)))?;

        let now = Utc::now();

        if let serde_json::Value::Object(ref mut map) = json_value {
            map.insert("id".to_string(), serde_json::Value::String(id.to_string()));

            if let Some(created_at) = created_at {
                map.insert("created_at".to_string(), created_at);
            }

            map.insert(
                "updated_at".to_string(),
                serde_json::Value::String(now.to_rfc3339()),
            );
        }

        let json_str = serde_json::to_string(&json_value)
            .map_err(|e| custom_error(format!("Failed to serialize to string: {}", e)))?;

        // Update the record
        let sql = format!("UPDATE {} SET data = ? WHERE data->>'id' = ?", table_name);
        let mut stmt = conn.prepare(&sql)?;
        stmt.execute(params![json_str.clone(), id])
            .map_err(Error::from)?;

        // Deserialize back to the model type
        let updated_item: T = serde_json::from_str(&json_str)
            .map_err(|e| custom_error(format!("Failed to deserialize updated item: {}", e)))?;

        Ok(updated_item)
    }
}

/// Get a repository for the specific model type
pub async fn get_repository<T: Model + 'static>(
    state: &tauri::State<'_, Mutex<AppData>>,
) -> Result<impl Repository<T>> {
    // Make sure we have a database connection
    let app_data = state.lock().await;

    // Update the global app data if needed
    if let Some(db_path) = &app_data.db_path {
        // Create and return the repository
        let repo = DuckDbRepository::<T>::new(db_path.clone());
        repo.ensure_table_exists().await?;
        Ok(repo)
    } else {
        Err(custom_error("Database path not initialized"))
    }
}
