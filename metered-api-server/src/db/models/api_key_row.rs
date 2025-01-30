use sqlx::{
    prelude::FromRow,
    types::chrono::{DateTime, Utc},
};

#[derive(FromRow)]
pub struct ApiKeyRow {
    id: u32,
    email: String,
    api_key: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ApiKeyRow {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
