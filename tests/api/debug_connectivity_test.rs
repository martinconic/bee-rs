use bee_rs::api::debug::connectivity::{BeeDebugConnectivityClient, NodeAddresses, Peer, RemovePeerResponse, Topology, PingResponse, Bin};
use warp::Filter;
use serde_json;
use std::collections::HashMap;

#[tokio::test]
async fn test_get_node_addresses() {
    let route = warp::path!("addresses").map(|| {
        warp::reply::json(&NodeAddresses {
            overlay: "0x1234567890123456789012345678901234567890123456789012345678901234".to_string(),
            underlay: vec!["underlay1".to_string(), "underlay2".to_string()],
            ethereum: "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd".to_string(),
            public_key: "public_key".to_string(),
            pss_public_key: "pss_public_key".to_string(),
        })
    });
    let (addr, server) = warp::serve(route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugConnectivityClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_node_addresses().await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.overlay, "0x1234567890123456789012345678901234567890123456789012345678901234");
}

#[tokio::test]
async fn test_get_peers() {
    let route = warp::path!("peers").map(|| {
        warp::reply::json(&serde_json::json!({
            "peers": [
                {
                    "address": "0x1234567890123456789012345678901234567890123456789012345678901234",
                    "fullNode": true
                }
            ]
        }))
    });
    let (addr, server) = warp::serve(route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugConnectivityClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_peers().await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.len(), 1);
    assert_eq!(response[0].address, "0x1234567890123456789012345678901234567890123456789012345678901234");
}

#[tokio::test]
async fn test_get_blocklist() {
    let route = warp::path!("blocklist").map(|| {
        warp::reply::json(&serde_json::json!({
            "peers": [
                {
                    "address": "0x1234567890123456789012345678901234567890123456789012345678901234",
                    "fullNode": false
                }
            ]
        }))
    });
    let (addr, server) = warp::serve(route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugConnectivityClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_blocklist().await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.len(), 1);
    assert_eq!(response[0].address, "0x1234567890123456789012345678901234567890123456789012345678901234");
}

#[tokio::test]
async fn test_remove_peer() {
    let route = warp::path!("peers" / String).and(warp::delete()).map(|peer_address: String| {
        warp::reply::json(&RemovePeerResponse {
            message: format!("peer {} removed", peer_address),
            code: 0,
        })
    });
    let (addr, server) = warp::serve(route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugConnectivityClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.remove_peer("0x1234567890123456789012345678901234567890123456789012345678901234").await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.code, 0);
}

#[tokio::test]
async fn test_get_topology() {
    let route = warp::path!("topology").map(|| {
        warp::reply::json(&serde_json::json!({
            "baseAddr": "0x123".to_string(),
            "population": 10,
            "connected": 5,
            "timestamp": "2023-01-01T00:00:00Z".to_string(),
            "nnLowWatermark": 1,
            "depth": 2,
            "reachability": "public".to_string(),
            "networkAvailability": "available".to_string(),
            "bins": {
                "bin_0": {
                    "population": 1,
                    "connected": 1,
                    "disconnectedPeers": [],
                    "connectedPeers": [{"address": "0x123".to_string(), "fullNode": true}],
                },
            },
        }))
    });
    let (addr, server) = warp::serve(route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugConnectivityClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_topology().await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.population, 10);
}

#[tokio::test]
async fn test_ping_peer() {
    let route = warp::path!("pingpong" / String).and(warp::post()).map(|_peer_address: String| {
        warp::reply::json(&PingResponse {
            rtt: "100ms".to_string(),
        })
    });
    let (addr, server) = warp::serve(route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugConnectivityClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.ping_peer("0x1234567890123456789012345678901234567890123456789012345678901234").await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.rtt, "100ms");
}