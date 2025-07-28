use bee_rs::api::bytes::{download, head, upload, DownloadOptions, RedundantUploadOptions, UploadOptions, UploadResult, ReferenceInformation};
use wiremock::{matchers::{method, path_regex, header}, Mock, MockServer, ResponseTemplate};
use serde_json;
use reqwest::header::HeaderValue;

#[tokio::test]
async fn test_upload() {
    let expected_reference = "36b7efd913ca4cf880b8eeac5093fa27b0825906c600685b6abdd6566e6cfe8f";
    let expected_tag_uid = 123;
    let expected_history_address = "another_history_address";
    let mock_response_body = format!(r#"{{"reference": "{}"}}"#, expected_reference);

    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path_regex("/bytes"))
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
    let data = vec![1, 2, 3];
    let postage_batch_id = "test_batch_id";
    let options = Some(RedundantUploadOptions {
        upload_options: UploadOptions {
            act: Some(true),
            act_history_address: Some("some_history_address".to_string()),
            pin: Some(true),
            encrypt: Some(true),
            tag: Some(expected_tag_uid),
            deferred: Some(false),
        },
        redundancy_level: Some(1),
    });

    let result = upload(&client, base_url, data, postage_batch_id, options).await;

    assert!(result.is_ok());
    let upload_result = result.unwrap();
    assert_eq!(upload_result.reference, expected_reference);
    assert_eq!(upload_result.tag_uid, Some(expected_tag_uid));
    assert_eq!(upload_result.history_address, Some(expected_history_address.to_string()));
}

#[tokio::test]
async fn test_head() {
    let expected_content_length = 12345;

    let mock_server = MockServer::start().await;
    Mock::given(method("HEAD"))
        .and(path_regex("/bytes/(.*)"))
        .respond_with(ResponseTemplate::new(200).insert_header("content-length", expected_content_length.to_string()))
        .mount(&mock_server)
        .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();
    let reference = "test_reference";

    let result = head(&client, base_url, reference).await;

    assert!(result.is_ok());
    let info = result.unwrap();
    assert_eq!(info.content_length, expected_content_length);
}

#[tokio::test]
async fn test_download() {
    let expected_data = vec![1, 2, 3, 4, 5];

    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex("/bytes/(.*)"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(expected_data.clone()))
        .mount(&mock_server)
        .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();
    let resource = "test_resource";
    let options = Some(DownloadOptions {
        redundancy_strategy: Some(1),
        fallback: Some(true),
        timeout_ms: Some(1000),
        act_publisher: Some("some_publisher".to_string()),
        act_history_address: Some("some_history_address".to_string()),
        act_timestamp: Some(12345),
    });

    let result = download(&client, base_url, resource, options).await;

    assert!(result.is_ok());
    let downloaded_data = result.unwrap();
    assert_eq!(downloaded_data, expected_data);
}
