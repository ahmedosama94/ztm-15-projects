use sqlx::{
    prelude::FromRow,
    types::chrono::{DateTime, Utc},
};

#[derive(FromRow)]
pub struct TodoItemRow {
    pub id: u32,
    pub item: String,
    pub done_at: DateTime<Utc>,
    pub deleted_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
