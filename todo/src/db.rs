use async_mutex::{Mutex, MutexGuard};
use log::LevelFilter;
use std::{env, str::FromStr, thread};

use color_eyre::eyre::{OptionExt, Result};
use lazy_static::lazy_static;
use models::TodoItemRow;
use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, Pool, Sqlite, SqlitePool};

pub mod models;

async fn initialize_connection() -> Result<Pool<Sqlite>> {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL env var not found");
    let options = SqliteConnectOptions::from_str(&url)?
        .create_if_missing(true)
        .log_statements(LevelFilter::Trace);

    Ok(SqlitePool::connect_with(options).await?)
}

lazy_static! {
    pub static ref CONNECTION: Mutex<Pool<Sqlite>> = {
        thread::spawn(|| {
            Mutex::new(tokio::runtime::Runtime::new().unwrap().block_on(async {
                initialize_connection()
                    .await
                    .expect("Failed to initialize database connection")
            }))
        })
        .join()
        .expect("Failed to join database initialization thread")
    };
}

pub async fn get_todos(ids: Option<&[u32]>) -> Result<Vec<TodoItemRow>> {
    let mut query_str = String::from("SELECT * FROM todo_items WHERE deleted_at IS NULL");

    let query = if let Some(ids) = ids {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let mut params = Vec::new();
        for i in 1..=ids.len() {
            params.push(format!("${}", i));
        }

        let query_condition = format!(" AND id IN ({})", params.join(", "));
        query_str.push_str(&query_condition);

        let mut query = sqlx::query_as(&query_str);
        for id in ids {
            query = query.bind(id);
        }

        query
    } else {
        sqlx::query_as(&query_str)
    };

    let rows = query.fetch_all(&(*get_connection()?)).await?;

    Ok(rows)
}

pub async fn add_todos(items: &[String]) -> Result<()> {
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

    query.execute(&(*get_connection()?)).await?;

    Ok(())
}

pub async fn edit_todos(id_and_item_pairs: &[(u32, String)]) -> Result<()> {
    let mut param_num = 1;
    let mut value_params = Vec::new();
    for _ in 0..id_and_item_pairs.len() {
        value_params.push(format!("(${}, ${})", param_num, param_num + 1));
        param_num += 2;
    }
    let value_params = value_params.join(", ");
    let query_str = format!(
        "
        WITH tmp(id, item) AS (VALUES {})
        UPDATE todo_items SET item = (SELECT item FROM tmp WHERE todo_items.id = tmp.id)
        WHERE deleted_at IS NULL AND id IN (SELECT id FROM tmp)
        ",
        value_params
    );
    let mut query = sqlx::query(&query_str);

    for (id, item) in id_and_item_pairs {
        query = query.bind(id).bind(item);
    }

    query.execute(&(*get_connection()?)).await?;

    Ok(())
}

pub async fn clear_todos() -> Result<()> {
    sqlx::query("UPDATE todo_items SET deleted_at = datetime('now') WHERE deleted_at IS NULL")
        .execute(&(*get_connection()?))
        .await?;

    Ok(())
}

pub async fn set_todos_done(ids: &[u32]) -> Result<()> {
    let mut params = Vec::new();
    for i in 0..ids.len() {
        params.push(format!("${}", i + 1));
    }

    let query_str = format!(
        "UPDATE todo_items SET done_at = datetime('now') WHERE deleted_at IS NULL AND id IN ({})",
        params.join(", ")
    );
    let mut query = sqlx::query(&query_str);

    for id in ids {
        query = query.bind(id);
    }

    query.execute(&(*get_connection()?)).await?;

    Ok(())
}

pub async fn remove_todos(ids: &[u32]) -> Result<()> {
    if ids.is_empty() {
        return Ok(());
    }

    let mut params = Vec::new();
    for i in 1..=ids.len() {
        params.push(format!("${}", i));
    }

    let query_str = format!(
        "UPDATE todo_items SET deleted_at = datetime('now') WHERE deleted_at IS NULL AND id IN ({})",
        params.join(", ")
    );
    let mut query = sqlx::query(&query_str);
    for id in ids {
        query = query.bind(id);
    }

    query.execute(&(*get_connection()?)).await?;

    Ok(())
}

fn get_connection() -> Result<MutexGuard<'static, Pool<Sqlite>>> {
    CONNECTION
        .try_lock()
        .ok_or_eyre("Failed to obtain lock on connection mutex")
}
