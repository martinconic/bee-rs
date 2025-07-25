use crate::common::test_utils::spawn_mock_bee_with_warps;
use bee_rs::api::pss::send;
use warp::{Filter, http::StatusCode};

mod common;

#[tokio::test]
async fn test_send_pss() {
    let topic = "test_topic";
    let target = "test_target";
    let data = vec![1, 2, 3, 4, 5];
    let postage_batch_id = "test_batch_id";
    let recipient = Some("test_recipient");

    let mock_server = spawn_mock_bee_with_warps(vec![
        warp::post()
            .and(warp::path!("pss" / "send" / String / String))
            .and(warp::header::exact("swarm-postage-batch-id", postage_batch_id))
            .and(warp::query::optional::<String>("recipient"))
            .and(warp::body::bytes())
            .map(move |topic_param: String, target_param: String, rec: Option<String>, body: bytes::Bytes| {
                assert_eq!(topic_param, topic);
                assert_eq!(target_param, target);
                assert_eq!(rec, recipient);
                assert_eq!(body.to_vec(), data);
                warp::reply::with_status("", StatusCode::OK)
            }),
    ])
    .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();

    let result = send(&client, base_url, topic, target, data, postage_batch_id, recipient).await;

    assert!(result.is_ok());
}
