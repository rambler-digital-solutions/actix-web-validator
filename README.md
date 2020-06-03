# actix-web-validator [![Latest Version]][crates.io] [![Documentation]][docs-rs] [![Coverage]][coveralls] [![Build Status]][travis]

[Latest Version]: https://img.shields.io/crates/v/actix-web-validator
[Documentation]: https://docs.rs/actix-web-validator/badge.svg
[docs-rs]: https://docs.rs/actix-web-validator/
[crates.io]: https://crates.io/crates/actix-web-validator
[Coverage]: https://coveralls.io/repos/github/rambler-digital-solutions/actix-web-validator/badge.svg?branch=master
[coveralls]: https://coveralls.io/github/rambler-digital-solutions/actix-web-validator?branch=master
[Build Status]: https://travis-ci.org/rambler-digital-solutions/actix-web-validator.svg?branch=master
[travis]: https://travis-ci.org/rambler-digital-solutions/actix-web-validator


This crate is a Rust library for providing validation mechanism to actix-web with Validator crate


Installation
============

This crate works with Cargo and can be found on
[crates.io] with a `Cargo.toml` like:

```toml
[dependencies]
actix-web-validator = "0.1"
```

## Supported `actix_web::web` extractors:
* `web::Json`
* `web::Query`
* `web::Path`

### Supported `actix_web` version is `1.*`

### Example:

```rust
use actix_web::{web, App};
use serde_derive::Deserialize;
use actix_web_validator::ValidatedQuery;
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Deserialize)]
pub enum ResponseType {
    Token,
    Code
}

#[derive(Deserialize, Validate)]
pub struct AuthRequest {
    #[validate(range(min = 1000, max = 9999))]
    id: u64,
    response_type: ResponseType,
}

// Use `Query` extractor for query information (and destructure it within the signature).
// This handler gets called only if the request's query string contains a `username` field.
// The correct request for this handler would be `/index.html?id=19&response_type=Code"`.
fn index(web::Query(info): web::Query<AuthRequest>) -> String {
    format!("Auth request for client with id={} and type={:?}!", info.id, info.response_type)
}

fn main() {
    let app = App::new().service(
        web::resource("/index.html").route(web::get().to(index))); // <- use `Query` extractor
}
```

## License

actix-web-validator is licensed under MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)
