use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
struct WarehouseColor<'a> {
    key: &'a str,
    color: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Warehouse<'a> {
    id: RecordId,
    name: &'a str,
    shorthand: &'a str,
    color: WarehouseColor<'a>,
}
