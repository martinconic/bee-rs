use bee_rs::api::feed::{create_feed_manifest, fetch_latest_feed_update, probe_feed, FeedUpdateOptions};
use bee_rs::api::bytes::UploadOptions;
use wiremock::{matchers::{method, path_regex, header, query_param}, Mock, MockServer, ResponseTemplate};
use serde_json;

#[tokio::test]
async fn test_create_feed_manifest() {
    let owner = "0x1234567890123456789012345678901234567890";
    let topic = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd";
    let postage_batch_id = "test_batch_id";
    let expected_reference = "36b7efd913ca4cf880b8eeac5093fa27b0825906c600685b6abdd6566e6cfe8f";

    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path_regex(&format!("/feeds/{}/{}", owner, topic)))
        .and(header("swarm-postage-batch-id", postage_batch_id))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({ "reference": expected_reference })))
        .mount(&mock_server)
        .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();
    let options = Some(UploadOptions {
        act: Some(true),
        act_history_address: Some("some_history_address".to_string()),
        pin: Some(true),
        encrypt: Some(true),
        tag: Some(123),
        deferred: Some(false),
    });

    let result = create_feed_manifest(&client, base_url, owner, topic, postage_batch_id, options).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_reference);
}

#[tokio::test]
async fn test_fetch_latest_feed_update() {
    let owner = "0x1234567890123456789012345678901234567890";
    let topic = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd";
    let expected_feed_index = "0000000000000000";
    let expected_feed_index_next = "0000000000000001";
    let expected_payload = vec![1, 2, 3, 4, 5];

    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(&format!("/feeds/{}/{}", owner, topic)))
        .respond_with(ResponseTemplate::new(200)
            .set_body_bytes(expected_payload.clone())
            .insert_header("swarm-feed-index", expected_feed_index)
            .insert_header("swarm-feed-index-next", expected_feed_index_next))
        .mount(&mock_server)
        .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();
    let options = Some(FeedUpdateOptions {
        at: Some(123),
        index: Some("0000000000000000".to_string()),
        has_timestamp: Some(true),
    });

    let result = fetch_latest_feed_update(&client, base_url, owner, topic, options).await;

    assert!(result.is_ok());
    let feed_payload = result.unwrap();
    assert_eq!(feed_payload.payload, expected_payload);
    assert_eq!(feed_payload.headers.feed_index, expected_feed_index);
    assert_eq!(feed_payload.headers.feed_index_next, expected_feed_index_next);
}

#[tokio::test]
async fn test_probe_feed() {
    let owner = "0x1234567890123456789012345678901234567890";
    let topic = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd";
    let expected_feed_index = "0000000000000000";
    let expected_feed_index_next = "0000000000000001";

    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(&format!("/feeds/{}/{}", owner, topic)))
        .and(header("Swarm-Only-Root-Chunk", "true"))
        .respond_with(ResponseTemplate::new(200)
            .insert_header("swarm-feed-index", expected_feed_index)
            .insert_header("swarm-feed-index-next", expected_feed_index_next))
        .mount(&mock_server)
        .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();

    let result = probe_feed(&client, base_url, owner, topic).await;

    assert!(result.is_ok());
    let feed_headers = result.unwrap();
    assert_eq!(feed_headers.feed_index, expected_feed_index);
    assert_eq!(feed_headers.feed_index_next, expected_feed_index_next);
}
