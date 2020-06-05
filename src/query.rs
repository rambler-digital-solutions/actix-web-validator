//! Query extractor.
use crate::error::Error;
use std::sync::Arc;
use std::{fmt, ops};

use actix_web::{FromRequest, HttpRequest};
use serde::de;
use std::ops::Deref;
use validator::Validate;

/// Query extractor configuration.
///
/// ## Example
///
/// ```rust
/// use actix_web::{error, web, App, FromRequest, HttpResponse};
/// use serde_derive::Deserialize;
/// use actix_web_validator::{ValidatedQuery, QueryConfig};
/// use validator::Validate;
/// use validator_derive::Validate;
///
/// #[derive(Deserialize, Validate)]
/// struct Info {
///     #[validate(length(min = 1))]
///     username: String,
/// }
///
/// /// deserialize `Info` from request's querystring
/// fn index(info: ValidatedQuery<Info>) -> String {
///     format!("Welcome {}!", info.username)
/// }
///
/// fn main() {
///     let app = App::new().service(
///         web::resource("/index.html").data(
///             // change query extractor configuration
///             ValidatedQuery::<Info>::configure(|cfg| {
///                 cfg.error_handler(|err, req| {  // <- create custom error response
///                     error::InternalError::from_response(
///                         err, HttpResponse::Conflict().finish()).into()
///                 })
///             }))
///             .route(web::post().to(index))
///     );
/// }
/// ```
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

/// Extract and validate typed information from the request's query.
///
/// For query decoding uses *serde_urlencoded* crate
/// [**QueryConfig**](struct.QueryConfig.html) allows to configure extraction process.
///
/// ## Example
///
/// ```rust
/// use actix_web::{web, App};
/// use serde_derive::Deserialize;
/// use actix_web_validator::ValidatedQuery;
/// use validator::Validate;
/// use validator_derive::Validate;
///
/// #[derive(Debug, Deserialize)]
/// pub enum ResponseType {
///     Token,
///     Code
/// }
///
/// #[derive(Deserialize, Validate)]
/// pub struct AuthRequest {
///     #[validate(range(min = 1000, max = 9999))]
///     id: u64,
///     response_type: ResponseType,
/// }
///
/// // Use `Query` extractor for query information (and destructure it within the signature).
/// // This handler gets called only if the request's query string contains a `id` and
/// // `response_type` fields.
/// // The correct request for this handler would be `/index.html?id=19&response_type=Code"`.
/// fn index(web::Query(info): web::Query<AuthRequest>) -> String {
///     format!("Authorization request for client with id={} and type={:?}!", info.id, info.response_type)
/// }
///
/// fn main() {
///     let app = App::new().service(
///        web::resource("/index.html").route(web::get().to(index))); // <- use `Query` extractor
/// }
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct ValidatedQuery<T>(pub T);

impl<T> AsRef<T> for ValidatedQuery<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> Deref for ValidatedQuery<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> ops::DerefMut for ValidatedQuery<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: fmt::Debug> fmt::Debug for ValidatedQuery<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: fmt::Display> fmt::Display for ValidatedQuery<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> ValidatedQuery<T>
where
    T: Validate,
{
    /// Deconstruct to an inner value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

/// Extract typed information from the request's query.
///
/// ## Example
///
/// ```rust
/// use actix_web::{web, App};
/// use serde_derive::Deserialize;
/// use actix_web_validator::ValidatedQuery;
/// use validator::Validate;
/// use validator_derive::Validate;
///
/// #[derive(Debug, Deserialize)]
/// pub enum ResponseType {
///     Token,
///     Code
/// }
///
/// #[derive(Deserialize, Validate)]
/// pub struct AuthRequest {
///     #[validate(range(min = 1000, max = 9999))]
///     id: u64,
///     response_type: ResponseType,
/// }
///
/// // Use `Query` extractor for query information (and destructure it within the signature).
/// // This handler gets called only if the request's query string contains a `id` and
/// // `response_type` fields.
/// // The correct request for this handler would be `/index.html?id=19&response_type=Code"`.
/// fn index(web::Query(info): web::Query<AuthRequest>) -> String {
///     format!("Authorization request for client with id={} and type={:?}!", info.id, info.response_type)
/// }
///
/// fn main() {
///     let app = App::new().service(
///        web::resource("/index.html").route(web::get().to(index))); // <- use `Query` extractor
/// }
/// ```
impl<T> FromRequest for ValidatedQuery<T>
where
    T: de::DeserializeOwned + Validate,
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
            .map_err(Error::from)
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
