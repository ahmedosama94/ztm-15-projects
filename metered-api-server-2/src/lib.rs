use axum::extract::FromRef;
use serde::{de::Visitor, Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use validator::{Validate, ValidationError, ValidationErrors};

pub enum Response<T: Serialize> {
    Success(T),
    ValidationError(ValidationErrors),
    ServerError(InternalServerErrorDto),
}

impl<T: Serialize> Serialize for Response<T> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::ValidationError(err) => err.serialize(serializer),
            Self::ServerError(err) => err.serialize(serializer),
            Self::Success(payload) => payload.serialize(serializer),
        }
    }
}

#[derive(Validate)]
pub struct Registration {
    #[validate(email(message = "Incorrect email format"))]
    email: String,
}

impl Registration {
    pub fn email(&self) -> &str {
        &self.email
    }

    pub async fn validate_email_uniqueness(
        &self,
        db_pool: &Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match sqlx::query_as::<_, ApiKeyRow>("SELECT id FROM api_keys WHERE email = $1 LIMIT 1")
            .bind(self.email())
            .fetch_optional(db_pool)
            .await
        {
            // TODO: Investigate ColumnNotFound error reason
            Err(_) => Err(Self::create_validation_error().into()),
            Ok(option) => match option {
                Some(_) => Err(Self::create_validation_error().into()),
                None => Ok(()),
            },
        }
    }

    fn create_validation_error() -> ValidationError {
        ValidationError::new("email_already_in_use").with_message("Email already in use".into())
    }
}

impl<'de> Deserialize<'de> for Registration {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            Email,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`email`")
                    }

                    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        match v {
                            "email" => Ok(Field::Email),
                            _ => Err(serde::de::Error::unknown_field(v, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct RegistrationVisitor;

        impl<'de> Visitor<'de> for RegistrationVisitor {
            type Value = Registration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Registration")
            }

            fn visit_seq<A>(self, mut seq: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let email: String = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;

                let email = email.to_lowercase();

                Ok(Registration { email })
            }

            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut email = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Email => {
                            if email.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }

                            email = Some(map.next_value()?);
                        }
                    }
                }

                let email: String =
                    email.ok_or_else(|| serde::de::Error::missing_field("email"))?;
                let email = email.to_lowercase();

                Ok(Registration { email })
            }
        }

        const FIELDS: &[&str] = &["email"];
        deserializer.deserialize_struct("Registration", FIELDS, RegistrationVisitor)
    }
}

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

#[derive(sqlx::FromRow, Serialize)]
pub struct ApiKeyDto {
    #[serde(rename = "apiKey")]
    api_key: String,
}

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
