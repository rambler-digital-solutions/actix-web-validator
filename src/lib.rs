//! # actix-web-validator is crate for provide validation mechanism to actix-web with Validator crate
pub mod error;
pub use error::Error;

pub mod query;
pub use query::{QueryConfig, ValidatedQuery};

pub mod json;
pub use json::{JsonConfig, ValidatedJson};
