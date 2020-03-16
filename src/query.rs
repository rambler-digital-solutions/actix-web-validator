use core::fmt::Debug;
use std::sync::Arc;

use actix_http::error::ResponseError;
use actix_web::http::StatusCode;
use actix_web::{FromRequest, HttpRequest, HttpResponse};
use derive_more::Display;
use serde::de;
use validator::Validate;

#[derive(Clone)]
pub struct QueryConfig {
    pub ehandler: Option<Arc<dyn Fn(Error, &HttpRequest) -> actix_web::Error + Send + Sync>>,
}

impl QueryConfig {
    /// Set custom error handler
    pub fn error_handler<F>(mut self, f: F) -> Self
    where
        F: Fn(Error, &HttpRequest) -> actix_web::Error + Send + Sync + 'static,
    {
        self.ehandler = Some(Arc::new(f));
        self
    }
}

impl Default for QueryConfig {
    fn default() -> Self {
        QueryConfig { ehandler: None }
    }
}

#[derive(Display, Debug, Clone)]
pub enum Error {
    #[display(fmt = "Query validate error: {}", _0)]
    Validate(validator::ValidationErrors),
    #[display(fmt = "Query deserialize error: {}", _0)]
    Deserialize(serde_urlencoded::de::Error),
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::new(StatusCode::BAD_REQUEST)
    }
}

/// Replaces actix_web Query struct to get T struct validated with Validator crate
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct ValidatedQuery<T>(pub T);

impl<T> AsRef<T> for ValidatedQuery<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> ValidatedQuery<T>
where
    T: Validate,
{
    /// Deconstruct to a inner value
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> FromRequest for ValidatedQuery<T>
where
    T: de::DeserializeOwned + Validate + Debug,
{
    type Error = actix_web::Error;
    type Future = Result<Self, actix_web::Error>;
    type Config = QueryConfig;

    /// Builds Query struct from request and provides validation mechanism
    #[inline]
    fn from_request(
        req: &actix_web::web::HttpRequest,
        _: &mut actix_http::Payload,
    ) -> Self::Future {
        let error_handler = req
            .app_data::<Self::Config>()
            .map(|c| c.ehandler.clone())
            .unwrap_or(None);

        serde_urlencoded::from_str::<T>(req.query_string())
            .map_err(Error::Deserialize)
            .and_then(|value| {
                value
                    .validate()
                    .map(move |_| value)
                    .map_err(Error::Validate)
            })
            .map(ValidatedQuery)
            .map_err(move |e| {
                log::debug!(
                    "Failed during Query extractor validation. \
                     Request path: {:?}",
                    req.path()
                );
                if let Some(error_handler) = error_handler {
                    (error_handler)(e, req)
                } else {
                    e.into()
                }
            })
    }
}
