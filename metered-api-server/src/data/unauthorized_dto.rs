use serde::Serialize;

#[derive(Serialize)]
pub struct UnauthorizedDto {
    error: String,
}

impl UnauthorizedDto {
    pub fn new() -> Self {
        Self {
            error: String::from("Unauthorized"),
        }
    }
}
