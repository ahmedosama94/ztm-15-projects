use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CodeRunResponseDto {
    error: String,
    info: Option<String>,
    language: String,
    output: String,
}
