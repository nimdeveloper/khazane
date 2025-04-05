use serde::Deserialize;
use surrealdb::Error;
use tauri::{async_runtime::Mutex, State};

use crate::{
    app::AppData,
    core::repository::{self, Repository},
    product::model::ProductCategory,
};

use super::{
    inputs::{MeasurementUnitDto, ProductCategoryDto, ProductDto},
    model::{MeasurementUnit, Product},
};

#[derive(Deserialize, Debug)]
pub struct FilterOptions {}

#[tauri::command]
pub async fn get_categories(
    state: State<'_, Mutex<AppData>>,
    filters: FilterOptions,
) -> Result<Vec<ProductCategory>, Error> {
    let db = repository::get_db(&state).await?;
    let repository = Repository::<ProductCategory>::new("category");
    repository.find_all(&db).await
}

#[tauri::command]
pub async fn create_category(
    state: State<'_, Mutex<AppData>>,
    category: ProductCategoryDto,
) -> Result<Option<ProductCategory>, Error> {
    let db = repository::get_db(&state).await?;
    let repository = Repository::<ProductCategory>::new("category");
    repository.create(&db, category).await
}

#[tauri::command]
pub async fn get_products(
    state: State<'_, Mutex<AppData>>,
    filters: FilterOptions,
) -> Result<Vec<Product>, Error> {
    let db = repository::get_db(&state).await?;
    let repository = Repository::<Product>::new("product");
    repository.find_all(&db).await
}

#[tauri::command]
pub async fn create_product(
    state: State<'_, Mutex<AppData>>,
    product: ProductDto,
) -> Result<Option<Product>, Error> {
    let db = repository::get_db(&state).await?;
    let repository = Repository::<Product>::new("product");
    repository.create(&db, product).await
}

#[tauri::command]
pub async fn get_measure_units(
    state: State<'_, Mutex<AppData>>,
    filters: FilterOptions,
) -> Result<Vec<MeasurementUnit>, Error> {
    let db = repository::get_db(&state).await?;
    let repository = Repository::<MeasurementUnit>::new("measure_unit");
    repository.find_all(&db).await
}

#[tauri::command]
pub async fn create_measure_unit(
    state: State<'_, Mutex<AppData>>,
    unit: MeasurementUnitDto,
) -> Result<Option<MeasurementUnit>, Error> {
    let db = repository::get_db(&state).await?;
    let repository = Repository::<MeasurementUnit>::new("measure_unit");
    repository.create(&db, unit).await
}
