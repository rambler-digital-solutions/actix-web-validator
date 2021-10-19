use actix_web::{dev::UrlEncoded, FromRequest, HttpRequest, dev::Payload};
use futures::future::LocalBoxFuture;
use futures::FutureExt;
use serde::de::DeserializeOwned;
use std::{ops::Deref, rc::Rc};
use validator::Validate;

use crate::Error;

/// Form can be used for extracting typed information and validation
/// from request's form data.
///
/// To extract and typed information from request's form data, the type `T` must
/// implement the `Deserialize` trait from *serde*
/// and `Validate` trait from *validator* crate.
///
/// [**FormConfig**](struct.FormConfig.html) allows to configure extraction
/// process.
///
/// ## Example
///
/// ```rust
/// use actix_web::{web, App};
/// use actix_web_validator::Form;
/// use serde::Deserialize;
/// use validator::Validate;
///
/// #[derive(Deserialize, Validate)]
/// struct Info {
///     #[validate(length(min = 3))]
///     username: String,
/// }
///
/// /// deserialize `Info` from request's form data
/// async fn index(info: Form<Info>) -> String {
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
pub struct Form<T>(pub T);

impl<T> Form<T> {
    /// Deconstruct to an inner value
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> AsRef<T> for Form<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> Deref for Form<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

/// Form data helper (`application/x-www-form-urlencoded`). Allow to extract typed information from request's
/// payload and validate it.
///
/// To extract typed information from request's body, the type `T` must
/// implement the `Deserialize` trait from *serde*.
///
/// To validate payload, the type `T` must implement the `Validate` trait
/// from *validator* crate.
///
/// [**FormConfig**](struct.FormConfig.html) allows to configure extraction
/// process.
///
/// ## Example
///
/// ```rust
/// use actix_web::{web, App};
/// use actix_web_validator::Form;
/// use serde::Deserialize;
/// use validator::Validate;
///
/// #[derive(Deserialize, Validate)]
/// struct Info {
///     #[validate(length(min = 3))]
///     username: String,
/// }
///
/// /// deserialize `Info` from request's form data
/// async fn index(info: Form<Info>) -> String {
///     format!("Welcome {}!", info.username)
/// }
/// ```
impl<T> FromRequest for Form<T>
where
    T: DeserializeOwned + Validate + 'static,
{
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;
    type Config = FormConfig;

    #[inline]
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let req2 = req.clone();
        let (limit, error_handler) = req
            .app_data::<Self::Config>()
            .map(|c| (c.limit, c.ehandler.clone()))
            .unwrap_or((16_384, None));

        UrlEncoded::new(req, payload)
            .limit(limit)
            .map(|res: Result<T, _>| match res {
                Ok(data) => data.validate().map(|_| Form(data)).map_err(Error::from),
                Err(e) => Err(Error::from(e)),
            })
            .map(move |res| match res {
                Err(e) => {
                    if let Some(err) = error_handler {
                        Err((*err)(e, &req2))
                    } else {
                        Err(e.into())
                    }
                }
                Ok(item) => Ok(item),
            })
            .boxed_local()
    }
}

/// Form extractor configuration
///
/// ```rust
/// use actix_web::{error, web, App, FromRequest, HttpResponse};
/// use serde::Deserialize;
/// use actix_web_validator::{Form, FormConfig};
/// use validator::Validate;
///
/// #[derive(Deserialize, Validate)]
/// struct Info {
///     #[validate(length(min = 3))]
///     username: String,
/// }
///
/// /// deserialize `Info` from request's form data, max payload size is 4kb
/// async fn index(info: Form<Info>) -> String {
///     format!("Welcome {}!", info.username)
/// }
///
/// fn main() {
///     let app = App::new().service(
///         web::resource("/index.html")
///             .app_data(
///                 // change form data extractor configuration
///                 Form::<Info>::configure(|cfg| {
///                     cfg.limit(4096)
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
pub struct FormConfig {
    limit: usize,
    ehandler: Option<Rc<dyn Fn(Error, &HttpRequest) -> actix_web::Error>>,
}

impl FormConfig {
    /// Change max size of payload. By default max size is 16Kb
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    /// Set custom error handler
    pub fn error_handler<F>(mut self, f: F) -> Self
    where
        F: Fn(Error, &HttpRequest) -> actix_web::Error + 'static,
    {
        self.ehandler = Some(Rc::new(f));
        self
    }
}

impl Default for FormConfig {
    fn default() -> Self {
        Self {
            limit: 16_384,
            ehandler: None,
        }
    }
}
