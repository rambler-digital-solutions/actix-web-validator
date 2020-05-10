//! # actix-web-validator is crate for provide validation mechanism to actix-web with Validator crate
pub mod error;
pub mod query;
pub mod json;
pub mod path;
pub use json::{JsonConfig, ValidatedJson};
pub use error::Error;
pub use query::{QueryConfig, ValidatedQuery};
pub use path::ValidatedPath;
