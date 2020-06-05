use actix_web::{error, http::StatusCode, test, web, App, HttpResponse, test::call_service};
use actix_web_validator::ValidatedQuery;
use serde_derive::Deserialize;
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Validate, Deserialize, PartialEq)]
struct QueryParams {
    #[validate(range(min = 8, max = 28))]
    id: u8,
}

async fn test_handler(_query: ValidatedQuery<QueryParams>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_rt::test]
async fn test_query_validation() {
    let mut app =
        test::init_service(App::new().service(web::resource("/test").to(test_handler))).await;

    // Test 400 status
    let req = test::TestRequest::with_uri("/test?id=42").to_request();
    let resp = call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    // Test 200 status
    let req = test::TestRequest::with_uri("/test?id=28").to_request();
    let resp = call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn test_custom_query_validation_error() {
    let mut app = test::init_service(
        App::new()
            .app_data(
                actix_web_validator::QueryConfig::default().error_handler(|err, _req| {
                    error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                        .into()
                }),
            )
            .service(web::resource("/test").to(test_handler)),
    )
    .await;

    let req = test::TestRequest::with_uri("/test?id=42").to_request();
    let resp = call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::CONFLICT);
}

#[actix_rt::test]
async fn test_deref_validated_query() {
    let mut app = test::init_service(App::new().service(web::resource("/test").to(
        |query: ValidatedQuery<QueryParams>| {
            assert_eq!(query.id, 28);
            HttpResponse::Ok().finish()
        },
    )))
    .await;

    let req = test::TestRequest::with_uri("/test?id=28").to_request();
    call_service(&mut app, req).await;
}

#[actix_rt::test]
async fn test_query_implementation() {
    async fn test_handler(query: ValidatedQuery<QueryParams>) -> HttpResponse {
        let reference = QueryParams { id: 28 };
        assert_eq!(query.as_ref(), &reference);
        assert_eq!(query.into_inner(), reference);
        HttpResponse::Ok().finish()
    }

    let mut app =
        test::init_service(App::new().service(web::resource("/test").to(test_handler))).await;
    let req = test::TestRequest::with_uri("/test?id=28").to_request();
    let resp = call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}
