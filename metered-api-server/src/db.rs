use std::{env, str::FromStr};

use color_eyre::eyre::Result;
use log::LevelFilter;
use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, Pool, Sqlite, SqlitePool};

pub mod models;

pub async fn create_db_pool() -> Result<Pool<Sqlite>> {
    let url = env::var("DATABASE_URL")?;

    let options = SqliteConnectOptions::from_str(&url)?
        .create_if_missing(true)
        .log_statements(LevelFilter::Trace);

    let pool = SqlitePool::connect_with(options).await?;

    Ok(pool)
}
