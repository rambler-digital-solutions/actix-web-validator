# actix-web-validator [![Latest Version]][crates.io] [![Documentation]][docs-rs] [![Coverage]][coveralls] [![Build Status]][travis]

[Latest Version]: https://img.shields.io/crates/v/actix-web-validator
[Documentation]: https://docs.rs/actix-web-validator/badge.svg
[docs-rs]: https://docs.rs/actix-web-validator/
[crates.io]: https://crates.io/crates/actix-web-validator
[Coverage]: https://coveralls.io/repos/github/rambler-digital-solutions/actix-web-validator/badge.svg?branch=master
[coveralls]: https://coveralls.io/github/rambler-digital-solutions/actix-web-validator?branch=master
[Build Status]: https://travis-ci.org/rambler-digital-solutions/actix-web-validator.svg?branch=master
[travis]: https://travis-ci.org/rambler-digital-solutions/actix-web-validator
[Validator crate]: https://crates.io/crates/validator

This crate is a Rust library for providing validation mechanism to actix-web with [Validator crate].

Installation
============

This crate works with Cargo and can be found on
[crates.io] with a `Cargo.toml` like:

```toml
[dependencies]
actix-web-validator = "6.0.0"
validator = { version = "0.16", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
```

## Supported extractors:
* `actix_web::web::Json`
* `actix_web::web::Query`
* `actix_web::web::Path`
* `actix_web::web::Form`
* `serde_qs::actix::QsQuery` 

### Supported `actix_web` versions:
* For actix-web-validator `0.*` supported version of actix-web is `1.*`
* For actix-web-validator `1.* ` supported version of actix-web is `2.*`
* For actix-web-validator `2.* ` supported version of actix-web is `3.*`
* For actix-web-validator `3+ ` supported version of actix-web is `4.*`

### Example:

```rust
use actix_web::{web, App};
use serde::Deserialize;
use actix_web_validator::Query;
use validator::Validate;

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
// This handler gets called only if the request's query string contains a `id` and
// `response_type` fields.
// The correct request for this handler would be `/index.html?id=1234&response_type=Code"`.
async fn index(info: Query<AuthRequest>) -> String {
    format!("Authorization request for client with id={} and type={:?}!", info.id, info.response_type)
}

fn main() {
    let app = App::new().service(
        web::resource("/index.html").route(web::get().to(index))); // <- use `Query` extractor
}
```

## License

actix-web-validator is licensed under MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)
