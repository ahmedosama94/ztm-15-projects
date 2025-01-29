use axum::{extract::State, http::StatusCode, Json};
use sqlx::{Pool, Sqlite};

use crate::{
    api::Response,
    data::{ApiKeyDto, InternalServerErrorDto},
    services::api_keys_service,
};

pub async fn list_api_keys(
    State(pool): State<Pool<Sqlite>>,
) -> (StatusCode, Json<Response<Vec<ApiKeyDto>>>) {
    match api_keys_service::fetch_all_api_key_rows(&pool).await {
        Ok(api_key_rows) => {
            let api_key_dtos = api_key_rows
                .iter()
                .map(|dto| {
                    ApiKeyDto::new(dto.id(), dto.email().to_owned(), dto.api_key().to_owned())
                })
                .collect();

            (StatusCode::OK, Json(Response::Success(api_key_dtos)))
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Response::ServerError(InternalServerErrorDto::new())),
        ),
    }
}
