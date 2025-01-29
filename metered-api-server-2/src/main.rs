use axum::{
    routing::{get, post},
    Router,
};
use color_eyre::eyre::Result;
use dotenv::dotenv;
use metered_api_server::{api::ApiState, db::create_db_pool, handlers};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenv().ok();

    let db = create_db_pool().await?;
    let state = ApiState::new(db);

    let app = Router::new()
        .route("/", get(handlers::welcome))
        .route("/register", post(handlers::register))
        .route("/api-keys", get(handlers::list_api_keys))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
