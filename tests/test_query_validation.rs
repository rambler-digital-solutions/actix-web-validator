use actix_web::{error, http::StatusCode, test, test::call_service, web, App, HttpResponse};
use actix_web_validator::{Error, Query};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, PartialEq)]
struct QueryParams {
    #[validate(range(min = 8, max = 28))]
    id: u8,
}

async fn test_handler(_query: Query<QueryParams>) -> HttpResponse {
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
                    assert!(matches!(err, Error::Validate(_)));
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
        |query: Query<QueryParams>| {
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
    async fn test_handler(query: Query<QueryParams>) -> HttpResponse {
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
