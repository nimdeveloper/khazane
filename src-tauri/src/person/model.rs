use crate::core::repository::Model;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Person {
    pub id: String,
    pub full_name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Model for Person {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_table_name() -> &'static str {
        "person"
    }
}
