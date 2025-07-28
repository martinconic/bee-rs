use bee_rs::api::debug::states::{BeeDebugStatesClient, ChainState, ReserveState, WalletBalance};
use wiremock::{matchers::{method, path_regex}, Mock, MockServer, ResponseTemplate};
use serde_json;

#[tokio::test]
async fn test_get_reserve_state() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex("/reservestate"))
        .respond_with(ResponseTemplate::new(200).set_body_json(ReserveState {
            commitment: 123,
            radius: 4,
            storage_radius: 5,
        }))
        .mount(&mock_server)
        .await;

    let client = BeeDebugStatesClient::new(&mock_server.uri()).unwrap();
    let result = client.get_reserve_state().await;
    assert!(result.is_ok());
    let reserve_state = result.unwrap();
    assert_eq!(reserve_state.commitment, 123);
}

#[tokio::test]
async fn test_get_chain_state() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex("/chainstate"))
        .respond_with(ResponseTemplate::new(200).set_body_json(ChainState {
            block: 100,
            chain_tip: 101,
            total_amount: "100000000000000000000".to_string(),
            current_price: "1.0".to_string(),
        }))
        .mount(&mock_server)
        .await;

    let client = BeeDebugStatesClient::new(&mock_server.uri()).unwrap();
    let result = client.get_chain_state().await;
    assert!(result.is_ok());
    let chain_state = result.unwrap();
    assert_eq!(chain_state.block, 100);
    assert_eq!(chain_state.total_amount, "100000000000000000000");
}

#[tokio::test]
async fn test_get_wallet_balance() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex("/wallet"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "bzzBalance": "76946885095818311",
            "nativeTokenBalance": "2999995780972583839"
        })))
        .mount(&mock_server)
        .await;

    let client = BeeDebugStatesClient::new(&mock_server.uri()).unwrap();
    let result = client.get_wallet_balance().await;
    if let Err(e) = &result {
        eprintln!("Error in test_get_wallet_balance: {}", e);
    }
    assert!(result.is_ok());
    let wallet_balance = result.unwrap();
    assert_eq!(wallet_balance.bzz_balance, "76946885095818311");
    assert_eq!(wallet_balance.native_token_balance, "2999995780972583839");
}