//! Error declaration.
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Display, Debug)]
pub enum Error {
    #[display(fmt = "Validation error: {}", _0)]
    Validate(validator::ValidationErrors),
    #[display(fmt = "{}", _0)]
    Deserialize(DeserializeErrors),
    #[display(fmt = "Payload error: {}", _0)]
    JsonPayloadError(actix_web::error::JsonPayloadError),
    #[display(fmt = "Url encoded error: {}", _0)]
    UrlEncodedError(actix_web::error::UrlencodedError),
    #[display(fmt = "Query error: {}", _0)]
    QsError(serde_qs::Error),
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

impl From<serde_qs::Error> for Error {
    fn from(error: serde_qs::Error) -> Self {
        Error::QsError(error)
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

impl From<actix_web::error::UrlencodedError> for Error {
    fn from(error: actix_web::error::UrlencodedError) -> Self {
        Error::UrlEncodedError(error)
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(StatusCode::BAD_REQUEST).body(match self {
            Self::Validate(e) => {
                format!("Validation errors in fields:\n{}", format_errors(e, None))
            }

            _ => format!("{}", *self),
        })
    }
}

#[inline(always)]
fn format_errors(errors: &validator::ValidationErrors, indent: Option<usize>) -> String {
    let indent = indent.unwrap_or(1);
    errors
        .errors()
        .iter()
        .filter_map(|(field, err)| match err {
            validator::ValidationErrorsKind::Field(errors) => {
                let error = errors.first().map(|err| format!("{}", err.code));
                Some(format!(
                    "{}{}: {}",
                    "\t".repeat(indent),
                    field,
                    error.unwrap_or_default()
                ))
            }
            validator::ValidationErrorsKind::Struct(errors) => Some(format!(
                "{}{}:\n{}",
                "\t".repeat(indent),
                field,
                format_errors(errors, Some(indent + 1))
            )),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

mod test {
    use serde::{Deserialize, Serialize};
    use validator::Validate;

    #[derive(Serialize, Deserialize, Validate, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct SearchParams {
        #[validate]
        page_params: PageParams,
        #[validate(url)]
        redirect_results: String,
    }

    #[derive(Serialize, Deserialize, Validate, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct PageParams {
        #[validate(range(min = 1))]
        page: u16,
        #[validate(range(min = 1, max = 100))]
        page_size: u8,
    }

    #[test]
    fn test_format_error() {
        let params = serde_json::from_str::<SearchParams>(
            "{\"pageParams\":{\"page\":0,\"pageSize\":101},\"redirectResults\":\"invalid url\"}",
        )
        .map_err(crate::Error::from)
        .expect("invalid json");
        let validation = params.validate();
        let msg = crate::error::format_errors(&validation.unwrap_err(), None);
        assert!(msg.contains("page_params"));
        assert!(msg.contains("page_size"));
        assert!(msg.contains("range"));
        assert!(msg.contains("page"));
        assert!(msg.contains("redirect_results"));
        assert!(msg.contains("url"));
    }
}
