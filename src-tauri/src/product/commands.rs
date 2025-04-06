use crate::core::error::Result;
use serde::Deserialize;
use tauri::{async_runtime::Mutex, State};

use crate::{
    app::AppData,
    core::repository::{self, Repository},
};

use super::{
    inputs::{MeasurementUnitDto, ProductCategoryDto, ProductDto},
    model::{MeasurementUnit, Product, ProductCategory},
};

#[derive(Deserialize, Debug)]
pub struct FilterOptions {}

#[tauri::command]
pub async fn get_categories(
    state: State<'_, Mutex<AppData>>,
    _filters: FilterOptions,
) -> Result<Vec<ProductCategory>> {
    let repo = repository::get_repository::<ProductCategory>(&state).await?;
    repo.find_all().await
}

#[tauri::command]
pub async fn add_category(
    state: State<'_, Mutex<AppData>>,
    category: ProductCategoryDto,
) -> Result<ProductCategory> {
    let repo = repository::get_repository::<ProductCategory>(&state).await?;
    repo.create(category).await
}

#[tauri::command]
pub async fn get_products(
    state: State<'_, Mutex<AppData>>,
    _filters: FilterOptions,
) -> Result<Vec<Product>> {
    let repo = repository::get_repository::<Product>(&state).await?;
    repo.find_all().await
}

#[tauri::command]
pub async fn add_product(state: State<'_, Mutex<AppData>>, product: ProductDto) -> Result<Product> {
    let repo = repository::get_repository::<Product>(&state).await?;
    repo.create(product).await
}

#[tauri::command]
pub async fn get_measure_units(
    state: State<'_, Mutex<AppData>>,
    _filters: FilterOptions,
) -> Result<Vec<MeasurementUnit>> {
    let repo = repository::get_repository::<MeasurementUnit>(&state).await?;
    repo.find_all().await
}

#[tauri::command]
pub async fn create_measure_unit(
    state: State<'_, Mutex<AppData>>,
    unit: MeasurementUnitDto,
) -> Result<MeasurementUnit> {
    let repo = repository::get_repository::<MeasurementUnit>(&state).await?;
    repo.create(unit).await
}
