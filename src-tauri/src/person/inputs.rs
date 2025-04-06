use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PersonDto {
    pub id: Option<String>,
    pub full_name: String,
}
