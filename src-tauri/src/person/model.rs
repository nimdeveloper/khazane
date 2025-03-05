use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Deserialize, Serialize)]
pub struct Person<'a> {
    id: RecordId,
    full_name: &'a str,
}
