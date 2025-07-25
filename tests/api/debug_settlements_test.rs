use bee_rs::api::debug::settlements::{BeeDebugSettlementsClient, Settlements, AllSettlements};
use warp::Filter;
use serde_json;

#[tokio::test]
async fn test_get_settlements() {
    let route = warp::path!("settlements" / String).map(|peer: String| {
        warp::reply::json(&Settlements {
            peer,
            received: "1000000000000000000".to_string(),
            sent: "500000000000000000".to_string(),
        })
    });
    let (addr, server) = warp::serve(route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugSettlementsClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_settlements("0x1234567890123456789012345678901234567890123456789012345678901234").await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.received, "1000000000000000000");
    assert_eq!(response.sent, "500000000000000000");
}

#[tokio::test]
async fn test_get_all_settlements() {
    let route = warp::path!("settlements").map(|| {
        warp::reply::json(&AllSettlements {
            total_received: "2000000000000000000".to_string(),
            total_sent: "1000000000000000000".to_string(),
            settlements: vec![
                Settlements {
                    peer: "0x1234567890123456789012345678901234567890123456789012345678901234".to_string(),
                    received: "1000000000000000000".to_string(),
                    sent: "500000000000000000".to_string(),
                },
            ],
        })
    });
    let (addr, server) = warp::serve(route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugSettlementsClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_all_settlements().await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.total_received, "2000000000000000000");
    assert_eq!(response.total_sent, "1000000000000000000");
    assert_eq!(response.settlements.len(), 1);
}