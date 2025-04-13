use crate::core::error::Result;
use tauri::{async_runtime::Mutex, State};

use crate::{
    app::AppData,
    core::repository::{self, Repository},
};

use super::{
    inputs::{MeasurementUnitDto, ProductCategoryDto, ProductDto},
    model::{MeasurementUnit, Product, ProductCategory},
    query::{self, FilterOptions},
};

#[tauri::command]
pub async fn get_categories(
    state: State<'_, Mutex<AppData>>,
    filters: FilterOptions,
) -> Result<Vec<ProductCategory>> {
    let repo = repository::get_repository::<ProductCategory>(&state).await?;
    repo.find_all().await
}

#[tauri::command]
pub async fn get_category_by_id(
    state: State<'_, Mutex<AppData>>,
    id: String,
) -> Result<Option<ProductCategory>> {
    let repo = repository::get_repository::<ProductCategory>(&state).await?;
    match query::get_category_by_id(&repo, &id) {
        Ok(category) => Ok(category),
        Err(e) => {
            eprintln!("Error in get_category_by_id: {}", e);
            repo.find_by_id(&id).await
        }
    }
}

#[tauri::command]
pub async fn add_category(
    state: State<'_, Mutex<AppData>>,
    category: ProductCategoryDto,
) -> Result<ProductCategory> {
    let repo = repository::get_repository::<ProductCategory>(&state).await?;
    match query::create_category(&repo, &category) {
        Ok(category) => Ok(category),
        Err(e) => {
            eprintln!("Error in create_category: {}", e);
            repo.create(category).await
        }
    }
}

#[tauri::command]
pub async fn update_category(
    state: State<'_, Mutex<AppData>>,
    id: String,
    category: ProductCategoryDto,
) -> Result<ProductCategory> {
    let repo = repository::get_repository::<ProductCategory>(&state).await?;
    match query::update_category(&repo, &id, &category) {
        Ok(category) => Ok(category),
        Err(e) => {
            eprintln!("Error in update_category: {}", e);
            repo.update(&id, category).await
        }
    }
}

#[tauri::command]
pub async fn get_products(
    state: State<'_, Mutex<AppData>>,
    filters: FilterOptions,
) -> Result<Vec<Product>> {
    let repo = repository::get_repository::<Product>(&state).await?;
    match query::get_product_with_filter(&repo, &filters) {
        Ok(products) => Ok(products),
        Err(e) => {
            eprintln!("Error in get_product_with_filter: {}", e);
            repo.find_all().await
        }
    }
}

#[tauri::command]
pub async fn get_product_by_id(
    state: State<'_, Mutex<AppData>>,
    id: String,
) -> Result<Option<Product>> {
    let repo = repository::get_repository::<Product>(&state).await?;
    match query::get_product_by_id(&repo, &id) {
        Ok(product) => Ok(product),
        Err(e) => {
            eprintln!("Error in get_product_by_id: {}", e);
            repo.find_by_id(&id).await
        }
    }
}

#[tauri::command]
pub async fn add_product(state: State<'_, Mutex<AppData>>, product: ProductDto) -> Result<Product> {
    let repo = repository::get_repository::<Product>(&state).await?;
    match query::create_product(&repo, &product) {
        Ok(product) => Ok(product),
        Err(e) => {
            eprintln!("Error in create_product: {}", e);
            repo.create(product).await
        }
    }
}

#[tauri::command]
pub async fn update_product(
    state: State<'_, Mutex<AppData>>, 
    id: String,
    product: ProductDto
) -> Result<Product> {
    let repo = repository::get_repository::<Product>(&state).await?;
    match query::update_product(&repo, &id, &product) {
        Ok(product) => Ok(product),
        Err(e) => {
            eprintln!("Error in update_product: {}", e);
            repo.update(&id, product).await
        }
    }
}

#[tauri::command]
pub async fn get_measure_units(
    state: State<'_, Mutex<AppData>>,
    filters: FilterOptions,
) -> Result<Vec<MeasurementUnit>> {
    let repo = repository::get_repository::<MeasurementUnit>(&state).await?;
    repo.find_all().await
}

#[tauri::command]
pub async fn get_measure_unit_by_id(
    state: State<'_, Mutex<AppData>>,
    id: String,
) -> Result<Option<MeasurementUnit>> {
    let repo = repository::get_repository::<MeasurementUnit>(&state).await?;
    match query::get_measurement_unit_by_id(&repo, &id) {
        Ok(unit) => Ok(unit),
        Err(e) => {
            eprintln!("Error in get_measurement_unit_by_id: {}", e);
            repo.find_by_id(&id).await
        }
    }
}

#[tauri::command]
pub async fn create_measure_unit(
    state: State<'_, Mutex<AppData>>,
    unit: MeasurementUnitDto,
) -> Result<MeasurementUnit> {
    let repo = repository::get_repository::<MeasurementUnit>(&state).await?;
    match query::create_measurement_unit(&repo, &unit) {
        Ok(unit) => Ok(unit),
        Err(e) => {
            eprintln!("Error in create_measurement_unit: {}", e);
            repo.create(unit).await
        }
    }
}

#[tauri::command]
pub async fn update_measure_unit(
    state: State<'_, Mutex<AppData>>,
    id: String,
    unit: MeasurementUnitDto,
) -> Result<MeasurementUnit> {
    let repo = repository::get_repository::<MeasurementUnit>(&state).await?;
    match query::update_measurement_unit(&repo, &id, &unit) {
        Ok(unit) => Ok(unit),
        Err(e) => {
            eprintln!("Error in update_measurement_unit: {}", e);
            repo.update(&id, unit).await
        }
    }
}
