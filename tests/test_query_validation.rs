use actix_service::Service;
use actix_web::{test, web, HttpResponse, http::StatusCode, App};
use actix_web_validator::ValidatedQuery;
use validator::Validate;
use validator_derive::Validate;
use serde_derive::Deserialize;

#[derive(Debug, Validate, Deserialize)]
struct QueryParams {
    #[validate(range(min = 8, max = 28))]
    id: u8,
}

fn test_handler(_query: ValidatedQuery<QueryParams>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[test]
fn test_query_validation() {
    let mut app = test::init_service(
        App::new()
            .service(web::resource("/test").to(test_handler))
    );

    // Test 400 status
    let req = test::TestRequest::with_uri("/test?id=42").to_request();
    let resp = test::block_on(app.call(req)).unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    // Test 200 status
    let req = test::TestRequest::with_uri("/test?id=28").to_request();
    let resp = test::block_on(app.call(req)).unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}
