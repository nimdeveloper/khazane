use serde::{Deserialize, Serialize};
use surrealdb::{sql::Datetime, RecordId};

use crate::warehouse::model::Warehouse;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductCategory {
    id: RecordId,
    pub label: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductWarehouse {
    quantity: i64,
    warehouse: Option<Warehouse>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MeasurementUnit {
    id: RecordId,
    title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    id: RecordId,
    title: String,
    code: String,
    unit: Option<MeasurementUnit>,
    base_price: i64,
    inventory: i64,
    initial_inventory: i64,
    status: String,
    image: String,
    created_at: Datetime,
    updated_at: Datetime,
    ware_houses: Vec<ProductWarehouse>,
    category: Option<ProductCategory>,
}
