use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Serialize, Deserialize)]
pub struct WarehouseColor {
    pub key: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Warehouse {
    pub id: RecordId,
    pub name: String,
    pub shorthand: String,
    pub color: Option<WarehouseColor>,
}
