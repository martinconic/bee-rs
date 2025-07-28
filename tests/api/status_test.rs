use bee_rs::api::debug::status::{BeeDebugClient, BeeMode};
use warp::Filter;
use warp::reply;

#[tokio::test]
async fn test_get_debug_status() {
    let status_route = warp::path!("status").map(|| {
        reply::json(&serde_json::json!({
            "overlay": "0x1234567890123456789012345678901234567890",
            "proximity": 8,
            "beeMode": "full",
            "reserveSize": 1000000,
            "reserveSizeWithinRadius": 500000,
            "pullsyncRate": 10,
            "storageRadius": 8,
            "connectedPeers": 16,
            "neighborhoodSize": 4,
            "batchCommitment": 1000,
            "isReachable": true,
            "lastSyncedBlock": 12345,
            "committedDepth": 8
        }))
    });
    let (addr, server) = warp::serve(status_route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_debug_status().await;
    assert!(result.is_ok());
    let status = result.unwrap();
    assert_eq!(status.overlay, "0x1234567890123456789012345678901234567890");
    assert_eq!(status.bee_mode, BeeMode::Full);
    assert_eq!(status.proximity, 8);
    assert_eq!(status.reserve_size, 1000000);
}

#[tokio::test]
async fn test_get_health() {
    let health_route = warp::path!("health").map(|| {
        reply::json(&serde_json::json!({
            "status": "ok",
            "version": "2.4.0-390a402e",
            "apiVersion": "7.2.0"
        }))
    });
    let (addr, server) = warp::serve(health_route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_health().await;
    assert!(result.is_ok());
    let health = result.unwrap();
    assert_eq!(health.status, "ok");
    assert_eq!(health.version, "2.4.0-390a402e");
    assert_eq!(health.api_version, "7.2.0");
}

#[tokio::test]
async fn test_get_readiness() {
    let readiness_route = warp::path!("readiness").map(|| {
        reply::json(&serde_json::json!({
            "status": "ready",
            "version": "2.4.0-390a402e",
            "apiVersion": "7.2.0"
        }))
    });
    let (addr, server) = warp::serve(readiness_route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_readiness().await;
    assert!(result.is_ok());
    let readiness = result.unwrap();
    assert_eq!(readiness.status, "ready");
    assert_eq!(readiness.version, "2.4.0-390a402e");
    assert_eq!(readiness.api_version, "7.2.0");
}

#[tokio::test]
async fn test_get_node_info() {
    let node_info_route = warp::path!("node").map(|| {
        reply::json(&serde_json::json!({
            "beeMode": "full",
            "chequebookEnabled": true,
            "swapEnabled": false
        }))
    });
    let (addr, server) = warp::serve(node_info_route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_node_info().await;
    assert!(result.is_ok());
    let node_info = result.unwrap();
    assert_eq!(node_info.bee_mode, BeeMode::Full);
    assert_eq!(node_info.chequebook_enabled, true);
    assert_eq!(node_info.swap_enabled, false);
}

#[tokio::test]
async fn test_is_supported_exact_version() {
    let health_route = warp::path!("health").map(|| {
        reply::json(&serde_json::json!({
            "status": "ok",
            "version": "2.4.0-390a402e",
            "apiVersion": "7.2.0"
        }))
    });
    let (addr, server) = warp::serve(health_route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.is_supported_exact_version().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true); // Matches SUPPORTED_BEE_VERSION_EXACT
}

#[tokio::test]
async fn test_is_supported_api_version() {
    let health_route = warp::path!("health").map(|| {
        reply::json(&serde_json::json!({
            "status": "ok",
            "version": "2.4.0-390a402e",
            "apiVersion": "7.2.0"
        }))
    });
    let (addr, server) = warp::serve(health_route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.is_supported_api_version().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true); // Major version matches SUPPORTED_API_VERSION
}

#[tokio::test]
async fn test_get_versions() {
    let health_route = warp::path!("health").map(|| {
        reply::json(&serde_json::json!({
            "status": "ok",
            "version": "2.4.0-390a402e",
            "apiVersion": "7.2.0"
        }))
    });
    let (addr, server) = warp::serve(health_route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_versions().await;
    assert!(result.is_ok());
    let versions = result.unwrap();
    assert_eq!(versions.supported_bee_version, "2.4.0-390a402e");
    assert_eq!(versions.supported_bee_api_version, "7.2.0");
    assert_eq!(versions.bee_version, "2.4.0-390a402e");
    assert_eq!(versions.bee_api_version, "7.2.0");
}