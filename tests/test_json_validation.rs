use actix_service::Service;
use actix_web::{test, web, HttpResponse, http::StatusCode, App, error};
use validator::Validate;
use validator_derive::Validate;
use serde_derive::{Deserialize, Serialize};
use actix_web_validator::ValidatedJson;

#[derive(Debug, Validate, Serialize, Deserialize)]
struct JsonPayload {
   #[validate(url)]
   page_url: String,
   #[validate(range(min = 18, max = 28))]
   age: u8,
}

fn test_handler(_query: ValidatedJson<JsonPayload>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[test]
fn test_json_validation() {
    let mut app = test::init_service(
        App::new()
            .service(web::resource("/test").route(web::post().to(test_handler)))
    );

    // Test 200 status
    let req = test::TestRequest::post()
        .uri("/test")
        .set_json(&JsonPayload { page_url: "https://my_page.com".to_owned(), age: 24 })
        .to_request();
    let resp = test::block_on(app.call(req)).unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Test 400 status
    let req = test::TestRequest::post()
        .uri("/test")
        .set_json(&JsonPayload { page_url: "invalid_url".to_owned(), age: 24 })
        .to_request();
    let resp = test::block_on(app.call(req)).unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[test]
fn test_custom_validation_error() {
    let mut app = test::init_service(
        App::new()
            .service(
                web::resource("/test")
                    .data(
                        actix_web_validator::JsonConfig::default()
                            .error_handler(|err, _req| {
                                error::InternalError::from_response(
                                    err, HttpResponse::Conflict().finish()
                                ).into()
                            })
                    )
                    .route(web::post().to(test_handler))
            )
    );

    let req = test::TestRequest::post()
        .uri("/test")
        .set_json(&JsonPayload { page_url: "invalid".to_owned(), age: 24 })
        .to_request();
    let resp = test::block_on(app.call(req)).unwrap();
    assert_eq!(resp.status(), StatusCode::CONFLICT);
}
