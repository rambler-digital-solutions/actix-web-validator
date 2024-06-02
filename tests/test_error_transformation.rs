use actix_web_validator::{error::DeserializeErrors, Error};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrorsKind};

#[derive(Deserialize)]
#[allow(dead_code)]
struct Query {
    test: i32,
    value: i32,
}

#[test]
fn test_serde_urlencoded_error_transformation() {
    let error = serde_urlencoded::from_str::<Query>("test=42&value=[").map_err(Error::from);
    assert!(matches!(
        error,
        Err(Error::Deserialize(DeserializeErrors::DeserializeQuery(_)))
    ));
}

#[test]
fn test_serde_json_error_transformation() {
    let error =
        serde_json::from_str::<Query>("{\"test\": 42, \"value\": null}").map_err(Error::from);
    assert!(matches!(
        error,
        Err(Error::Deserialize(DeserializeErrors::DeserializeJson(_)))
    ));
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchParams {
    #[validate(nested)]
    page_params: PageParams,
    #[validate(url)]
    redirect_results: String,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PageParams {
    #[validate(range(min = 1))]
    page: u16,
    #[validate(range(min = 1, max = 100))]
    page_size: u8,
}

macro_rules! cast {
    ($target: expr, $pat: path) => {{
        if let $pat(a) = $target {
            a
        } else {
            panic!("mismatch variant when cast to {}", stringify!($pat));
        }
    }};
}

#[test]
fn test_flatten_error() {
    let params = serde_json::from_str::<SearchParams>(
        "{\"pageParams\":{\"page\":0,\"pageSize\":101},\"redirectResults\":\"invalid url\"}",
    )
    .map_err(crate::Error::from)
    .expect("invalid json");
    let validation = params.validate().unwrap_err();
    let errors = actix_web_validator::error::flatten_errors(&validation);
    assert_eq!(
        (
            &1u16,
            &cast!(
                &validation.errors().get("page_params").unwrap(),
                ValidationErrorsKind::Struct
            )
            .field_errors()
            .get("page_size")
            .unwrap()[0]
        ),
        errors
            .iter()
            .find(|(_, field, _)| field == "page_params.page_size")
            .map(|(indent, _, e)| (indent, *e))
            .unwrap()
    );
    assert_eq!(
        (
            &0u16,
            &validation.field_errors().get("redirect_results").unwrap()[0]
        ),
        errors
            .iter()
            .find(|(_, field, _)| field == "redirect_results")
            .map(|(indent, _, e)| (indent, *e))
            .unwrap()
    );
}
