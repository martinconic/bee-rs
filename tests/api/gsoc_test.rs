use crate::common::test_utils::spawn_mock_bee_with_warps;
use bee_rs::api::gsoc::send;
use bee_rs::api::bytes::UploadOptions;
use warp::{http::HeaderValue, Filter};

mod common;

#[tokio::test]
async fn test_send_gsoc() {
    let expected_reference = "36b7efd913ca4cf880b8eeac5093fa27b0825906c600685b6abdd6566e6cfe8f";
    let postage_batch_id = "test_batch_id";
    let soc_data = vec![1, 2, 3, 4, 5];

    let mock_server = spawn_mock_bee_with_warps(vec![
        warp::post()
            .and(warp::path("chunks")) // gsoc send calls chunk upload
            .and(warp::header::exact("content-type", "application/octet-stream"))
            .and(warp::header::exact("swarm-postage-batch-id", postage_batch_id))
            .and(warp::body::bytes())
            .map(move |body: bytes::Bytes| {
                assert_eq!(body.to_vec(), soc_data);
                warp::reply::json(&serde_json::json!({ "reference": expected_reference }))
            }),
    ])
    .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();
    let options = Some(UploadOptions::default());

    let result = send(&client, base_url, soc_data, postage_batch_id, options).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_reference);
}
