use actix_web_validator::{error::DeserializeErrors, Error};
use serde::Deserialize;

#[derive(Deserialize)]
struct Query {
    pub test: i32,
    pub value: i32,
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
