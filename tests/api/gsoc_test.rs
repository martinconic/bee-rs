use bee_rs::api::gsoc::send;
use bee_rs::api::bytes::UploadOptions;
use wiremock::{matchers::{method, path_regex, header}, Mock, MockServer, ResponseTemplate};
use serde_json;

#[tokio::test]
async fn test_send_gsoc() {
    let expected_reference = "36b7efd913ca4cf880b8eeac5093fa27b0825906c600685b6abdd6566e6cfe8f";
    let postage_batch_id = "test_batch_id";
    let soc_data = vec![1, 2, 3, 4, 5];

    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path_regex("/chunks")) // gsoc send calls chunk upload
        .and(header("content-type", "application/octet-stream"))
        .and(header("swarm-postage-batch-id", postage_batch_id))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({ "reference": expected_reference })))
        .mount(&mock_server)
        .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();
    let options = Some(UploadOptions::default());

    let result = send(&client, base_url, soc_data, postage_batch_id, options).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_reference);
}
