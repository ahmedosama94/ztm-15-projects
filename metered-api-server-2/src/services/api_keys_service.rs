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

    let api_key_row: ApiKeyRow = sqlx::query_as(
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

pub async fn fetch_all(pool: &Pool<Sqlite>) -> Result<Vec<ApiKeyRow>, Box<dyn std::error::Error>> {
    let api_key_rows: Vec<ApiKeyRow> = sqlx::query_as("SELECT * FROM api_keys")
        .fetch_all(pool)
        .await?;

    Ok(api_key_rows)
}

pub async fn fetch_one_by_api_key(
    pool: &Pool<Sqlite>,
    api_key: &str,
) -> Result<Option<ApiKeyRow>, Box<dyn std::error::Error>> {
    let api_key_row: Option<ApiKeyRow> =
        sqlx::query_as("SELECT * FROM api_keys WHERE api_key = $1")
            .bind(api_key)
            .fetch_optional(pool)
            .await?;

    Ok(api_key_row)
}
