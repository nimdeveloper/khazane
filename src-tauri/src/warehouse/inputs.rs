use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WarehouseColorDto {
    key: String,
    code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WarehouseDto {
    name: String,
    shorthand: String,
    color: Option<WarehouseColorDto>,
}
