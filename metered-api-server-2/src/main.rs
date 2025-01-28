use std::{env, str::FromStr};

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use color_eyre::eyre::Result;
use dotenv::dotenv;
use log::LevelFilter;
use metered_api_server_2::{ApiKeyDto, ApiState, InternalServerErrorDto, Registration, Response};
use rand::RngCore;
use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, Pool, Sqlite, SqlitePool};
use validator::{Validate, ValidationError, ValidationErrors};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenv().ok();

    let db = create_db_pool().await?;
    let state = ApiState::new(db);

    let app = Router::new()
        .route("/", get(welcome))
        .route("/register", post(register))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn welcome() -> &'static str {
    "Welcome to API inc.\n"
}

async fn register(
    State(pool): State<Pool<Sqlite>>,
    Json(registration_data): Json<Registration>,
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

async fn create_db_pool() -> Result<Pool<Sqlite>> {
    let url = env::var("DATABASE_URL")?;

    let options = SqliteConnectOptions::from_str(&url)?
        .create_if_missing(true)
        .log_statements(LevelFilter::Trace);

    let pool = SqlitePool::connect_with(options).await?;

    Ok(pool)
}
