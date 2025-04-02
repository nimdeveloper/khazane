#![allow(dead_code)]
use serde::{de::DeserializeOwned, Serialize};
use surrealdb::{Error, Surreal};
use tauri::async_runtime::Mutex;

use crate::app::AppData;

pub struct Repository<T> {
    table_name: String,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Repository<T>
where
    T: DeserializeOwned + Serialize + Send + Sync,
{
    pub fn new(table_name: &str) -> Self {
        Self {
            table_name: table_name.to_string(),
            _marker: std::marker::PhantomData,
        }
    }

    pub async fn find_all(
        &self,
        db: &Surreal<surrealdb::engine::local::Db>,
    ) -> Result<Vec<T>, Error> {
        db.select::<Vec<T>>(&self.table_name).await
    }

    pub async fn find_by_id(
        &self,
        db: &Surreal<surrealdb::engine::local::Db>,
        id: &str,
    ) -> Result<Option<T>, Error> {
        db.select::<Option<T>>((self.table_name.as_str(), id)).await
    }

    pub async fn create<D>(
        &self,
        db: &Surreal<surrealdb::engine::local::Db>,
        item: D,
    ) -> Result<Option<T>, Error>
    where
        D: Serialize + 'static,
    {
        db.create(&self.table_name).content(item).await
    }

    pub async fn update<D>(
        &self,
        db: &Surreal<surrealdb::engine::local::Db>,
        id: &str,
        item: D,
    ) -> Result<Option<T>, Error>
    where
        D: Serialize + 'static,
    {
        db.update((self.table_name.as_str(), id)).merge(item).await
    }

    pub async fn delete(
        &self,
        db: &Surreal<surrealdb::engine::local::Db>,
        id: &str,
    ) -> Result<Option<T>, Error> {
        db.delete((self.table_name.as_str(), id)).await
    }
}

pub async fn get_db(
    state: &Mutex<AppData>,
) -> Result<Surreal<surrealdb::engine::local::Db>, Error> {
    let app_data = state.lock().await;
    if let Some(db) = &app_data.db {
        Ok(db.clone())
    } else {
        Err(Error::from(surrealdb::error::Db::Unreachable(
            "DB not initialized".to_string(),
        )))
    }
}
