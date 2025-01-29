use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct ApiKeyQueryParamDto {
    #[validate(required, length(min = 1))]
    #[serde(rename = "apiKey")]
    api_key: Option<String>,
}
