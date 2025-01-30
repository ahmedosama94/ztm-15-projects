use axum::extract::FromRef;
use sqlx::{Pool, Sqlite};

#[derive(Clone)]
pub struct ApiState {
    db_pool: Pool<Sqlite>,
}

impl ApiState {
    pub fn new(db_pool: Pool<Sqlite>) -> Self {
        Self { db_pool }
    }

    pub fn db_pool(&self) -> &Pool<Sqlite> {
        &self.db_pool
    }
}

impl FromRef<ApiState> for Pool<Sqlite> {
    fn from_ref(api_state: &ApiState) -> Self {
        api_state.db_pool().clone()
    }
}
