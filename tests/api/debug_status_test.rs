use bee_rs::api::debug::status::{BeeDebugClient, BeeMode, DebugStatus, Health, NodeInfo, Readiness, BeeVersions};
use wiremock::{matchers::{method, path}, Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_debug_status() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(DebugStatus {
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
        }))
        .mount(&mock_server)
        .await;

    let client = BeeDebugClient::new(&mock_server.uri()).unwrap();
    let result = client.get_debug_status().await;
    assert!(result.is_ok());
    let status = result.unwrap();
    assert_eq!(status.overlay, "test_overlay");
    assert_eq!(status.bee_mode, BeeMode::Full);
}

#[tokio::test]
async fn test_get_health() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(200).set_body_json(Health {
            status: "ok".to_string(),
            version: "1.0.0".to_string(),
            api_version: "1.0.0".to_string(),
        }))
        .mount(&mock_server)
        .await;

    let client = BeeDebugClient::new(&mock_server.uri()).unwrap();
    let result = client.get_health().await;
    assert!(result.is_ok());
    let health = result.unwrap();
    assert_eq!(health.status, "ok");
    assert_eq!(health.version, "1.0.0");
}

#[tokio::test]
async fn test_get_readiness() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/readiness"))
        .respond_with(ResponseTemplate::new(200).set_body_json(Readiness {
            status: "ready".to_string(),
            version: "1.0.0".to_string(),
            api_version: "1.0.0".to_string(),
        }))
        .mount(&mock_server)
        .await;

    let client = BeeDebugClient::new(&mock_server.uri()).unwrap();
    let result = client.get_readiness().await;
    assert!(result.is_ok());
    let readiness = result.unwrap();
    assert_eq!(readiness.status, "ready");
}

#[tokio::test]
async fn test_get_node_info() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/node"))
        .respond_with(ResponseTemplate::new(200).set_body_json(NodeInfo {
            bee_mode: BeeMode::Light,
            chequebook_enabled: false,
            swap_enabled: true,
        }))
        .mount(&mock_server)
        .await;

    let client = BeeDebugClient::new(&mock_server.uri()).unwrap();
    let result = client.get_node_info().await;
    assert!(result.is_ok());
    let node_info = result.unwrap();
    assert_eq!(node_info.bee_mode, BeeMode::Light);
}

#[tokio::test]
async fn test_is_supported_exact_version() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(200).set_body_json(Health {
            status: "ok".to_string(),
            version: "2.4.0-390a402e".to_string(),
            api_version: "7.2.0".to_string(),
        }))
        .mount(&mock_server)
        .await;

    let client = BeeDebugClient::new(&mock_server.uri()).unwrap();
    let result = client.is_supported_exact_version().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}

#[tokio::test]
async fn test_is_supported_api_version() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(200).set_body_json(Health {
            status: "ok".to_string(),
            version: "2.4.0-390a402e".to_string(),
            api_version: "7.2.0".to_string(),
        }))
        .mount(&mock_server)
        .await;

    let client = BeeDebugClient::new(&mock_server.uri()).unwrap();
    let result = client.is_supported_api_version().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}

#[tokio::test]
async fn test_get_versions() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(200).set_body_json(Health {
            status: "ok".to_string(),
            version: "2.4.0-390a402e".to_string(),
            api_version: "7.2.0".to_string(),
        }))
        .mount(&mock_server)
        .await;

    let client = BeeDebugClient::new(&mock_server.uri()).unwrap();
    let result = client.get_versions().await;
    assert!(result.is_ok());
    let versions = result.unwrap();
    assert_eq!(versions.supported_bee_version, "2.4.0-390a402e");
    assert_eq!(versions.supported_bee_api_version, "7.2.0");
    assert_eq!(versions.bee_version, "2.4.0-390a402e");
    assert_eq!(versions.bee_api_version, "7.2.0");
}
