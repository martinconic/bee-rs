use bee_rs::api::debug::balance::{BeeDebugBalanceClient, BalanceResponse, PeerBalance};
use wiremock::{matchers::{method, path_regex}, Mock, MockServer, ResponseTemplate};
use serde_json;

#[tokio::test]
async fn test_get_all_balances() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex("/balances"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "balances": [
                {
                    "peer": "0x1234567890123456789012345678901234567890",
                    "balance": "100000000000000000000"
                },
                {
                    "peer": "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd",
                    "balance": "50000000000000000000"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    // Use mock_server.uri() to get the base URL
    let client = BeeDebugBalanceClient::new(&mock_server.uri()).unwrap();
    let result = client.get_all_balances().await;
    assert!(result.is_ok());
    let balances = result.unwrap();
    assert_eq!(balances.balances.len(), 2);
    assert_eq!(balances.balances[0].peer, "0x1234567890123456789012345678901234567890");
    assert_eq!(balances.balances[0].balance, "100000000000000000000");
}

#[tokio::test]
async fn test_get_peer_balance() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex("/balances/(.*)"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "peer": "0x1234567890123456789012345678901234567890",
            "balance": "100000000000000000000"
        })))
        .mount(&mock_server)
        .await;

    // Use mock_server.uri() to get the base URL
    let client = BeeDebugBalanceClient::new(&mock_server.uri()).unwrap();
    let result = client.get_peer_balance("0x1234567890123456789012345678901234567890").await;
    assert!(result.is_ok());
    let peer_balance = result.unwrap();
    assert_eq!(peer_balance.peer, "0x1234567890123456789012345678901234567890");
    assert_eq!(peer_balance.balance, "100000000000000000000");
}

#[tokio::test]
async fn test_get_past_due_consumption_balances() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex("/consumed"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "balances": [
                {
                    "peer": "0x1234567890123456789012345678901234567890",
                    "balance": "100000000000000000000"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    // Use mock_server.uri() to get the base URL
    let client = BeeDebugBalanceClient::new(&mock_server.uri()).unwrap();
    let result = client.get_past_due_consumption_balances().await;
    assert!(result.is_ok());
    let balances = result.unwrap();
    assert_eq!(balances.balances.len(), 1);
    assert_eq!(balances.balances[0].peer, "0x1234567890123456789012345678901234567890");
    assert_eq!(balances.balances[0].balance, "100000000000000000000");
}

#[tokio::test]
async fn test_get_past_due_consumption_peer_balance() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex("/consumed/(.*)"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "peer": "0x1234567890123456789012345678901234567890",
            "balance": "100000000000000000000"
        })))
        .mount(&mock_server)
        .await;

    // Use mock_server.uri() to get the base URL
    let client = BeeDebugBalanceClient::new(&mock_server.uri()).unwrap();
    let result = client.get_past_due_consumption_peer_balance("0x1234567890123456789012345678901234567890").await;
    assert!(result.is_ok());
    let peer_balance = result.unwrap();
    assert_eq!(peer_balance.peer, "0x1234567890123456789012345678901234567890");
    assert_eq!(peer_balance.balance, "100000000000000000000");
}