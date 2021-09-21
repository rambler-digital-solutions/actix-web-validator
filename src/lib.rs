//! *actix-web-validator* is a crate for providing validation mechanism to actix-web with *validator* crate.
//!
//! The main idea of this crate is to add full validation support provided by validator derive traits
//! and provide maximum compatibility with base `actix_web::web::{Query, Json, Path}` structures.
//!
//! ## Example
//!
//! ```rust
//! use actix_web::{web, App};
//! use serde::Deserialize;
//! use actix_web_validator::Query;
//! use validator::Validate;
//!
//! #[derive(Debug, Deserialize)]
//! pub enum ResponseType {
//!     Token,
//!     Code
//! }
//!
//! #[derive(Deserialize, Validate)]
//! pub struct AuthRequest {
//!     #[validate(range(min = 1000, max = 9999))]
//!     id: u64,
//!     response_type: ResponseType,
//! }
//!
//! // Use `Query` extractor for query information (and destructure it within the signature).
//! // This handler gets called only if the request's query string contains a `id` and
//! // `response_type` fields.
//! // The correct request for this handler would be `/index.html?id=19&response_type=Code"`.
//! async fn index(info: Query<AuthRequest>) -> String {
//!     assert!(info.id >= 1000);
//!     format!("Authorization request for client with id={} and type={:?}!", info.id, info.response_type)
//! }
//!
//! fn main() {
//!     let app = App::new().service(
//!        web::resource("/index.html").route(web::get().to(index))); // <- use `Query` extractor
//! }
//! ```
pub mod error;
mod form;
mod json;
mod path;
mod qsquery;
mod query;
pub use error::Error;
pub use form::*;
pub use json::*;
pub use path::*;
pub use qsquery::*;
pub use query::*;

#[deprecated(
    note = "Please explicit use Validate trait or macro from `validator` crate.",
    since = "2.1.0"
)]
pub use validator::Validate;
