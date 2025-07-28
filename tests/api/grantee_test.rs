use bee_rs::api::grantee::{create_grantees, get_grantees, patch_grantees, GetGranteesResult, GranteesResult};
// Add the `body_json` matcher to the import list
use wiremock::{matchers::{method, path_regex, header, body_json}, Mock, MockServer, ResponseTemplate};
use serde_json;
use std::collections::HashMap;

#[tokio::test]
async fn test_get_grantees() {
    let reference = "test_reference";
    let expected_grantees = vec![
        "0x1234567890123456789012345678901234567890".to_string(),
        "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd".to_string(),
    ];

    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(format!("/grantee/{}", reference))) // More specific path matching
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({ "grantees": expected_grantees })))
        .mount(&mock_server)
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

    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path_regex("/grantee"))
        .and(header("swarm-postage-batch-id", postage_batch_id))
        // Use the `body_json` matcher instead of `.with_body()`
        .and(body_json(serde_json::json!({ "grantees": grantees_to_create.clone() })))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({ // Changed to 201 Created for RESTful practice
            "ref": expected_reference,
            "historyref": expected_history_reference
        })))
        .mount(&mock_server)
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

    let mock_server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path_regex(format!("/grantee/{}", reference))) // More specific path matching
        .and(header("swarm-postage-batch-id", postage_batch_id))
        .and(header("swarm-act-history-address", history_reference))
        // Use the `body_json` matcher instead of `.with_body()`
        .and(body_json(serde_json::json!({
            "add": add_grantees,
            "revoke": revoke_grantees
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "ref": expected_reference,
            "historyref": expected_history_reference
        })))
        .mount(&mock_server)
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