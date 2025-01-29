#[derive(sqlx::FromRow)]
pub struct ApiKeyRow {
    id: u32,
    email: String,
    api_key: String,
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
}
