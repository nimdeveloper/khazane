use serde::Deserialize;
use surrealdb::Error::{self, Db};
use tauri::{async_runtime::Mutex, State};

use crate::{app::AppData, product::model::ProductCategory};

use super::{
    inputs::{ProductCategoryDto, ProductDto},
    model::Product,
};

#[derive(Deserialize, Debug)]
pub struct FilterOptions {}

#[tauri::command]
pub async fn get_categories(
    state: State<'_, Mutex<AppData>>,
    filters: FilterOptions,
) -> Result<Vec<ProductCategory>, Error> {
    dbg!(filters);
    let app_data = state.lock().await;
    if let Some(db) = &app_data.db {
        let data = db.select::<Vec<ProductCategory>>("category").await;
        if let Ok(categories) = data {
            dbg!(&categories);
            return Ok(categories);
        } else if let Err(temp) = data {
            dbg!(temp);
        }
        println!("OPS");
    }
    Err(Error::from(Db(surrealdb::error::Db::Unreachable(
        "DB not initialized!!".to_string(),
    ))))
}

#[tauri::command]
pub async fn add_category(
    state: State<'_, Mutex<AppData>>,
    category: ProductCategoryDto,
) -> Result<Option<ProductCategory>, Error> {
    let app_data = state.lock().await;
    if let Some(db) = &app_data.db {
        let created: Option<ProductCategory> = db.create("category").content(category).await?;
        dbg!(&created);
        return Ok(created);
    }
    Err(Error::from(Db(surrealdb::error::Db::Unreachable(
        "DB not initialized!!".to_string(),
    ))))
}

#[tauri::command]
pub async fn get_products(
    state: State<'_, Mutex<AppData>>,
    filters: FilterOptions,
) -> Result<Vec<Product>, Error> {
    dbg!(filters);
    let app_data = state.lock().await;
    if let Some(db) = &app_data.db {
        let data = db.select::<Vec<Product>>("product").await;
        if let Ok(products) = data {
            dbg!(&products);
            return Ok(products);
        } else if let Err(temp) = data {
            dbg!(temp);
        }
        println!("OPS");
    }
    Err(Error::from(Db(surrealdb::error::Db::Unreachable(
        "DB not initialized!!".to_string(),
    ))))
}

#[tauri::command]
pub async fn add_product(
    state: State<'_, Mutex<AppData>>,
    product: ProductDto,
) -> Result<Option<Product>, Error> {
    dbg!(&product);
    let app_data = state.lock().await;
    if let Some(db) = &app_data.db {
        let created: Option<Product> = db.create("product").content(product).await?;
        return Ok(created);
    }
    Err(Error::from(Db(surrealdb::error::Db::Unreachable(
        "DB not initialized!!".to_string(),
    ))))
}
