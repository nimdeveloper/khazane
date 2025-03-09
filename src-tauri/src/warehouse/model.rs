use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Serialize, Deserialize)]
pub struct WarehouseColor {
    key: String,
    color: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Warehouse {
    id: RecordId,
    name: String,
    shorthand: String,
    color: Option<WarehouseColor>,
}
