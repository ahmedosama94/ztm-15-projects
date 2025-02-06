use async_mutex::Mutex;
use log::LevelFilter;
use std::{env, str::FromStr, thread};

use color_eyre::eyre::Result;
use lazy_static::lazy_static;
use models::TodoItemRow;
use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, Pool, Sqlite, SqlitePool};

pub mod models;

async fn initialize_connection() -> Pool<Sqlite> {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL env var not found");
    let options = SqliteConnectOptions::from_str(&url)
        .unwrap_or_else(|_| {
            panic!(
                "Failed to initialize database connection options from url {}",
                url
            )
        })
        .create_if_missing(true)
        .log_statements(LevelFilter::Debug);

    SqlitePool::connect_with(options)
        .await
        .expect("Unable to connect to database")
}

lazy_static! {
    pub static ref CONNECTION: Mutex<Pool<Sqlite>> = {
        thread::spawn(|| {
            Mutex::new(
                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(async { initialize_connection().await }),
            )
        })
        .join()
        .unwrap()
    };
}

pub async fn get_all_todos() -> Result<Vec<TodoItemRow>> {
    let connection = CONNECTION.try_lock().unwrap();
    let rows = sqlx::query_as("SELECT * FROM todo_items")
        .fetch_all(&(*connection))
        .await?;

    Ok(rows)
}

pub async fn edit_todos(id_and_item_pairs: Vec<(u32, String)>) -> Result<()> {
    let values: Vec<String> = id_and_item_pairs
        .iter()
        .map(|(id, item)| format!("({}, {})", id, item))
        .collect();

    let values_str = values.join(", ");

    let connection = CONNECTION.try_lock().unwrap();
    sqlx::query(
        "
        WITH tmp(id, item) AS (VALUES $1)
        UPDATE todo_items SET item = (SELECT item FROM tmp WHERE todo_items.id = tmp.id)
        WHERE id IN (SELECT id FROM tmp)
    ",
    )
    .bind(values_str)
    .execute(&(*connection))
    .await?;

    Ok(())
}

pub async fn edit_todo(id: u32, new_item: &str) -> Result<()> {
    let connection = CONNECTION.try_lock().unwrap();

    sqlx::query("UPDATE todo_items SET item = $1 WHERE id = $2")
        .bind(new_item)
        .bind(id)
        .execute(&(*connection))
        .await?;

    Ok(())
}

pub async fn add_todos(items: &Vec<String>) -> Result<()> {
    let mut param_num = 1;
    let mut value_params = Vec::new();
    for _ in 0..items.len() {
        value_params.push(format!("(${})", param_num));
        param_num += 1;
    }
    let value_params = value_params.join(", ");
    let query_string = format!("INSERT INTO todo_items (item) VALUES {}", value_params);
    let mut query = sqlx::query(&query_string);

    for item in items {
        query = query.bind(item);
    }

    let connection = CONNECTION.try_lock().unwrap();

    query.execute(&(*connection)).await?;

    Ok(())
}

pub async fn add_todo(item: &str) -> Result<()> {
    add_todos(&vec![item.to_string()]).await?;

    Ok(())
}
