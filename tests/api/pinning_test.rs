use crate::common::test_utils::spawn_mock_bee_with_warps;
use bee_rs::api::pinning::{get_all_pins, get_pin, pin, unpin};
use warp::{Filter, http::StatusCode};

mod common;

#[tokio::test]
async fn test_pin() {
    let reference = "test_reference_pin";

    let mock_server = spawn_mock_bee_with_warps(vec![
        warp::post()
            .and(warp::path!("pins" / String))
            .map(move |ref_param: String| {
                assert_eq!(ref_param, reference);
                warp::reply::with_status("", StatusCode::OK)
            }),
    ])
    .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();

    let result = pin(&client, base_url, reference).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_unpin() {
    let reference = "test_reference_unpin";

    let mock_server = spawn_mock_bee_with_warps(vec![
        warp::delete()
            .and(warp::path!("pins" / String))
            .map(move |ref_param: String| {
                assert_eq!(ref_param, reference);
                warp::reply::with_status("", StatusCode::OK)
            }),
    ])
    .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();

    let result = unpin(&client, base_url, reference).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_pin() {
    let reference = "test_reference_get_pin";
    let expected_reference = "returned_reference_get_pin";

    let mock_server = spawn_mock_bee_with_warps(vec![
        warp::get()
            .and(warp::path!("pins" / String))
            .map(move |ref_param: String| {
                assert_eq!(ref_param, reference);
                warp::reply::json(&serde_json::json!({ "reference": expected_reference }))
            }),
    ])
    .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();

    let result = get_pin(&client, base_url, reference).await;

    assert!(result.is_ok());
    let pin_info = result.unwrap();
    assert_eq!(pin_info.reference, expected_reference);
}

#[tokio::test]
async fn test_get_all_pins() {
    let expected_references = vec![
        "ref1".to_string(),
        "ref2".to_string(),
    ];

    let mock_server = spawn_mock_bee_with_warps(vec![
        warp::get()
            .and(warp::path("pins"))
            .map(move || {
                warp::reply::json(&serde_json::json!({ "references": expected_references }))
            }),
    ])
    .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();

    let result = get_all_pins(&client, base_url).await;

    assert!(result.is_ok());
    let all_pins = result.unwrap();
    assert_eq!(all_pins, expected_references);
}
