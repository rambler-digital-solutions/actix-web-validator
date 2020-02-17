//! # actix-web-validator is crate for provide validation mechanism to actix-web with Validator crate
pub mod query;
pub use query::{QueryConfig, Error, ValidatedQuery};
