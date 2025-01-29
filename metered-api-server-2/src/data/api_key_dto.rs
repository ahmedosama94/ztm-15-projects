use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(FromRow, Serialize)]
pub struct ApiKeyDto {
    id: u32,
    email: String,
    #[serde(rename = "apiKey")]
    api_key: String,
}

impl ApiKeyDto {
    pub fn new(id: u32, email: String, api_key: String) -> Self {
        Self { id, email, api_key }
    }
}
