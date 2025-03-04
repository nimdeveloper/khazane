use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use surrealdb::Surreal;

#[derive(Debug, Deserialize, Serialize)]
pub struct Person<'a> {
    id: RecordId,
    full_name: &'a str,
}
