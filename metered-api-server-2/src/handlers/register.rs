use axum::{extract::State, http::StatusCode, Json};
use rand::RngCore;
use sqlx::{Pool, Sqlite};
use validator::{Validate, ValidationError, ValidationErrors};

use crate::{
    api::Response,
    data::{ApiKeyDto, InternalServerErrorDto, RegistrationDto},
};

pub async fn register(
    State(pool): State<Pool<Sqlite>>,
    Json(registration_data): Json<RegistrationDto>,
) -> (StatusCode, Json<Response<ApiKeyDto>>) {
    if let Err(err) = registration_data.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(Response::ValidationError(err)),
        );
    }

    if let Err(err) = registration_data.validate_email_uniqueness(&pool).await {
        return if err.is::<ValidationError>() {
            let err = *err.downcast::<ValidationError>().unwrap();

            let mut errors = ValidationErrors::new();
            errors.add("email", err);

            (
                StatusCode::BAD_REQUEST,
                Json(Response::ValidationError(errors)),
            )
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Response::ServerError(InternalServerErrorDto::new())),
            )
        };
    }

    let email = registration_data.email();
    let mut bytes = [0; 256];
    rand::rng().fill_bytes(&mut bytes);
    let api_key = hex::encode(bytes);

    match sqlx::query_as(
        "INSERT INTO api_keys
        (email, api_key) VALUES ($1, $2)
        RETURNING api_key",
    )
    .bind(&email)
    .bind(&api_key)
    .fetch_one(&pool)
    .await
    {
        Ok(api_key_dto) => (StatusCode::CREATED, Json(Response::Success(api_key_dto))),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Response::ServerError(InternalServerErrorDto::new())),
        ),
    }
}
