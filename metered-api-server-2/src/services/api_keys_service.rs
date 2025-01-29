use rand::RngCore;
use sqlx::{Pool, Sqlite};

use crate::db::models::ApiKeyRow;

pub async fn create_api_key_row(
    pool: &Pool<Sqlite>,
    email: &str,
) -> Result<ApiKeyRow, Box<dyn std::error::Error>> {
    let mut bytes = [0; 256];
    rand::rng().fill_bytes(&mut bytes);
    let api_key = hex::encode(bytes);

    let api_key_row = sqlx::query_as::<_, ApiKeyRow>(
        "INSERT INTO api_keys
        (email, api_key) VALUES ($1, $2)
        RETURNING *",
    )
    .bind(&email)
    .bind(&api_key)
    .fetch_one(pool)
    .await?;

    Ok(api_key_row)
}
