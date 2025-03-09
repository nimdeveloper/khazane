use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Deserialize, Serialize)]
pub struct Location<'a> {
    id: RecordId,
    name: &'a str,
}
