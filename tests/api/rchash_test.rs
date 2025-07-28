use bee_rs::api::rchash::rchash;
use wiremock::{matchers::{method, path_regex}, Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_rchash() {
    let depth = 10;
    let anchor1 = "anchor1_value";
    let anchor2 = "anchor2_value";
    let expected_duration = 123.45;

    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(&format!("/rchash/{}/{}/{}", depth, anchor1, anchor2)))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({ "durationSeconds": expected_duration })))
        .mount(&mock_server)
        .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();

    let result = rchash(&client, base_url, depth, anchor1, anchor2).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_duration);
}
