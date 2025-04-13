use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PersonDto {
    pub id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub national_code: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
}
