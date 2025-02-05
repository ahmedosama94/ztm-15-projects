use std::{
    env,
    rc::Rc,
    str::FromStr,
    sync::{Arc, Mutex},
};

use color_eyre::eyre::Result;
use lazy_static::lazy_static;
use models::TodoItemRow;
use sqlx::{sqlite::SqliteConnectOptions, Connection, Pool, Sqlite, SqliteConnection, SqlitePool};

mod models;

async fn initialize_connection() -> Pool<Sqlite> {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL env var not found");
    let options = SqliteConnectOptions::from_str(&url)
        .unwrap_or_else(|_| {
            panic!(
                "Failed to initialize database connection options from url {}",
                url
            )
        })
        .create_if_missing(true);

    SqlitePool::connect_with(options)
        .await
        .expect("Unable to connect to database")
}

lazy_static! {
    pub static ref CONNECTION: Mutex<Pool<Sqlite>> = {
        Mutex::new(
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async { initialize_connection().await }),
        )
    };
    //pub static ref CONNECTION: Arc<Mutex<Pool<Sqlite>>> = {
    //    Arc::new(Mutex::new(
    //        tokio::runtime::Runtime::new()
    //            .unwrap()
    //            .block_on(async { initialize_connection().await }),
    //    ))
    //};
}

async fn get_all_todos() -> Result<Vec<TodoItemRow>> {
    //let arc = Arc::clone(&CONNECTION);
    let connection = CONNECTION.lock().unwrap();
    let rows = sqlx::query_as("SELECT * FROM todo_items")
        .fetch_all(&(*connection))
        .await
        .unwrap();

    Ok(rows)
}
