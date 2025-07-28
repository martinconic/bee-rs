use bee_rs::api::bzz::{download_file, upload_file, FileData, FileUploadOptions, CollectionUploadOptions};
use bee_rs::api::bytes::{DownloadOptions, RedundantUploadOptions, UploadOptions, UploadResult};
use wiremock::{matchers::{method, path_regex, header, query_param}, Mock, MockServer, ResponseTemplate};
use serde_json;

#[tokio::test]
async fn test_upload_file() {
    let expected_reference = "36b7efd913ca4cf880b8eeac5093fa27b0825906c600685b6abdd6566e6cfe8f";
    let expected_tag_uid = 456;
    let expected_history_address = "another_history_address_for_bzz";

    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path_regex("/bzz"))
        .and(header("swarm-postage-batch-id", "test_batch_id"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(serde_json::json!({ "reference": expected_reference }))
            .insert_header("swarm-tag", expected_tag_uid.to_string())
            .insert_header("swarm-act-history-address", expected_history_address))
        .mount(&mock_server)
        .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();
    let data = vec![10, 20, 30];
    let postage_batch_id = "test_batch_id";
    let name = Some("test_file.txt");
    let options = Some(FileUploadOptions {
        redundant_upload_options: RedundantUploadOptions {
            upload_options: UploadOptions {
                act: Some(true),
                act_history_address: Some("some_bzz_history_address".to_string()),
                pin: Some(true),
                encrypt: Some(true),
                tag: Some(expected_tag_uid),
                deferred: Some(false),
            },
            redundancy_level: Some(2),
        },
        size: Some(3),
        content_type: Some("text/plain".to_string()),
    });

    let result = upload_file(&client, base_url, data, postage_batch_id, name, options).await;

    assert!(result.is_ok());
    let upload_result = result.unwrap();
    assert_eq!(upload_result.reference, expected_reference);
    assert_eq!(upload_result.tag_uid, Some(expected_tag_uid));
    assert_eq!(upload_result.history_address, Some(expected_history_address.to_string()));
}

#[tokio::test]
async fn test_download_file() {
    let expected_data = vec![1, 2, 3, 4, 5];
    let expected_name = "downloaded_file.txt";
    let expected_tag_uid = 789;
    let expected_content_type = "application/json";

    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex("/bzz/(.*)/(.*)"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_bytes(expected_data.clone())
            .insert_header("swarm-file-name", expected_name)
            .insert_header("swarm-tag", expected_tag_uid.to_string())
            .insert_header("content-type", expected_content_type))
        .mount(&mock_server)
        .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();
    let reference = "test_reference";
    let path = Some("test_path");
    let options = Some(DownloadOptions {
        redundancy_strategy: Some(1),
        fallback: Some(true),
        timeout_ms: Some(1000),
        act_publisher: Some("some_publisher".to_string()),
        act_history_address: Some("some_history_address".to_string()),
        act_timestamp: Some(12345),
    });

    let result = download_file(&client, base_url, reference, path, options).await;

    assert!(result.is_ok());
    let file_data = result.unwrap();
    assert_eq!(file_data.data, expected_data);
    assert_eq!(file_data.name, Some(expected_name.to_string()));
    assert_eq!(file_data.tag_uid, Some(expected_tag_uid));
    assert_eq!(file_data.content_type, Some(expected_content_type.to_string()));
}
