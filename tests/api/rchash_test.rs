use crate::common::test_utils::spawn_mock_bee_with_warps;
use bee_rs::api::rchash::rchash;
use warp::Filter;

mod common;

#[tokio::test]
async fn test_rchash() {
    let depth = 10;
    let anchor1 = "anchor1_value";
    let anchor2 = "anchor2_value";
    let expected_duration = 123.45;

    let mock_server = spawn_mock_bee_with_warps(vec![
        warp::get()
            .and(warp::path!("rchash" / u32 / String / String))
            .map(move |d: u32, a1: String, a2: String| {
                assert_eq!(d, depth);
                assert_eq!(a1, anchor1);
                assert_eq!(a2, anchor2);
                warp::reply::json(&serde_json::json!({ "durationSeconds": expected_duration }))
            }),
    ])
    .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();

    let result = rchash(&client, base_url, depth, anchor1, anchor2).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_duration);
}
