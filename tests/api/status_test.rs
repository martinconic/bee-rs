use crate::common::test_utils::spawn_mock_bee_with_warps;
use bee_rs::api::status::{check_connection, is_gateway};
use warp::Filter;

mod common;

#[tokio::test]
async fn test_check_connection() {
    let mock_server = spawn_mock_bee_with_warps(vec![
        warp::get()
            .and(warp::path::end())
            .map(|| "OK"),
    ])
    .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();

    let result = check_connection(&client, base_url).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_is_gateway() {
    let mock_server = spawn_mock_bee_with_warps(vec![
        warp::get()
            .and(warp::path!("gateway"))
            .map(|| "{\"gateway\": true}"),
    ])
    .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();

    let result = is_gateway(&client, base_url).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}
