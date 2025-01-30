use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Deserialize, Validate)]
pub struct CodeRunInputDto {
    #[validate(required, length(min = 1))]
    #[serde(rename = "apiKey")]
    api_key: Option<String>,

    #[validate(required)]
    code: Option<String>,

    #[validate(required, custom(function = "validate_language"))]
    language: Option<String>,
}

impl CodeRunInputDto {
    pub fn api_key(&self) -> &str {
        self.api_key.as_ref().unwrap()
    }

    pub fn code(&self) -> &str {
        self.code.as_ref().unwrap()
    }

    pub fn language(&self) -> &str {
        self.language.as_ref().unwrap()
    }
}

fn validate_language(code: &str) -> Result<(), ValidationError> {
    if ["java", "py", "cpp", "c", "go", "cs", "js"].contains(&code) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_language"))
    }
}
