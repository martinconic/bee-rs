use crate::common::test_utils::spawn_mock_bee_with_warps;
use bee_rs::api::grantee::{create_grantees, get_grantees, patch_grantees, GetGranteesResult, GranteesResult};
use warp::{Filter, http::StatusCode};

mod common;

#[tokio::test]
async fn test_get_grantees() {
    let reference = "test_reference";
    let expected_grantees = vec![
        "0x1234567890123456789012345678901234567890".to_string(),
        "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd".to_string(),
    ];

    let mock_server = spawn_mock_bee_with_warps(vec![
        warp::get()
            .and(warp::path!("grantee" / String))
            .map(move |ref_param: String| {
                assert_eq!(ref_param, reference);
                warp::reply::json(&expected_grantees)
            }),
    ])
    .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();

    let result = get_grantees(&client, base_url, reference).await;

    assert!(result.is_ok());
    let grantees_result = result.unwrap();
    assert_eq!(grantees_result.grantees, expected_grantees);
}

#[tokio::test]
async fn test_create_grantees() {
    let postage_batch_id = "test_batch_id";
    let grantees_to_create = vec![
        "0x1111111111111111111111111111111111111111".to_string(),
        "0x2222222222222222222222222222222222222222".to_string(),
    ];
    let expected_reference = "created_reference";
    let expected_history_reference = "created_history_reference";

    let mock_server = spawn_mock_bee_with_warps(vec![
        warp::post()
            .and(warp::path("grantee"))
            .and(warp::header::exact("swarm-postage-batch-id", postage_batch_id))
            .and(warp::body::json())
            .map(move |body: serde_json::Value| {
                let received_grantees: Vec<String> = serde_json::from_value(body["grantees"].clone()).unwrap();
                assert_eq!(received_grantees, grantees_to_create);
                warp::reply::json(&serde_json::json!({
                    "ref": expected_reference,
                    "historyref": expected_history_reference
                }))
            }),
    ])
    .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();

    let result = create_grantees(&client, base_url, postage_batch_id, grantees_to_create).await;

    assert!(result.is_ok());
    let grantees_result = result.unwrap();
    assert_eq!(grantees_result.reference, expected_reference);
    assert_eq!(grantees_result.history_reference, expected_history_reference);
}

#[tokio::test]
async fn test_patch_grantees() {
    let postage_batch_id = "patch_batch_id";
    let reference = "patch_reference";
    let history_reference = "patch_history_reference";
    let add_grantees = Some(vec!["0x3333333333333333333333333333333333333333".to_string()]);
    let revoke_grantees = Some(vec!["0x4444444444444444444444444444444444444444".to_string()]);
    let expected_reference = "patched_reference";
    let expected_history_reference = "patched_history_reference";

    let mock_server = spawn_mock_bee_with_warps(vec![
        warp::patch()
            .and(warp::path!("grantee" / String))
            .and(warp::header::exact("swarm-postage-batch-id", postage_batch_id))
            .and(warp::header::exact("swarm-act-history-address", history_reference))
            .and(warp::body::json())
            .map(move |ref_param: String, body: serde_json::Value| {
                assert_eq!(ref_param, reference);
                let received_add: Option<Vec<String>> = serde_json::from_value(body["add"].clone()).unwrap();
                let received_revoke: Option<Vec<String>> = serde_json::from_value(body["revoke"].clone()).unwrap();
                assert_eq!(received_add, add_grantees);
                assert_eq!(received_revoke, revoke_grantees);
                warp::reply::json(&serde_json::json!({
                    "ref": expected_reference,
                    "historyref": expected_history_reference
                }))
            }),
    ])
    .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();

    let result = patch_grantees(
        &client,
        base_url,
        postage_batch_id,
        reference,
        history_reference,
        add_grantees,
        revoke_grantees,
    )
    .await;

    assert!(result.is_ok());
    let grantees_result = result.unwrap();
    assert_eq!(grantees_result.reference, expected_reference);
    assert_eq!(grantees_result.history_reference, expected_history_reference);
}
