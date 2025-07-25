use crate::common::test_utils::spawn_mock_bee_with_warps;
use bee_rs::api::envelope::{post_envelope, EnvelopeWithBatchId};
use warp::{http::HeaderValue, Filter};

mod common;

#[tokio::test]
async fn test_post_envelope() {
    let expected_issuer = vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22, 0x33, 0x44];
    let expected_index = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    let expected_timestamp = vec![0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01];
    let expected_signature = vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11];
    let expected_batch_id = "test_batch_id";
    let reference = "test_reference";

    let mock_response_body = serde_json::json!({
        "issuer": hex::encode(&expected_issuer),
        "index": hex::encode(&expected_index),
        "timestamp": hex::encode(&expected_timestamp),
        "signature": hex::encode(&expected_signature),
    }).to_string();

    let mock_server = spawn_mock_bee_with_warps(vec![
        warp::post()
            .and(warp::path!("envelope" / String))
            .and(warp::header::exact("swarm-postage-batch-id", expected_batch_id))
            .map(move |ref_param: String| {
                assert_eq!(ref_param, reference);
                warp::reply::json(&serde_json::from_str::<EnvelopeWithBatchId>(&mock_response_body).unwrap())
            }),
    ])
    .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();

    let result = post_envelope(&client, base_url, expected_batch_id, reference).await;

    assert!(result.is_ok());
    let envelope = result.unwrap();
    assert_eq!(envelope.issuer, expected_issuer);
    assert_eq!(envelope.index, expected_index);
    assert_eq!(envelope.timestamp, expected_timestamp);
    assert_eq!(envelope.signature, expected_signature);
    assert_eq!(envelope.batch_id, expected_batch_id);
}
