use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WarehouseColorDto {
    pub key: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WarehouseDto {
    pub name: String,
    pub shorthand: String,
    pub color: Option<WarehouseColorDto>,
}
