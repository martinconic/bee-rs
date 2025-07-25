use bee_rs::api::debug::status::{BeeDebugClient, BeeMode, DebugStatus, Health, NodeInfo, Readiness, BeeVersions};
use warp::Filter;

#[tokio::test]
async fn test_get_debug_status() {
    let debug_status_route = warp::path!("status").map(|| {
        warp::reply::json(&DebugStatus {
            overlay: "test_overlay".to_string(),
            proximity: 1,
            bee_mode: BeeMode::Full,
            reserve_size: 100,
            reserve_size_within_radius: 50,
            pullsync_rate: 10,
            storage_radius: 5,
            connected_peers: 2,
            neighborhood_size: 3,
            batch_commitment: 4,
            is_reachable: true,
            last_synced_block: 1000,
            committed_depth: 10,
        })
    });
    let (addr, server) = warp::serve(debug_status_route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_debug_status().await;
    assert!(result.is_ok());
    let status = result.unwrap();
    assert_eq!(status.overlay, "test_overlay");
    assert_eq!(status.bee_mode, BeeMode::Full);
}

#[tokio::test]
async fn test_get_health() {
    let health_route = warp::path!("health").map(|| {
        warp::reply::json(&Health {
            status: "ok".to_string(),
            version: "1.0.0".to_string(),
            api_version: "1.0.0".to_string(),
        })
    });
    let (addr, server) = warp::serve(health_route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_health().await;
    assert!(result.is_ok());
    let health = result.unwrap();
    assert_eq!(health.status, "ok");
    assert_eq!(health.version, "1.0.0");
}

#[tokio::test]
async fn test_get_readiness() {
    let readiness_route = warp::path!("readiness").map(|| {
        warp::reply::json(&Readiness {
            status: "ready".to_string(),
            version: "1.0.0".to_string(),
            api_version: "1.0.0".to_string(),
        })
    });
    let (addr, server) = warp::serve(readiness_route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_readiness().await;
    assert!(result.is_ok());
    let readiness = result.unwrap();
    assert_eq!(readiness.status, "ready");
}

#[tokio::test]
async fn test_get_node_info() {
    let node_info_route = warp::path!("node").map(|| {
        warp::reply::json(&NodeInfo {
            bee_mode: BeeMode::Light,
            chequebook_enabled: false,
            swap_enabled: true,
        })
    });
    let (addr, server) = warp::serve(node_info_route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_node_info().await;
    assert!(result.is_ok());
    let node_info = result.unwrap();
    assert_eq!(node_info.bee_mode, BeeMode::Light);
}

#[tokio::test]
async fn test_is_supported_exact_version() {
    let health_route = warp::path!("health").map(|| {
        warp::reply::json(&Health {
            status: "ok".to_string(),
            version: "2.4.0-390a402e".to_string(),
            api_version: "7.2.0".to_string(),
        })
    });
    let (addr, server) = warp::serve(health_route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.is_supported_exact_version().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}

#[tokio::test]
async fn test_is_supported_api_version() {
    let health_route = warp::path!("health").map(|| {
        warp::reply::json(&Health {
            status: "ok".to_string(),
            version: "2.4.0-390a402e".to_string(),
            api_version: "7.2.0".to_string(),
        })
    });
    let (addr, server) = warp::serve(health_route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.is_supported_api_version().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}

#[tokio::test]
async fn test_get_versions() {
    let health_route = warp::path!("health").map(|| {
        warp::reply::json(&Health {
            status: "ok".to_string(),
            version: "2.4.0-390a402e".to_string(),
            api_version: "7.2.0".to_string(),
        })
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
