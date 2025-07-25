use crate::common::test_utils::spawn_mock_bee_with_warps;
use bee_rs::api::bzz::{download_file, upload_file, FileData, FileUploadOptions, CollectionUploadOptions};
use bee_rs::api::bytes::{DownloadOptions, RedundantUploadOptions, UploadOptions};
use warp::{http::HeaderValue, Filter};

mod common;

#[tokio::test]
async fn test_upload_file() {
    let expected_reference = "36b7efd913ca4cf880b8eeac5093fa27b0825906c600685b6abdd6566e6cfe8f";
    let expected_tag_uid = 456;
    let expected_history_address = "another_history_address_for_bzz";
    let mock_response_body = format!("{{\"reference\": \"{}\"}}", expected_reference);

    let mock_server = spawn_mock_bee_with_warps(vec![
        warp::post()
            .and(warp::path("bzz"))
            .and(warp::header::exact("swarm-postage-batch-id", "test_batch_id"))
            .and(warp::header::optional::<String>("content-type"))
            .and(warp::header::optional::<String>("content-length"))
            .and(warp::header::optional::<String>("swarm-act"))
            .and(warp::header::optional::<String>("swarm-act-history-address"))
            .and(warp::header::optional::<String>("swarm-pin"))
            .and(warp::header::optional::<String>("swarm-encrypt"))
            .and(warp::header::optional::<String>("swarm-tag"))
            .and(warp::header::optional::<String>("swarm-deferred"))
            .and(warp::header::optional::<String>("swarm-redundancy-level"))
            .and(warp::query::optional::<String>("name"))
            .and(warp::body::bytes())
            .map(move |content_type: Option<String>, content_length: Option<String>, act: Option<String>, act_history_address: Option<String>, pin: Option<String>, encrypt: Option<String>, tag: Option<String>, deferred: Option<String>, redundancy_level: Option<String>, name: Option<String>, body: bytes::Bytes| {
                assert_eq!(body.to_vec(), vec![10, 20, 30]);
                if let Some(ct) = content_type { assert_eq!(ct, "text/plain"); }
                if let Some(cl) = content_length { assert_eq!(cl, "3"); }
                if let Some(act_val) = act { assert_eq!(act_val, "true"); }
                if let Some(act_history_address_val) = act_history_address { assert_eq!(act_history_address_val, "some_bzz_history_address"); }
                if let Some(pin_val) = pin { assert_eq!(pin_val, "true"); }
                if let Some(encrypt_val) = encrypt { assert_eq!(encrypt_val, "true"); }
                if let Some(tag_val) = tag { assert_eq!(tag_val, "456"); }
                if let Some(deferred_val) = deferred { assert_eq!(deferred_val, "false"); }
                if let Some(redundancy_level_val) = redundancy_level { assert_eq!(redundancy_level_val, "2"); }
                if let Some(n) = name { assert_eq!(n, "test_file.txt"); }

                warp::reply::with_headers(
                    warp::reply::json(&serde_json::from_str::<UploadResult>(&mock_response_body).unwrap()),
                    {
                        let mut headers = warp::http::HeaderMap::new();
                        headers.insert("swarm-tag", HeaderValue::from_str(&expected_tag_uid.to_string()).unwrap());
                        headers.insert("swarm-act-history-address", HeaderValue::from_str(expected_history_address).unwrap());
                        headers
                    },
                )
            }),
    ])
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

    let mock_server = spawn_mock_bee_with_warps(vec![
        warp::get()
            .and(warp::path!("bzz" / String / String))
            .and(warp::header::optional::<String>("swarm-redundancy-strategy"))
            .and(warp::header::optional::<String>("swarm-fallback"))
            .and(warp::header::optional::<String>("swarm-timeout"))
            .and(warp::header::optional::<String>("swarm-act-publisher"))
            .and(warp::header::optional::<String>("swarm-act-history-address"))
            .and(warp::header::optional::<String>("swarm-act-timestamp"))
            .map(move |reference: String, path: String, redundancy_strategy: Option<String>, fallback: Option<String>, timeout: Option<String>, act_publisher: Option<String>, act_history_address: Option<String>, act_timestamp: Option<String>| {
                assert_eq!(reference, "test_reference");
                assert_eq!(path, "test_path");
                if let Some(strategy) = redundancy_strategy { assert_eq!(strategy, "1"); }
                if let Some(fb) = fallback { assert_eq!(fb, "true"); }
                if let Some(to) = timeout { assert_eq!(to, "1000"); }
                if let Some(publisher) = act_publisher { assert_eq!(publisher, "some_publisher"); }
                if let Some(history_address) = act_history_address { assert_eq!(history_address, "some_history_address"); }
                if let Some(timestamp) = act_timestamp { assert_eq!(timestamp, "12345"); }

                warp::reply::with_headers(
                    warp::reply::with_status(expected_data.clone(), warp::http::StatusCode::OK),
                    {
                        let mut headers = warp::http::HeaderMap::new();
                        headers.insert("swarm-file-name", HeaderValue::from_str(expected_name).unwrap());
                        headers.insert("swarm-tag", HeaderValue::from_str(&expected_tag_uid.to_string()).unwrap());
                        headers.insert("content-type", HeaderValue::from_str(expected_content_type).unwrap());
                        headers
                    },
                )
            }),
    ])
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
