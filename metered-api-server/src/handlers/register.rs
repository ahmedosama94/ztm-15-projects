use axum::{extract::State, http::StatusCode, Json};
use sqlx::{Pool, Sqlite};
use validator::{Validate, ValidationError, ValidationErrors};

use crate::{
    api::Response,
    data::{InternalServerErrorDto, RegistrationInputDto, RegistrationResponseDto},
    services::api_keys_service,
};

pub async fn register(
    State(pool): State<Pool<Sqlite>>,
    Json(registration_data): Json<RegistrationInputDto>,
) -> (StatusCode, Json<Response<RegistrationResponseDto>>) {
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

    match api_keys_service::create_api_key_row(&pool, registration_data.email()).await {
        Ok(api_key_row) => {
            let api_key_dto = RegistrationResponseDto::new(api_key_row.api_key().to_owned());

            (StatusCode::CREATED, Json(Response::Success(api_key_dto)))
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Response::ServerError(InternalServerErrorDto::new())),
        ),
    }
}
