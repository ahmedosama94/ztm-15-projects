use serde::Serialize;
use validator::ValidationErrors;

use crate::data::{InternalServerErrorDto, UnauthorizedDto};

pub enum Response<T: Serialize> {
    Success(T),
    ValidationError(ValidationErrors),
    ServerError(InternalServerErrorDto),
    Unauthorized(UnauthorizedDto),
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
            Self::Unauthorized(err) => err.serialize(serializer),
        }
    }
}
