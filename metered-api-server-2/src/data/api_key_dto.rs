use serde::Serialize;
use sqlx::FromRow;

#[derive(FromRow, Serialize)]
pub struct ApiKeyDto {
    #[serde(rename = "apiKey")]
    api_key: String,
}
