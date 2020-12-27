//! Query extractor.
use crate::error::Error;
use std::ops::Deref;
use std::sync::Arc;
use std::{fmt, ops};

use actix_web::{FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use serde::de;
use validator::Validate;

/// Query extractor configuration.
///
/// ## Example
///
/// ```rust
/// use actix_web::{error, web, App, FromRequest, HttpResponse};
/// use serde_derive::Deserialize;
/// use actix_web_validator::{Query, QueryConfig};
/// use validator::Validate;
///
/// #[derive(Deserialize, Validate)]
/// struct Info {
///     #[validate(length(min = 1))]
///     username: String,
/// }
///
/// /// deserialize `Info` from request's querystring
/// async fn index(info: Query<Info>) -> String {
///     format!("Welcome {}!", info.username)
/// }
///
/// fn main() {
///     let app = App::new().service(
///         web::resource("/index.html")
///             .app_data(
///                 // change query extractor configuration
///                 Query::<Info>::configure(|cfg| {
///                     cfg.error_handler(|err, req| {  // <- create custom error response
///                         error::InternalError::from_response(
///                             err, HttpResponse::Conflict().finish()).into()
///                     })
///                 }))
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
/// use actix_web_validator::Query;
/// use validator::Validate;
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
/// // The correct request for this handler would be `/index.html?id=1234&response_type=Code"`.
/// async fn index(info: Query<AuthRequest>) -> String {
///     format!("Authorization request for client with id={} and type={:?}!", info.id, info.response_type)
/// }
///
/// fn main() {
///     let app = App::new().service(
///         web::resource("/index.html").route(web::get().to(index))); // <- use `Query` extractor
/// }
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Query<T>(pub T);

#[deprecated(
    note = "Please, use actix_web_validator::Query instead.",
    since = "2.0.0"
)]
pub type ValidatedQuery<T> = Query<T>;

impl<T> AsRef<T> for Query<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> Deref for Query<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> ops::DerefMut for Query<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: fmt::Debug> fmt::Debug for Query<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: fmt::Display> fmt::Display for Query<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> Query<T>
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
/// use actix_web_validator::Query;
/// use validator::Validate;
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
/// async fn index(web::Query(info): web::Query<AuthRequest>) -> String {
///     format!("Authorization request for client with id={} and type={:?}!", info.id, info.response_type)
/// }
///
/// fn main() {
///     let app = App::new().service(
///        web::resource("/index.html").route(web::get().to(index))); // <- use `Query` extractor
/// }
/// ```
impl<T> FromRequest for Query<T>
where
    T: de::DeserializeOwned + Validate,
{
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = QueryConfig;

    /// Builds Query struct from request and provides validation mechanism
    #[inline]
    fn from_request(
        req: &actix_web::web::HttpRequest,
        _: &mut actix_web::dev::Payload,
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
            .map(|value| ok(Query(value)))
            .unwrap_or_else(|e| err(e))
    }
}
