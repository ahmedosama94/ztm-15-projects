use serde::Serialize;

#[derive(Serialize)]
pub struct RegistrationResponseDto {
    #[serde(rename = "apiKey")]
    api_key: String,
}

impl RegistrationResponseDto {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}
