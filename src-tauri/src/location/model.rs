use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use surrealdb::Surreal;

#[derive(Debug, Deserialize, Serialize)]
pub struct Location<'a> {
    id: RecordId,
    name: &'a str,
}
