//! Path extractor.
use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

use actix_router::PathDeserializer;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use futures::future::{ready, Ready};
use serde::de::{Deserialize, DeserializeOwned};
use validator::Validate;

use crate::error::{DeserializeErrors, Error};

/// Extract typed information from the request's path.
///
/// ## Example
///
/// It is possible to extract path information to a specific type that
/// implements `Deserialize` trait from *serde* and `Validate` trait from *validator*.
///
/// ```rust
/// use actix_web::{web, App, Error};
/// use serde::Deserialize;
/// use actix_web_validator::Path;
/// use validator::Validate;
///
/// #[derive(Deserialize, Validate)]
/// struct Info {
///     #[validate(length(min = 1))]
///     username: String,
/// }
///
/// /// extract `Info` from a path using serde
/// async fn index(info: Path<Info>) -> Result<String, Error> {
///     Ok(format!("Welcome {}!", info.username))
/// }
///
/// fn main() {
///     let app = App::new().service(
///         web::resource("/{username}/index.html") // <- define path parameters
///              .route(web::get().to(index)) // <- use handler with Path` extractor
///     );
/// }
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Path<T> {
    inner: T,
}

#[deprecated(
    note = "Please, use actix_web_validator::Path instead.",
    since = "2.0.0"
)]
pub type ValidatedPath<T> = Path<T>;

impl<T> Path<T> {
    /// Deconstruct to an inner value
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> AsRef<T> for Path<T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T> Deref for Path<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T: fmt::Debug> fmt::Debug for Path<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T: fmt::Display> fmt::Display for Path<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

/// Extract typed information from the request's path.
///
/// ## Example
///
/// It is possible to extract path information to a specific type that
/// implements `Deserialize` trait from *serde* and `Validate` trait from *validator*.
///
/// ```rust
/// use actix_web::{web, App, Error};
/// use serde::Deserialize;
/// use actix_web_validator::Path;
/// use validator::Validate;
///
/// #[derive(Deserialize, Validate)]
/// struct Info {
///     #[validate(length(min = 1))]
///     username: String,
/// }
///
/// /// extract `Info` from a path using serde
/// async fn index(info: Path<Info>) -> Result<String, Error> {
///     Ok(format!("Welcome {}!", info.username))
/// }
///
/// fn main() {
///     let app = App::new().service(
///         web::resource("/{username}/index.html") // <- define path parameters
///              .route(web::get().to(index)) // <- use handler with Path` extractor
///     );
/// }
/// ```
impl<T> FromRequest for Path<T>
where
    T: DeserializeOwned + Validate,
{
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = PathConfig;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let error_handler = req
            .app_data::<Self::Config>()
            .map(|c| c.ehandler.clone())
            .unwrap_or(None);
        ready(
            Deserialize::deserialize(PathDeserializer::new(req.match_info()))
                .map(|inner: T| Path{ inner })
                .map_err(|error| Error::Deserialize(DeserializeErrors::DeserializePath(error)))
                .and_then(|value| {
                    value
                        .as_ref()
                        .validate()
                        .map(move |_| value)
                        .map_err(Error::Validate)
                })
                .map_err(move |e| {
                    log::debug!(
                        "Failed during Path extractor deserialization. \
                         Request path: {:?}",
                        req.path()
                    );
                    if let Some(error_handler) = error_handler {
                        (error_handler)(e, req)
                    } else {
                        actix_web::error::ErrorNotFound(e)
                    }
                }),
        )
    }
}

/// Path extractor configuration
///
/// ```rust
/// use actix_web_validator::{PathConfig, Path};
/// use actix_web::{error, web, App, FromRequest, HttpResponse};
/// use serde::Deserialize;
/// use validator::Validate;
///
/// #[derive(Deserialize, Debug)]
/// enum Folder {
///     #[serde(rename = "inbox")]
///     Inbox,
///     #[serde(rename = "outbox")]
///     Outbox,
/// }
///
/// #[derive(Deserialize, Debug, Validate)]
/// struct Filter {
///     folder: Folder,
///     #[validate(range(min = 1024))]
///     id: u64,
/// }
///
/// // deserialize `Info` from request's path
/// async fn index(folder: Path<Filter>) -> String {
///     format!("Selected folder: {:?}!", folder)
/// }
///
/// fn main() {
///     let app = App::new().service(
///         web::resource("/messages/{folder}")
///             .app_data(PathConfig::default().error_handler(|err, req| {
///                 error::InternalError::from_response(
///                     err,
///                     HttpResponse::Conflict().finish(),
///                 )
///                 .into()
///             }))
///             .route(web::post().to(index)),
///     );
/// }
/// ```
#[derive(Clone)]
pub struct PathConfig {
    ehandler: Option<Arc<dyn Fn(Error, &HttpRequest) -> actix_web::Error + Send + Sync>>,
}

impl PathConfig {
    /// Set custom error handler
    pub fn error_handler<F>(mut self, f: F) -> Self
    where
        F: Fn(Error, &HttpRequest) -> actix_web::Error + Send + Sync + 'static,
    {
        self.ehandler = Some(Arc::new(f));
        self
    }
}

impl Default for PathConfig {
    fn default() -> Self {
        Self { ehandler: None }
    }
}
