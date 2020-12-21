use std::fmt;

use actix_web::{error, http::StatusCode, test, test::call_service, web, App, HttpResponse};
use actix_web_validator::Path;
use serde_derive::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, PartialEq)]
struct PathParams {
    #[validate(range(min = 8, max = 28))]
    id: u8,
}

impl fmt::Display for PathParams {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ id: {} }}", self.id)
    }
}

async fn test_handler(_query: Path<PathParams>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_rt::test]
async fn test_path_validation() {
    let mut app =
        test::init_service(App::new().service(web::resource("/test/{id}/").to(test_handler))).await;

    // Test 400 status
    let req = test::TestRequest::with_uri("/test/42/").to_request();
    let resp = call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    // Test 200 status
    let req = test::TestRequest::with_uri("/test/28/").to_request();
    let resp = call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn test_custom_path_validation_error() {
    let mut app = test::init_service(
        App::new()
            .app_data(
                actix_web_validator::PathConfig::default().error_handler(|err, _req| {
                    error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                        .into()
                }),
            )
            .service(web::resource("/test/{id}/").to(test_handler)),
    )
    .await;

    let req = test::TestRequest::with_uri("/test/42/").to_request();
    let resp = call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::CONFLICT);
}

#[actix_rt::test]
async fn test_deref_validated_path() {
    let mut app = test::init_service(App::new().service(web::resource("/test/{id}/").to(
        |query: Path<PathParams>| {
            assert_eq!(query.id, 28);
            HttpResponse::Ok().finish()
        },
    )))
    .await;

    let req = test::TestRequest::with_uri("/test/28/").to_request();
    call_service(&mut app, req).await;
}

#[actix_rt::test]
async fn test_path_implementation() {
    async fn test_handler(query: Path<PathParams>) -> HttpResponse {
        let reference = PathParams { id: 28 };
        assert_eq!(format!("{:?}", &reference), format!("{:?}", &query));
        assert_eq!(format!("{}", &reference), format!("{}", &query));
        assert_eq!(query.as_ref(), &reference);
        assert_eq!(query.into_inner(), reference);
        HttpResponse::Ok().finish()
    }

    let mut app =
        test::init_service(App::new().service(web::resource("/test/{id}/").to(test_handler))).await;
    let req = test::TestRequest::with_uri("/test/28/").to_request();
    let resp = call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}
