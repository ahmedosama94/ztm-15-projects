use serde::Serialize;

#[derive(Serialize)]
pub struct InternalServerErrorDto {
    error: String,
}

impl InternalServerErrorDto {
    pub fn new() -> Self {
        Self {
            error: String::from("Internal server error"),
        }
    }
}
