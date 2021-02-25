//! Error declaration.
use actix_web;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Display, Debug)]
pub enum Error {
    #[display(fmt = "Query validate error: {}", _0)]
    Validate(validator::ValidationErrors),
    #[display(fmt = "Query validate error: {}", _0)]
    Deserialize(DeserializeErrors),
    #[display(fmt = "Payload error: {}", _0)]
    JsonPayloadError(actix_web::error::JsonPayloadError),
}

#[derive(Display, Debug)]
pub enum DeserializeErrors {
    #[display(fmt = "Query deserialize error: {}", _0)]
    DeserializeQuery(serde_urlencoded::de::Error),
    #[display(fmt = "Json deserialize error: {}", _0)]
    DeserializeJson(serde_json::error::Error),
    #[display(fmt = "Path deserialize error: {}", _0)]
    DeserializePath(serde::de::value::Error),
}

impl From<serde_json::error::Error> for Error {
    fn from(error: serde_json::error::Error) -> Self {
        Error::Deserialize(DeserializeErrors::DeserializeJson(error))
    }
}

impl From<serde_urlencoded::de::Error> for Error {
    fn from(error: serde_urlencoded::de::Error) -> Self {
        Error::Deserialize(DeserializeErrors::DeserializeQuery(error))
    }
}

impl From<actix_web::error::JsonPayloadError> for Error {
    fn from(error: actix_web::error::JsonPayloadError) -> Self {
        Error::JsonPayloadError(error)
    }
}

impl From<validator::ValidationErrors> for Error {
    fn from(error: validator::ValidationErrors) -> Self {
        Error::Validate(error)
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(StatusCode::BAD_REQUEST).body(format!("{{\"message\": {} }}", *self))
    }
}
