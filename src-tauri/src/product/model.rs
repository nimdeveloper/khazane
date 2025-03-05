use serde::{Deserialize, Serialize};
use surrealdb::{sql::Datetime, RecordId};

use crate::warehouse::model::Warehouse;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductCategory<'a> {
    id: RecordId,
    label: &'a str,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MeasurementUnit<'a> {
    id: RecordId,
    title: &'a str,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Product<'a> {
    id: RecordId,
    title: &'a str,
    code: &'a str,
    unit: Option<MeasurementUnit<'a>>,
    base_price: i64,
    inventory: i64,
    initial_inventory: i64,
    status: &'a str,
    image: &'a str,
    created_at: Datetime,
    updated_at: Datetime,
    ware_houses: Vec<Warehouse<'a>>,
    category: Option<ProductCategory<'a>>,
}
