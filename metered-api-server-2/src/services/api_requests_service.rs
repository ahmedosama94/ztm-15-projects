use sqlx::{prelude::FromRow, Pool, Sqlite};

use crate::db::models::ApiRequestRow;

pub async fn log_api_request(
    pool: &Pool<Sqlite>,
    api_key_id: u32,
    code: &str,
) -> Result<ApiRequestRow, Box<dyn std::error::Error>> {
    let line_count = code.lines().count() as u32;

    let api_request_row: ApiRequestRow = sqlx::query_as(
        "INSERT INTO api_requests
        (api_key_id, lines_of_code) VALUES ($1, $2)
        RETURNING *",
    )
    .bind(api_key_id)
    .bind(line_count)
    .fetch_one(pool)
    .await?;

    Ok(api_request_row)
}

#[derive(FromRow)]
struct CountRow {
    count: u32,
}

const MAX_REQUESTS_PER_DURATION: u32 = 10;
const DURATION_MS: &str = "-1 minute";

pub async fn check_quota(
    pool: &Pool<Sqlite>,
    api_key_id: u32,
) -> Result<bool, Box<dyn std::error::Error>> {
    let CountRow { count } = sqlx::query_as(
        "SELECT COUNT(*) AS count
        FROM api_requests
        WHERE api_key_id = $1
        AND created_at > datetime('now', $2)",
    )
    .bind(api_key_id)
    .bind(DURATION_MS)
    .fetch_one(pool)
    .await?;

    Ok(count < MAX_REQUESTS_PER_DURATION)
}
