use sqlx::{
    prelude::FromRow,
    types::chrono::{DateTime, Utc},
};

#[derive(FromRow)]
pub struct ApiRequestRow {
    id: u32,
    api_key_id: u32,
    lines_of_code: u32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ApiRequestRow {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn api_key_id(&self) -> u32 {
        self.api_key_id
    }

    pub fn lines_of_code(&self) -> u32 {
        self.lines_of_code
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
