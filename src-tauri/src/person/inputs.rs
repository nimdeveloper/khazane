use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PersonDto {
    full_name: String,
}
