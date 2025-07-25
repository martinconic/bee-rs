use bee_rs::api::debug::balance::{BeeDebugBalanceClient, BalanceResponse, PeerBalance};
use warp::Filter;
use serde_json;

#[tokio::test]
async fn test_get_all_balances() {
    let balances_route = warp::path!("balances").map(|| {
        warp::reply::json(&serde_json::json!({
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
        }))
    });
    let (addr, server) = warp::serve(balances_route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugBalanceClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_all_balances().await;
    assert!(result.is_ok());
    let balances = result.unwrap();
    assert_eq!(balances.balances.len(), 2);
    assert_eq!(balances.balances[0].peer, "0x1234567890123456789012345678901234567890");
    assert_eq!(balances.balances[0].balance, "100000000000000000000");
}

#[tokio::test]
async fn test_get_peer_balance() {
    let peer_balance_route = warp::path!("balances" / String).map(|peer_address: String| {
        warp::reply::json(&serde_json::json!({
            "peer": peer_address,
            "balance": "100000000000000000000"
        }))
    });
    let (addr, server) = warp::serve(peer_balance_route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugBalanceClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_peer_balance("0x1234567890123456789012345678901234567890").await;
    assert!(result.is_ok());
    let peer_balance = result.unwrap();
    assert_eq!(peer_balance.peer, "0x1234567890123456789012345678901234567890");
    assert_eq!(peer_balance.balance, "100000000000000000000");
}

#[tokio::test]
async fn test_get_past_due_consumption_balances() {
    let consumed_route = warp::path!("consumed").map(|| {
        warp::reply::json(&serde_json::json!({
            "balances": [
                {
                    "peer": "0x1234567890123456789012345678901234567890",
                    "balance": "100000000000000000000"
                }
            ]
        }))
    });
    let (addr, server) = warp::serve(consumed_route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugBalanceClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_past_due_consumption_balances().await;
    assert!(result.is_ok());
    let balances = result.unwrap();
    assert_eq!(balances.balances.len(), 1);
    assert_eq!(balances.balances[0].peer, "0x1234567890123456789012345678901234567890");
    assert_eq!(balances.balances[0].balance, "100000000000000000000");
}

#[tokio::test]
async fn test_get_past_due_consumption_peer_balance() {
    let consumed_peer_route = warp::path!("consumed" / String).map(|peer_address: String| {
        warp::reply::json(&serde_json::json!({
            "peer": peer_address,
            "balance": "100000000000000000000"
        }))
    });
    let (addr, server) = warp::serve(consumed_peer_route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugBalanceClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_past_due_consumption_peer_balance("0x1234567890123456789012345678901234567890").await;
    assert!(result.is_ok());
    let peer_balance = result.unwrap();
    assert_eq!(peer_balance.peer, "0x1234567890123456789012345678901234567890");
    assert_eq!(peer_balance.balance, "100000000000000000000");
}
