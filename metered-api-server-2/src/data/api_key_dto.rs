use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(FromRow, Serialize)]
pub struct ApiKeyDto {
    #[serde(rename = "apiKey")]
    api_key: String,
}

impl ApiKeyDto {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}
