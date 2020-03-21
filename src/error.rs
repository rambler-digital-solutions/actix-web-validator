use actix_web;
use actix_web::{HttpResponse, ResponseError};
use actix_web::http::StatusCode;
use derive_more::Display;

#[derive(Display, Debug)]
pub enum Error {
    #[display(fmt = "Query validate error: {}", _0)]
    Validate(validator::ValidationErrors),
    #[display(fmt = "Query deserialize error: {}", _0)]
    Deserialize(serde_urlencoded::de::Error),
    #[display(fmt = "Json deserialize error: {}", _0)]
    DeserializeJson(serde_json::error::Error),
    #[display(fmt = "Query deserialize error: {}", _0)]
    JsonPayloadError(actix_web::error::JsonPayloadError),
}

impl From<serde_json::error::Error> for Error {
    fn from(error: serde_json::error::Error) -> Self {
        Error::DeserializeJson(error)
    }
}

impl From<actix_web::error::JsonPayloadError> for Error {
    fn from(error: actix_web::error::JsonPayloadError) -> Self {
        Error::JsonPayloadError(error)
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::new(StatusCode::BAD_REQUEST)
    }
}