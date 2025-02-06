use sqlx::{
    prelude::FromRow,
    types::chrono::{DateTime, Utc},
};

#[derive(FromRow)]
pub struct TodoItemRow {
    id: u32,
    item: String,
    done_at: Option<DateTime<Utc>>,
    deleted_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TodoItemRow {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn item(&self) -> &str {
        &self.item
    }

    pub fn done_at(&self) -> Option<&DateTime<Utc>> {
        if let Some(done_at) = &self.done_at {
            Some(done_at)
        } else {
            None
        }
    }

    pub fn is_done(&self) -> bool {
        self.done_at.is_some()
    }

    pub fn deleted_at(&self) -> Option<&DateTime<Utc>> {
        if let Some(deleted_at) = &self.deleted_at {
            Some(deleted_at)
        } else {
            None
        }
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}
