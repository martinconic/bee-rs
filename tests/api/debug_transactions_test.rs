use bee_rs::api::debug::transactions::{BeeDebugTransactionsClient, TransactionInfo};
use warp::Filter;
use serde_json;
use std::collections::HashMap;

#[tokio::test]
async fn test_get_all_transactions() {
    let route = warp::path!("transactions").map(|| {
        warp::reply::json(&serde_json::json!({
            "pendingTransactions": [
                {
                    "transactionHash": "0x1234567890123456789012345678901234567890123456789012345678901234",
                    "to": "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd",
                    "nonce": 1,
                    "gasPrice": "1000000000000000",
                    "gasLimit": 21000,
                    "data": "0x",
                    "created": "2023-01-01T00:00:00Z",
                    "description": "test transaction",
                    "value": "1000000000000000000"
                }
            ]
        }))
    });
    let (addr, server) = warp::serve(route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugTransactionsClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_all_transactions().await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.len(), 1);
    assert_eq!(response[0].transaction_hash, "0x1234567890123456789012345678901234567890123456789012345678901234");
}

#[tokio::test]
async fn test_get_transaction() {
    let route = warp::path!("transactions" / String).map(|tx_hash: String| {
        warp::reply::json(&TransactionInfo {
            transaction_hash: tx_hash,
            to_address: "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd".to_string(),
            nonce: 1,
            gas_price: "1000000000000000".to_string(),
            gas_limit: 21000,
            data: "0x".to_string(),
            created: "2023-01-01T00:00:00Z".to_string(),
            description: "test transaction".to_string(),
            value: "1000000000000000000".to_string(),
        })
    });
    let (addr, server) = warp::serve(route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugTransactionsClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_transaction("0x1234567890123456789012345678901234567890123456789012345678901234").await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.transaction_hash, "0x1234567890123456789012345678901234567890123456789012345678901234");
}

#[tokio::test]
async fn test_rebroadcast_transaction() {
    let route = warp::path!("transactions" / String).and(warp::post()).map(|_tx_hash: String| {
        warp::reply::json(&serde_json::json!({
            "transactionHash": "0x1234567890123456789012345678901234567890123456789012345678901234"
        }))
    });
    let (addr, server) = warp::serve(route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugTransactionsClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.rebroadcast_transaction("0x1234567890123456789012345678901234567890123456789012345678901234").await;
    assert!(result.is_ok());
    let tx_hash = result.unwrap();
    assert_eq!(tx_hash, "0x1234567890123456789012345678901234567890123456789012345678901234");
}

#[tokio::test]
async fn test_cancel_transaction() {
    let route = warp::path!("transactions" / String).and(warp::delete()).and(warp::header::optional("gas-price")).map(|_tx_hash: String, _gas_price: Option<String>| {
        warp::reply::json(&serde_json::json!({
            "transactionHash": "0x1234567890123456789012345678901234567890123456789012345678901234"
        }))
    });
    let (addr, server) = warp::serve(route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugTransactionsClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.cancel_transaction("0x1234567890123456789012345678901234567890123456789012345678901234", None).await;
    assert!(result.is_ok());
    let tx_hash = result.unwrap();
    assert_eq!(tx_hash, "0x1234567890123456789012345678901234567890123456789012345678901234");
}
