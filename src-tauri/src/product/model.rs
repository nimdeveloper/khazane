use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::{sql::Datetime, RecordId};

#[derive(Debug, Serialize)]
pub struct MeasurementUnit<'a> {
    id: RecordId,
    title: &'a str,
}

#[derive(Debug, Serialize)]
pub struct Product<'a> {
    id: RecordId,
    title: &'a str,
    code: &'a str,
    unit: MeasurementUnit<'a>,
    base_price: &'a int,
    inventory: &'a int,
    initial_inventory: &'a int,
    status: &'a str,
    image: &'a str,
    created_at: Datetime,
    updated_at: Datetime,
}
