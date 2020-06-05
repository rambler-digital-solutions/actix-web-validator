use actix_service::Service;
use actix_web::{error, http::StatusCode, test, web, App, FromRequest, HttpResponse};
use actix_web_validator::{JsonConfig, ValidatedJson};
use serde_derive::{Deserialize, Serialize};
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, PartialEq, Validate, Serialize, Deserialize)]
struct JsonPayload {
    #[validate(url)]
    page_url: String,
    #[validate(range(min = 18, max = 28))]
    age: u8,
}

async fn test_handler(query: ValidatedJson<JsonPayload>) -> HttpResponse {
    dbg!(&query.into_inner());
    HttpResponse::Ok().finish()
}

#[actix_rt::test]
async fn test_json_validation() {
    let mut app = test::init_service(
        App::new().service(web::resource("/test").route(web::post().to(test_handler))),
    )
    .await;

    // Test 200 status
    let req = test::TestRequest::post()
        .uri("/test")
        .set_json(&JsonPayload {
            page_url: "https://my_page.com".to_owned(),
            age: 24,
        })
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Test 400 status
    let req = test::TestRequest::post()
        .uri("/test")
        .set_json(&JsonPayload {
            page_url: "invalid_url".to_owned(),
            age: 24,
        })
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_rt::test]
async fn test_custom_json_validation_error() {
    let mut app = test::init_service(
        App::new().service(
            web::resource("/test")
                .app_data(ValidatedJson::<JsonPayload>::configure(|cfg| {
                    cfg.error_handler(|err, _req| {
                        error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                            .into()
                    })
                }))
                .route(web::post().to(test_handler)),
        ),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/test")
        .set_json(&JsonPayload {
            page_url: "invalid".to_owned(),
            age: 24,
        })
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    dbg!(&resp);
    assert_eq!(resp.status(), StatusCode::CONFLICT);
}

#[actix_rt::test]
async fn test_validated_json_asref_deref() {
    let mut app = test::init_service(App::new().service(web::resource("/test").to(
        |payload: ValidatedJson<JsonPayload>| {
            assert_eq!(payload.age, 24);
            let reference = JsonPayload {
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
        .set_json(&JsonPayload {
            page_url: "https://my_page.com".to_owned(),
            age: 24,
        })
        .to_request();
    test::call_service(&mut app, req).await;
}

#[actix_rt::test]
async fn test_validated_json_into_inner() {
    let mut app = test::init_service(App::new().service(web::resource("/test").to(
        |payload: ValidatedJson<JsonPayload>| {
            let payload = payload.into_inner();
            assert_eq!(payload.age, 24);
            assert_eq!(payload.page_url, "https://my_page.com");
            HttpResponse::Ok().finish()
        },
    )))
    .await;

    let req = test::TestRequest::post()
        .uri("/test")
        .set_json(&JsonPayload {
            page_url: "https://my_page.com".to_owned(),
            age: 24,
        })
        .to_request();
    test::call_service(&mut app, req).await;
}

#[actix_rt::test]
async fn test_validated_json_limit() {
    let mut app = test::init_service(
        App::new()
            .app_data(JsonConfig::default().limit(1))
            .service(web::resource("/test").route(web::post().to(test_handler))),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/test")
        .set_json(&JsonPayload {
            page_url: "https://my_page.com".to_owned(),
            age: 24,
        })
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}
