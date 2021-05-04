//! Query extractor.
use crate::error::Error;
use std::ops::Deref;
use std::sync::Arc;
use std::{fmt, ops};

use actix_web::{FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use serde::de;
use validator::Validate;
use serde_qs::Config as QsConfig;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct QsQuery<T>(pub T);

impl<T> AsRef<T> for QsQuery<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> Deref for QsQuery<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> ops::DerefMut for QsQuery<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: fmt::Debug> fmt::Debug for QsQuery<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: fmt::Display> fmt::Display for QsQuery<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> QsQuery<T>
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
/// use actix_web_validator::{Query, Validate};
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
impl<T> FromRequest for QsQuery<T>
where
    T: de::DeserializeOwned + Validate,
{
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = QsQueryConfig;

    /// Builds Query struct from request and provides validation mechanism
    #[inline]
    fn from_request(
        req: &actix_web::web::HttpRequest,
        _: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let query_config = req.app_data::<QsQueryConfig>();

        let error_handler = query_config.map(|c| c.ehandler.clone())
            .unwrap_or(None);

        let default_qsconfig = QsConfig::default();
        let qsconfig = query_config
            .map(|c| &c.qs_config)
            .unwrap_or(&default_qsconfig);

        qsconfig
            .deserialize_str::<T>(req.query_string())
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
            .map(|value| ok(QsQuery(value)))
            .unwrap_or_else(|e| err(e))
    }
}

/// Query extractor configuration
///
/// ```rust
/// # #[macro_use] extern crate serde_derive;
/// # #[cfg(feature = "actix")]
/// # use actix_web;
/// # #[cfg(feature = "actix2")]
/// # use actix_web2 as actix_web;
/// use actix_web::{error, web, App, FromRequest, HttpResponse};
/// use serde_qs::actix::QsQuery;
/// use serde_qs::Config as QsConfig;
///
/// #[derive(Deserialize)]
/// struct Info {
///     username: String,
/// }
///
/// /// deserialize `Info` from request's querystring
/// fn index(info: QsQuery<Info>) -> HttpResponse {
///     format!("Welcome {}!", info.username).into()
/// }
///
/// fn main() {
///     let app = App::new().service(
///         web::resource("/index.html").app_data(
///             // change query extractor configuration
///             QsQuery::<Info>::configure(|cfg| {
///                 cfg.error_handler(|err, req| {  // <- create custom error response
///                     error::InternalError::from_response(
///                         err, HttpResponse::Conflict().finish()).into()
///                 })
///                 .qs_config(QsConfig::default())
///             }))
///             .route(web::post().to(index))
///     );
/// }
/// ```

pub struct QsQueryConfig {
    ehandler: Option<Arc<dyn Fn(Error, &HttpRequest) -> actix_web::Error + Send + Sync>>,
    qs_config: QsConfig,
}

impl QsQueryConfig {
    /// Set custom error handler
    pub fn error_handler<F>(mut self, f: F) -> Self
        where
            F: Fn(Error, &HttpRequest) -> actix_web::Error + Send + Sync + 'static,
    {
        self.ehandler = Some(Arc::new(f));
        self
    }

    /// Set custom serialization parameters
    pub fn qs_config(mut self, config: QsConfig) -> Self {
        self.qs_config = config;
        self
    }
}

impl Default for QsQueryConfig {
    fn default() -> Self {
        QsQueryConfig {
            ehandler: None,
            qs_config: QsConfig::default(),
        }
    }
}
