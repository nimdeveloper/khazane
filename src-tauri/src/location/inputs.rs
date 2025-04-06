use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LocationDto {
    pub id: Option<String>,
    pub name: String,
}
