use axum::{extract::State, http::StatusCode, Form, Json};
use sqlx::{Pool, Sqlite};
use validator::{Validate, ValidationError, ValidationErrors};

use crate::{
    api::Response,
    data::{CodeRunInputDto, CodeRunResponseDto, InternalServerErrorDto, UnauthorizedDto},
    services::{api_keys_service, api_requests_service},
};

pub async fn run_code(
    State(pool): State<Pool<Sqlite>>,
    Form(data): Form<CodeRunInputDto>,
) -> (StatusCode, Json<Response<CodeRunResponseDto>>) {
    if let Err(errors) = data.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(Response::ValidationError(errors)),
        );
    };

    let api_key_row = api_keys_service::fetch_one_by_api_key(&pool, data.api_key())
        .await
        .unwrap();
    if let None = api_key_row {
        return (
            StatusCode::UNAUTHORIZED,
            Json(Response::Unauthorized(UnauthorizedDto::new())),
        );
    }

    let api_key_row = api_key_row.unwrap();
    let below_quota_result = api_requests_service::check_quota(&pool, api_key_row.id()).await;
    if let Err(_) = below_quota_result {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Response::ServerError(InternalServerErrorDto::new())),
        );
    }
    if !below_quota_result.unwrap() {
        let mut errors = ValidationErrors::new();
        let error = ValidationError::new("quota_exceeded").with_message("Quota exceeded".into());

        errors.add("apiKey", error);

        return (
            StatusCode::BAD_REQUEST,
            Json(Response::ValidationError(errors)),
        );
    }

    let params = [("language", data.language()), ("code", data.code())];

    let client = reqwest::Client::new();
    match client
        .post("https://api.codex.jaagrav.in")
        .form(&params)
        .send()
        .await
    {
        Ok(resp) => match resp.json::<CodeRunResponseDto>().await {
            Ok(response_body) => {
                match api_requests_service::log_api_request(&pool, api_key_row.id(), data.code())
                    .await
                {
                    Ok(_) => (StatusCode::OK, Json(Response::Success(response_body))),
                    Err(_) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(Response::ServerError(InternalServerErrorDto::new())),
                    ),
                }
            }
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Response::ServerError(InternalServerErrorDto::new())),
            ),
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Response::ServerError(InternalServerErrorDto::new())),
        ),
    }
}
