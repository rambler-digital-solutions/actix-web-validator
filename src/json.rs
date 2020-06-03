//! Json extractor/responder
use core::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;

use actix_http::Payload;
use actix_web::dev::JsonBody;
use actix_web::FromRequest;
use actix_web::HttpRequest;
use futures::Future;
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::error::Error;

/// ValidatedJson can be used for exstracting typed information and validation
/// from request's payload.
///
/// To extract and typed information from request's body, the type `T` must
/// implement the `Deserialize` trait from *serde* 
/// and `Validate` trait from *validator* crate.
///
/// [**JsonConfig**](struct.JsonConfig.html) allows to configure extraction
/// process.
///
/// ## Example
///
/// ```rust
/// use actix_web::{web, App};
/// use actix_web_validator::ValidatedJson;
/// use serde_derive::Deserialize;
/// use validator::Validate;
/// use validator_derive::Validate;
///
/// #[derive(Deserialize, Validate)]
/// struct Info {
///     #[validate(length(min = 3))]
///     username: String,
/// }
///
/// /// deserialize `Info` from request's body
/// fn index(info: ValidatedJson<Info>) -> String {
///     format!("Welcome {}!", info.username)
/// }
///
/// fn main() {
///     let app = App::new().service(
///        web::resource("/index.html").route(
///            web::post().to(index))
///     );
/// }
/// ```
#[derive(Debug)]
pub struct ValidatedJson<T>(pub T);

impl<T> ValidatedJson<T> {
    /// Deconstruct to an inner value
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> AsRef<T> for ValidatedJson<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> Deref for ValidatedJson<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

/// Json extractor. Allow to extract typed information from request's
/// payload and validate it.
///
/// To extract typed information from request's body, the type `T` must
/// implement the `Deserialize` trait from *serde*.
///
/// To validate payload, the type `T` must implement the `Validate` trait
/// from *validator* crate.
///
/// [**JsonConfig**](struct.JsonConfig.html) allows to configure extraction
/// process.
///
/// ## Example
///
/// ```rust
/// use actix_web::{web, App};
/// use actix_web_validator::ValidatedJson;
/// use serde_derive::Deserialize;
/// use validator::Validate;
/// use validator_derive::Validate;
///
/// #[derive(Deserialize, Validate)]
/// struct Info {
///     #[validate(length(min = 3))]
///     username: String,
/// }
///
/// /// deserialize `Info` from request's body
/// fn index(info: ValidatedJson<Info>) -> String {
///     format!("Welcome {}!", info.username)
/// }
///
/// fn main() {
///     let app = App::new().service(
///        web::resource("/index.html").route(
///            web::post().to(index))
///     );
/// }
/// ```
impl<T> FromRequest for ValidatedJson<T>
where
    T: DeserializeOwned + Validate + 'static,
{
    type Error = actix_web::Error;
    type Future = Box<dyn Future<Item = Self, Error = Self::Error>>;
    type Config = JsonConfig;

    #[inline]
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let req2 = req.clone();
        let (limit, err, ctype) = req
            .app_data::<Self::Config>()
            .map(|c| (c.limit, c.ehandler.clone(), c.content_type.clone()))
            .unwrap_or((32768, None, None));

        Box::new(
            JsonBody::new(req, payload, ctype)
                .limit(limit)
                .map_err(Error::from)
                .and_then(|value: T| {
                    value
                        .validate()
                        .map(|_| ValidatedJson(value))
                        .map_err(Error::from)
                })
                .map_err(move |e| {
                    log::debug!(
                        "Failed to deserialize Json from payload. \
                         Request path: {}",
                        req2.path()
                    );
                    if let Some(err) = err {
                        (*err)(e, &req2)
                    } else {
                        e.into()
                    }
                }),
        )
    }
}

/// Json extractor configuration
///
/// ```rust
/// use actix_web::{error, web, App, FromRequest, HttpResponse};
/// use serde_derive::Deserialize;
/// use actix_web_validator::{ValidatedJson, JsonConfig};
/// use validator::Validate;
/// use validator_derive::Validate;
///
/// #[derive(Deserialize, Validate)]
/// struct Info {
///     #[validate(length(min = 3))]
///     username: String,
/// }
///
/// /// deserialize `Info` from request's body, max payload size is 4kb
/// fn index(info: ValidatedJson<Info>) -> String {
///     format!("Welcome {}!", info.username)
/// }
///
/// fn main() {
///     let app = App::new().service(
///         web::resource("/index.html")
///             .data(
///                 // change json extractor configuration
///                 ValidatedJson::<Info>::configure(|cfg| {
///                     cfg.limit(4096)
///                        .content_type(|mime| {  // <- accept text/plain content type
///                            mime.type_() == mime::TEXT && mime.subtype() == mime::PLAIN
///                        })
///                        .error_handler(|err, req| {  // <- create custom error response
///                           error::InternalError::from_response(
///                               err, HttpResponse::Conflict().finish()).into()
///                        })
///             }))
///             .route(web::post().to(index))
///     );
/// }
/// ```
#[derive(Clone)]
pub struct JsonConfig {
    limit: usize,
    ehandler: Option<Arc<dyn Fn(Error, &HttpRequest) -> actix_web::Error + Send + Sync>>,
    content_type: Option<Arc<dyn Fn(mime::Mime) -> bool + Send + Sync>>,
}

impl JsonConfig {
    /// Change max size of payload. By default max size is 32Kb
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    /// Set custom error handler
    pub fn error_handler<F>(mut self, f: F) -> Self
    where
        F: Fn(Error, &HttpRequest) -> actix_web::Error + Send + Sync + 'static,
    {
        self.ehandler = Some(Arc::new(f));
        self
    }

    /// Set predicate for allowed content types
    pub fn content_type<F>(mut self, predicate: F) -> Self
    where
        F: Fn(mime::Mime) -> bool + Send + Sync + 'static,
    {
        self.content_type = Some(Arc::new(predicate));
        self
    }
}

impl Default for JsonConfig {
    fn default() -> Self {
        JsonConfig {
            limit: 32768,
            ehandler: None,
            content_type: None,
        }
    }
}
