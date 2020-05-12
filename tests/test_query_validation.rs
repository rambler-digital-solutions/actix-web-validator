use actix_service::Service;
use actix_web::{error, http::StatusCode, test, web, App, HttpResponse};
use actix_web_validator::ValidatedQuery;
use serde_derive::Deserialize;
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Validate, Deserialize, PartialEq)]
struct QueryParams {
    #[validate(range(min = 8, max = 28))]
    id: u8,
}

fn test_handler(_query: ValidatedQuery<QueryParams>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[test]
fn test_query_validation() {
    let mut app = test::init_service(App::new().service(web::resource("/test").to(test_handler)));

    // Test 400 status
    let req = test::TestRequest::with_uri("/test?id=42").to_request();
    let resp = test::block_on(app.call(req)).unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    // Test 200 status
    let req = test::TestRequest::with_uri("/test?id=28").to_request();
    let resp = test::block_on(app.call(req)).unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[test]
fn test_custom_validation_error() {
    let mut app = test::init_service(
        App::new()
            .data(
                actix_web_validator::QueryConfig::default().error_handler(|err, _req| {
                    error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                        .into()
                }),
            )
            .service(web::resource("/test").to(test_handler)),
    );

    let req = test::TestRequest::with_uri("/test?id=42").to_request();
    let resp = test::block_on(app.call(req)).unwrap();
    assert_eq!(resp.status(), StatusCode::CONFLICT);
}

#[test]
fn test_deref_validated_query() {
    let mut app = test::init_service(App::new().service(web::resource("/test").to(
        |query: ValidatedQuery<QueryParams>| {
            assert_eq!(query.id, 28);
            HttpResponse::Ok().finish()
        },
    )));

    let req = test::TestRequest::with_uri("/test?id=28").to_request();
    test::block_on(app.call(req)).unwrap();
}

#[test]
fn test_query_implementation() {
    fn test_handler(query: ValidatedQuery<QueryParams>) -> HttpResponse {
        let reference = QueryParams { id: 28 };
        assert_eq!(query.as_ref(), &reference);
        assert_eq!(query.into_inner(), reference);
        HttpResponse::Ok().finish()
    }

    let mut app = test::init_service(App::new().service(web::resource("/test").to(test_handler)));
    let req = test::TestRequest::with_uri("/test?id=28").to_request();
    let resp = test::block_on(app.call(req)).unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}
