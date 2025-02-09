use log::LevelFilter;
use std::{env, str::FromStr, sync::Arc, thread};

use color_eyre::eyre::Result;
use lazy_static::lazy_static;
use models::{TodoListItemRow, TodoListRow};
use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, Pool, QueryBuilder, Sqlite, SqlitePool};

pub mod models;

async fn initialize_connection() -> Result<Pool<Sqlite>> {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL env var not found");
    let options = SqliteConnectOptions::from_str(&url)?
        .create_if_missing(true)
        .log_statements(LevelFilter::Trace);

    Ok(SqlitePool::connect_with(options).await?)
}

lazy_static! {
    pub static ref CONNECTION: Arc<Pool<Sqlite>> = {
        thread::spawn(|| {
            Arc::new(tokio::runtime::Runtime::new().unwrap().block_on(async {
                initialize_connection()
                    .await
                    .expect("Failed to initialize database connection")
            }))
        })
        .join()
        .expect("Failed to join database initialization thread")
    };
}

pub async fn get_todo_lists(with_deleted: bool) -> Result<Vec<TodoListRow>> {
    let mut qb = QueryBuilder::new("SELECT * FROM todo_lists");

    if !with_deleted {
        qb.push(" WHERE deleted_at IS NULL");
    }

    Ok(qb.build_query_as().fetch_all(&(*get_connection())).await?)
}

pub async fn get_todo_list_by_title(todo_title: &str) -> Result<Option<TodoListRow>> {
    let row = sqlx::query_as("SELECT * FROM todo_lists WHERE title = $1 AND deleted_at IS NULL")
        .bind(todo_title)
        .fetch_optional(&(*get_connection()))
        .await?;

    Ok(row)
}

pub async fn create_todo_list(title: &str) -> Result<()> {
    let mut qb = QueryBuilder::new("INSERT INTO todo_lists (title) ");
    qb.push_values([title], |mut qb, title| {
        qb.push_bind(title);
    });
    let query = qb.build();

    query.execute(&(*get_connection())).await?;

    Ok(())
}

pub async fn get_todo_list_items(todo_id: u32, with_deleted: bool) -> Result<Vec<TodoListItemRow>> {
    let mut qb = QueryBuilder::new("SELECT * FROM todo_list_items WHERE todo_list_id = ");
    qb.push_bind(todo_id);

    if !with_deleted {
        qb.push(" AND deleted_at IS NULL");
    }

    Ok(qb.build_query_as().fetch_all(&(*get_connection())).await?)
}

pub async fn add_todo_list_items(todo_list_id: u32, items: &[String]) -> Result<()> {
    add_todo_list_items_dyn(todo_list_id, items, false).await?;

    Ok(())
}

pub async fn add_todo_list_items_returning(
    todo_list_id: u32,
    items: &[String],
) -> Result<Vec<TodoListItemRow>> {
    let added_rows = add_todo_list_items_dyn(todo_list_id, items, true)
        .await?
        .expect("A vector of added rows should be returned");

    Ok(added_rows)
}

async fn add_todo_list_items_dyn(
    todo_list_id: u32,
    items: &[String],
    returning: bool,
) -> Result<Option<Vec<TodoListItemRow>>> {
    if items.is_empty() {
        return Ok(Some(Vec::new()));
    }

    let mut qb: QueryBuilder<'_, Sqlite> =
        QueryBuilder::new("INSERT INTO todo_list_items (todo_list_id, title) ");

    qb.push_values(items.iter(), |mut qb, item| {
        qb.push_bind(todo_list_id).push_bind(item);
    });

    if returning {
        qb.push("RETURNING *");
        let rows: Vec<TodoListItemRow> =
            qb.build_query_as().fetch_all(&(*get_connection())).await?;

        Ok(Some(rows))
    } else {
        qb.build().execute(&(*get_connection())).await?;

        Ok(None)
    }
}

pub async fn edit_todo_list_items(
    todo_list_id: u32,
    id_and_item_pairs: &[(u32, String)],
) -> Result<Vec<TodoListItemRow>> {
    if id_and_item_pairs.is_empty() {
        return Ok(Vec::new());
    }

    let mut qb = QueryBuilder::new("WITH tmp(id, title) AS (");
    qb.push_values(id_and_item_pairs, |mut qb, (id, title)| {
        qb.push_bind(id).push_bind(title);
    });
    qb.push(")");
    qb.push("\nUPDATE todo_list_items SET title = (SELECT title FROM tmp WHERE todo_list_items.id = tmp.id)");
    qb.push("\nWHERE deleted_at IS NULL AND id IN (SELECT id FROM tmp) AND todo_list_id = ");
    qb.push_bind(todo_list_id);
    qb.push(" RETURNING *");

    let query = qb.sql();
    println!("{}", query);

    let rows = qb.build_query_as().fetch_all(&(*get_connection())).await?;

    Ok(rows)
}

pub async fn clear_todo_list_items(todo_list_id: u32) -> Result<()> {
    let mut qb = QueryBuilder::new("UPDATE todo_list_items SET deleted_at = datetime('now') WHERE deleted_at IS NULL AND todo_list_id = ");
    qb.push_bind(todo_list_id);

    let query = qb.build();
    query.execute(&(*get_connection())).await?;

    Ok(())
}

pub async fn set_todo_list_items_done(todo_list_id: u32, ids: &[u32]) -> Result<()> {
    if ids.is_empty() {
        return Ok(());
    }

    let mut qb = QueryBuilder::new("UPDATE todo_list_items SET done_at = datetime('now') WHERE deleted_at IS NULL AND todo_list_id = ");
    qb.push_bind(todo_list_id);
    qb.push("AND id IN (");
    let mut separated = qb.separated(", ");
    for id in ids {
        separated.push_bind(id);
    }
    qb.push(")");
    qb.build().execute(&(*get_connection())).await?;

    Ok(())
}

pub async fn remove_todo_list_items(todo_list_id: u32, ids: &[u32]) -> Result<()> {
    if ids.is_empty() {
        return Ok(());
    }

    let mut qb = QueryBuilder::new("UPDATE todo_list_items SET deleted_at = datetime('now') WHERE deleted_at IS NULL AND todo_list_id = ");
    qb.push_bind(todo_list_id);
    qb.push(" AND id IN (");
    let mut separated = qb.separated(", ");
    for id in ids {
        separated.push_bind(id);
    }
    qb.push(")");

    qb.build().execute(&(*get_connection())).await?;

    Ok(())
}

fn get_connection() -> Arc<Pool<Sqlite>> {
    Arc::clone(&CONNECTION)
}
