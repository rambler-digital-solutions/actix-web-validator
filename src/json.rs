//! Json extractor/responder
use core::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;

use actix_http::{Payload, Response};
use actix_http::http::StatusCode;
use actix_web::dev::JsonBody;
use actix_web::FromRequest;
use actix_web::HttpRequest;
use actix_web::Responder;
use futures::Future;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;
use validator::Validate;

use crate::error::Error;

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

impl<T: Serialize> Responder for ValidatedJson<T> {
    type Error = Error;
    type Future = Result<Response, Error>;

    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        let body = match serde_json::to_string(&self.0) {
            Ok(body) => body,
            Err(e) => return Err(e.into()),
        };

        Ok(Response::build(StatusCode::OK)
            .content_type("application/json")
            .body(body))
    }
}

impl<T> FromRequest for ValidatedJson<T>
where
    T: DeserializeOwned + Validate + Debug + 'static,
{
    type Error = Error;
    type Future = Box<dyn Future<Item = Self, Error = Error>>;
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
                        e
                    }
                })
        )
    }
}

#[derive(Clone)]
pub struct JsonConfig {
    limit: usize,
    ehandler: Option<Arc<dyn Fn(Error, &HttpRequest) -> Error + Send + Sync>>,
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
        F: Fn(Error, &HttpRequest) -> Error + Send + Sync + 'static,
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
