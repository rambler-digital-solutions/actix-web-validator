use actix_service::Service;
use actix_web::{test, web, HttpResponse, http::StatusCode, App, error};
use actix_web_validator::ValidatedPath;
use validator::Validate;
use validator_derive::Validate;
use serde_derive::Deserialize;

#[derive(Debug, Validate, Deserialize, PartialEq)]
struct PathParams {
    #[validate(range(min = 8, max = 28))]
    id: u8,
}

fn test_handler(_query: ValidatedPath<PathParams>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[test]
fn test_path_validation() {
    let mut app = test::init_service(
        App::new()
            .service(web::resource("/test/{id}/").to(test_handler))
    );

    // Test 400 status
    let req = test::TestRequest::with_uri("/test/42/").to_request();
    let resp = test::block_on(app.call(req)).unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    // Test 200 status
    let req = test::TestRequest::with_uri("/test/28/").to_request();
    let resp = test::block_on(app.call(req)).unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[test]
fn test_custom_validation_error() {
    let mut app = test::init_service(
        App::new()
            .data(actix_web_validator::PathConfig::default()
                .error_handler(|err, _req| {
                    error::InternalError::from_response(
                        err, HttpResponse::Conflict().finish()).into()
                }))
            .service(web::resource("/test/{id}/").to(test_handler))
    );

    let req = test::TestRequest::with_uri("/test/42/").to_request();
    let resp = test::block_on(app.call(req)).unwrap();
    assert_eq!(resp.status(), StatusCode::CONFLICT);
}

#[test]
fn test_deref_validated_path() {
    let mut app = test::init_service(
        App::new()
            .service(web::resource("/test/{id}/")
                .to(|query: ValidatedPath<PathParams>| {
                    assert_eq!(query.id, 28);
                    HttpResponse::Ok().finish()
                })
            ));

    let req = test::TestRequest::with_uri("/test/28/").to_request();
    test::block_on(app.call(req)).unwrap();
}

#[test]
fn test_query_implementation() {
    fn test_handler(query: ValidatedPath<PathParams>) -> HttpResponse {
        let reference = PathParams { id: 28 };
        assert_eq!(query.as_ref(), &reference);
        assert_eq!(query.into_inner(), reference);
        HttpResponse::Ok().finish()
    }

    let mut app = test::init_service(
        App::new()
            .service(web::resource("/test/{id}/").to(test_handler))
    );
    let req = test::TestRequest::with_uri("/test/28/").to_request();
    let resp = test::block_on(app.call(req)).unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}
