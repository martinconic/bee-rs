use bee_rs::api::pinning::{get_all_pins, get_pin, pin, unpin};
use wiremock::{matchers::{method, path_regex}, Mock, MockServer, ResponseTemplate};
use serde_json;

#[tokio::test]
async fn test_pin() {
    let reference = "test_reference_pin";

    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path_regex("/pins/(.*)"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();

    let result = pin(&client, base_url, reference).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_unpin() {
    let reference = "test_reference_unpin";

    let mock_server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path_regex("/pins/(.*)"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();

    let result = unpin(&client, base_url, reference).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_pin() {
    let reference = "test_reference_get_pin";
    let expected_reference = "returned_reference_get_pin";

    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex("/pins/(.*)"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({ "reference": expected_reference })))
        .mount(&mock_server)
        .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();

    let result = get_pin(&client, base_url, reference).await;

    assert!(result.is_ok());
    let pin_info = result.unwrap();
    assert_eq!(pin_info.reference, expected_reference);
}

#[tokio::test]
async fn test_get_all_pins() {
    let expected_references = vec![
        "ref1".to_string(),
        "ref2".to_string(),
    ];

    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex("/pins"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({ "references": expected_references })))
        .mount(&mock_server)
        .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();

    let result = get_all_pins(&client, base_url).await;

    assert!(result.is_ok());
    let all_pins = result.unwrap();
    assert_eq!(all_pins, expected_references);
}
