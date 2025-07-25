use bee_rs::api::debug::chequebook::{BeeDebugChequebookClient, ChequebookAddressResponse, ChequebookBalanceResponse, LastCashoutActionResponse, LastChequesForPeerResponse, LastChequesResponse, Cheque, CashoutResult};
use warp::Filter;
use serde_json;
use std::collections::HashMap;

#[tokio::test]
async fn test_get_chequebook_address() {
    let route = warp::path!("chequebook" / "address").map(|| {
        warp::reply::json(&ChequebookAddressResponse {
            chequebook_address: "0x1234567890123456789012345678901234567890".to_string(),
        })
    });
    let (addr, server) = warp::serve(route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugChequebookClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_chequebook_address().await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.chequebook_address, "0x1234567890123456789012345678901234567890");
}

#[tokio::test]
async fn test_get_chequebook_balance() {
    let route = warp::path!("chequebook" / "balance").map(|| {
        warp::reply::json(&ChequebookBalanceResponse {
            total_balance: "100000000000000000000".to_string(),
            available_balance: "50000000000000000000".to_string(),
        })
    });
    let (addr, server) = warp::serve(route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugChequebookClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_chequebook_balance().await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.total_balance, "100000000000000000000");
    assert_eq!(response.available_balance, "50000000000000000000");
}

#[tokio::test]
async fn test_get_last_cashout_action() {
    let route = warp::path!("chequebook" / "cashout" / String).map(|peer: String| {
        warp::reply::json(&LastCashoutActionResponse {
            peer,
            uncashed_amount: "10000000000000000000".to_string(),
            transaction_hash: Some("0x1234567890123456789012345678901234567890123456789012345678901234".to_string()),
            last_cashed_cheque: Some(Cheque {
                beneficiary: "0x1234567890123456789012345678901234567890".to_string(),
                chequebook: "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd".to_string(),
                payout: "1000000000000000000".to_string(),
            }),
            result: Some(CashoutResult {
                recipient: "0x1234567890123456789012345678901234567890".to_string(),
                last_payout: "1000000000000000000".to_string(),
                bounced: false,
            }),
        })
    });
    let (addr, server) = warp::serve(route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugChequebookClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_last_cashout_action("0x1234567890123456789012345678901234567890123456789012345678901234").await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.uncashed_amount, "10000000000000000000");
}

#[tokio::test]
async fn test_cashout_last_cheque() {
    let route = warp::path!("chequebook" / "cashout" / String).and(warp::post()).map(|_peer: String| {
        warp::reply::json(&serde_json::json!({
            "transactionHash": "0x1234567890123456789012345678901234567890123456789012345678901234"
        }))
    });
    let (addr, server) = warp::serve(route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugChequebookClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.cashout_last_cheque("0x1234567890123456789012345678901234567890123456789012345678901234", None, None).await;
    assert!(result.is_ok());
    let tx_hash = result.unwrap();
    assert_eq!(tx_hash, "0x1234567890123456789012345678901234567890123456789012345678901234");
}

#[tokio::test]
async fn test_get_last_cheques_for_peer() {
    let route = warp::path!("chequebook" / "cheque" / String).map(|peer: String| {
        warp::reply::json(&LastChequesForPeerResponse {
            peer,
            lastreceived: Some(Cheque {
                beneficiary: "0x1234567890123456789012345678901234567890".to_string(),
                chequebook: "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd".to_string(),
                payout: "1000000000000000000".to_string(),
            }),
            lastsent: None,
        })
    });
    let (addr, server) = warp::serve(route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugChequebookClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_last_cheques_for_peer("0x1234567890123456789012345678901234567890123456789012345678901234").await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response.lastreceived.is_some());
}

#[tokio::test]
async fn test_get_last_cheques() {
    let route = warp::path!("chequebook" / "cheque").map(|| {
        warp::reply::json(&LastChequesResponse {
            lastcheques: vec![
                LastChequesForPeerResponse {
                    peer: "0x1234567890123456789012345678901234567890123456789012345678901234".to_string(),
                    lastreceived: Some(Cheque {
                        beneficiary: "0x1234567890123456789012345678901234567890".to_string(),
                        chequebook: "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd".to_string(),
                        payout: "1000000000000000000".to_string(),
                    }),
                    lastsent: None,
                },
            ],
        })
    });
    let (addr, server) = warp::serve(route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugChequebookClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_last_cheques().await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.lastcheques.len(), 1);
}

#[tokio::test]
async fn test_deposit_tokens() {
    let route = warp::path!("chequebook" / "deposit").and(warp::post()).and(warp::query::query()).map(|_query: HashMap<String, String>| {
        warp::reply::json(&serde_json::json!({
            "transactionHash": "0x1234567890123456789012345678901234567890123456789012345678901234"
        }))
    });
    let (addr, server) = warp::serve(route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugChequebookClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.deposit_tokens("1000000000000000000", None).await;
    assert!(result.is_ok());
    let tx_hash = result.unwrap();
    assert_eq!(tx_hash, "0x1234567890123456789012345678901234567890123456789012345678901234");
}

#[tokio::test]
async fn test_withdraw_tokens() {
    let route = warp::path!("chequebook" / "withdraw").and(warp::post()).and(warp::query::query()).map(|_query: HashMap<String, String>| {
        warp::reply::json(&serde_json::json!({
            "transactionHash": "0x1234567890123456789012345678901234567890123456789012345678901234"
        }))
    });
    let (addr, server) = warp::serve(route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugChequebookClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.withdraw_tokens("1000000000000000000", None).await;
    assert!(result.is_ok());
    let tx_hash = result.unwrap();
    assert_eq!(tx_hash, "0x1234567890123456789012345678901234567890123456789012345678901234");
}