use actix_web::{
    error,
    http::StatusCode,
    test,
    test::call_service,
    web::{self},
    App, HttpResponse,
};
use actix_web_validator::{Form, FormConfig};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, PartialEq, Validate, Serialize, Deserialize)]
struct FormData {
    #[validate(url)]
    page_url: String,
    #[validate(range(min = 18, max = 28))]
    age: u8,
}

async fn test_handler(query: Form<FormData>) -> HttpResponse {
    dbg!(&query.into_inner());
    HttpResponse::Ok().finish()
}

#[actix_web::test]
async fn test_form_validation() {
    let mut app = test::init_service(
        App::new().service(web::resource("/test").route(web::post().to(test_handler))),
    )
    .await;

    // Test 200 status
    let req = test::TestRequest::post()
        .uri("/test")
        .set_form(&FormData {
            page_url: "https://my_page.com".to_owned(),
            age: 24,
        })
        .to_request();
    let resp = call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Test 400 status
    let req = test::TestRequest::post()
        .uri("/test")
        .set_form(&FormData {
            page_url: "invalid_url".to_owned(),
            age: 24,
        })
        .to_request();
    let resp = call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn test_custom_form_validation_error() {
    let form_config = FormConfig::default().error_handler(|err, _req| {
        error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
    });
    let mut app = test::init_service(
        App::new().service(
            web::resource("/test")
                .app_data(form_config)
                .route(web::post().to(test_handler)),
        ),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/test")
        .set_form(&FormData {
            page_url: "invalid".to_owned(),
            age: 24,
        })
        .to_request();
    let resp = call_service(&mut app, req).await;
    dbg!(&resp);
    assert_eq!(resp.status(), StatusCode::CONFLICT);
}

#[actix_web::test]
async fn test_validated_form_asref_deref() {
    let mut app = test::init_service(App::new().service(web::resource("/test").to(
        |payload: Form<FormData>| async move {
            assert_eq!(payload.age, 24);
            let reference = FormData {
                page_url: "https://my_page.com".to_owned(),
                age: 24,
            };
            assert_eq!(payload.as_ref(), &reference);
            HttpResponse::Ok().finish()
        },
    )))
    .await;

    let req = test::TestRequest::post()
        .uri("/test")
        .set_form(&FormData {
            page_url: "https://my_page.com".to_owned(),
            age: 24,
        })
        .to_request();
    call_service(&mut app, req).await;
}

#[actix_web::test]
async fn test_validated_form_into_inner() {
    let mut app = test::init_service(App::new().service(web::resource("/test").to(
        |payload: Form<FormData>| async {
            let payload = payload.into_inner();
            assert_eq!(payload.age, 24);
            assert_eq!(payload.page_url, "https://my_page.com");
            HttpResponse::Ok().finish()
        },
    )))
    .await;

    let req = test::TestRequest::post()
        .uri("/test")
        .set_form(&FormData {
            page_url: "https://my_page.com".to_owned(),
            age: 24,
        })
        .to_request();
    call_service(&mut app, req).await;
}

#[actix_web::test]
async fn test_validated_form_limit() {
    let mut app = test::init_service(
        App::new()
            .app_data(FormConfig::default().limit(1))
            .service(web::resource("/test").route(web::post().to(test_handler))),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/test")
        .set_form(&FormData {
            page_url: "https://my_page.com".to_owned(),
            age: 24,
        })
        .to_request();
    let resp = call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}
