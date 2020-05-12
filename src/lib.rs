//! # actix-web-validator is crate for provide validation mechanism to actix-web with Validator crate
pub mod error;
pub mod json;
pub mod path;
pub mod query;
pub use error::Error;
pub use json::{JsonConfig, ValidatedJson};
pub use path::{PathConfig, ValidatedPath};
pub use query::{QueryConfig, ValidatedQuery};
