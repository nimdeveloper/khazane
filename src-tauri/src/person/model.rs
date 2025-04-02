use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Deserialize, Serialize)]
pub struct Person {
    pub id: RecordId,
    pub full_name: String,
}
