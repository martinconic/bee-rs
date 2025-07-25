use crate::common::test_utils::spawn_mock_bee_with_warps;
use bee_rs::api::feed::{create_feed_manifest, fetch_latest_feed_update, probe_feed, FeedUpdateOptions};
use bee_rs::api::bytes::UploadOptions;
use warp::{http::HeaderValue, Filter};

mod common;

#[tokio::test]
async fn test_create_feed_manifest() {
    let owner = "0x1234567890123456789012345678901234567890";
    let topic = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd";
    let postage_batch_id = "test_batch_id";
    let expected_reference = "36b7efd913ca4cf880b8eeac5093fa27b0825906c600685b6abdd6566e6cfe8f";

    let mock_server = spawn_mock_bee_with_warps(vec![
        warp::post()
            .and(warp::path!("feeds" / String / String))
            .and(warp::header::exact("swarm-postage-batch-id", postage_batch_id))
            .and(warp::header::optional::<String>("swarm-act"))
            .and(warp::header::optional::<String>("swarm-act-history-address"))
            .and(warp::header::optional::<String>("swarm-pin"))
            .and(warp::header::optional::<String>("swarm-encrypt"))
            .and(warp::header::optional::<String>("swarm-tag"))
            .and(warp::header::optional::<String>("swarm-deferred"))
            .map(move |owner_param: String, topic_param: String, act: Option<String>, act_history_address: Option<String>, pin: Option<String>, encrypt: Option<String>, tag: Option<String>, deferred: Option<String>| {
                assert_eq!(owner_param, owner);
                assert_eq!(topic_param, topic);
                if let Some(act_val) = act { assert_eq!(act_val, "true"); }
                if let Some(act_history_address_val) = act_history_address { assert_eq!(act_history_address_val, "some_history_address"); }
                if let Some(pin_val) = pin { assert_eq!(pin_val, "true"); }
                if let Some(encrypt_val) = encrypt { assert_eq!(encrypt_val, "true"); }
                if let Some(tag_val) = tag { assert_eq!(tag_val, "123"); }
                if let Some(deferred_val) = deferred { assert_eq!(deferred_val, "false"); }
                warp::reply::json(&serde_json::json!({ "reference": expected_reference }))
            }),
    ])
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

    let mock_server = spawn_mock_bee_with_warps(vec![
        warp::get()
            .and(warp::path!("feeds" / String / String))
            .and(warp::query::optional::<FeedUpdateOptions>())
            .map(move |owner_param: String, topic_param: String, options: Option<FeedUpdateOptions>| {
                assert_eq!(owner_param, owner);
                assert_eq!(topic_param, topic);
                if let Some(opts) = options {
                    assert_eq!(opts.at, Some(123));
                    assert_eq!(opts.index, Some("0000000000000000".to_string()));
                    assert_eq!(opts.has_timestamp, Some(true));
                }
                warp::reply::with_headers(
                    warp::reply::with_status(expected_payload.clone(), warp::http::StatusCode::OK),
                    {
                        let mut headers = warp::http::HeaderMap::new();
                        headers.insert("swarm-feed-index", HeaderValue::from_str(expected_feed_index).unwrap());
                        headers.insert("swarm-feed-index-next", HeaderValue::from_str(expected_feed_index_next).unwrap());
                        headers
                    },
                )
            }),
    ])
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

    let mock_server = spawn_mock_bee_with_warps(vec![
        warp::get()
            .and(warp::path!("feeds" / String / String))
            .and(warp::header::exact("Swarm-Only-Root-Chunk", "true"))
            .map(move |owner_param: String, topic_param: String| {
                assert_eq!(owner_param, owner);
                assert_eq!(topic_param, topic);
                warp::reply::with_headers(
                    warp::reply::reply(),
                    {
                        let mut headers = warp::http::HeaderMap::new();
                        headers.insert("swarm-feed-index", HeaderValue::from_str(expected_feed_index).unwrap());
                        headers.insert("swarm-feed-index-next", HeaderValue::from_str(expected_feed_index_next).unwrap());
                        headers
                    },
                )
            }),
    ])
    .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();

    let result = probe_feed(&client, base_url, owner, topic).await;

    assert!(result.is_ok());
    let feed_headers = result.unwrap();
    assert_eq!(feed_headers.feed_index, expected_feed_index);
    assert_eq!(feed_headers.feed_index_next, expected_feed_index_next);
}
