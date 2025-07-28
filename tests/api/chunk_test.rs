use bee_rs::api::chunk::{download, upload};
use bee_rs::api::bytes::{DownloadOptions, UploadOptions, UploadResult};
use wiremock::{matchers::{method, path_regex, header}, Mock, MockServer, ResponseTemplate};
use reqwest::header::HeaderValue;
use serde_json;

#[tokio::test]
async fn test_upload_chunk() {
    let expected_reference = "36b7efd913ca4cf880b8eeac5093fa27b0825906c600685b6abdd6566e6cfe8f";
    let expected_tag_uid = 789;
    let expected_history_address = "another_history_address_for_chunk";
    let mock_response_body = format!(r#"{{"reference": "{}"}}"#, expected_reference);

    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path_regex("/chunks"))
        .and(header("content-type", "application/octet-stream"))
        .and(header("swarm-postage-batch-id", "test_batch_id"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(serde_json::json!({ "reference": expected_reference }))
            .insert_header("swarm-tag", expected_tag_uid.to_string())
            .insert_header("swarm-act-history-address", expected_history_address))
        .mount(&mock_server)
        .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let postage_batch_id = "test_batch_id";
    let options = Some(UploadOptions {
        act: Some(true),
        act_history_address: Some("some_chunk_history_address".to_string()),
        pin: Some(true),
        encrypt: Some(true),
        tag: Some(expected_tag_uid),
        deferred: Some(false),
    });

    let result = upload(&client, base_url, data, postage_batch_id, options).await;

    assert!(result.is_ok());
    let upload_result = result.unwrap();
    assert_eq!(upload_result.reference, expected_reference);
    assert_eq!(upload_result.tag_uid, Some(expected_tag_uid));
    assert_eq!(upload_result.history_address, Some(expected_history_address.to_string()));
}

#[tokio::test]
async fn test_download_chunk() {
    let expected_data = vec![10, 20, 30, 40, 50];

    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex("/chunks/(.*)"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(expected_data.clone()))
        .mount(&mock_server)
        .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();
    let reference = "test_chunk_reference";
    let options = Some(DownloadOptions {
        redundancy_strategy: Some(1),
        fallback: Some(true),
        timeout_ms: Some(1000),
        act_publisher: Some("some_publisher".to_string()),
        act_history_address: Some("some_history_address".to_string()),
        act_timestamp: Some(12345),
    });

    let result = download(&client, base_url, reference, options).await;

    assert!(result.is_ok());
    let downloaded_data = result.unwrap();
    assert_eq!(downloaded_data, expected_data);
}
