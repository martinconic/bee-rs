use bee_rs::api::debug::stamps::{
    BeeDebugStampsClient, GlobalPostageBatch, PostageBatch, PostageBatchBucket, PostageBatchBuckets,
};
use serde_json;
use wiremock::{
    matchers::{header, method, path_regex, query_param},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn test_get_global_postage_batches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex("/batches"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "batchID": "0x1234567890123456789012345678901234567890123456789012345678901234",
                "value": "1000000000000000000",
                "start": 0,
                "depth": 0,
                "bucketDepth": 0,
                "immutableFlag": true,
                "batchTTL": 0,
                "owner": "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd",
                "storageRadius": 0
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = BeeDebugStampsClient::new(&mock_server.uri()).unwrap();
    let result = client.get_global_postage_batches().await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.len(), 1);
    assert_eq!(
        response[0].batch_id,
        "0x1234567890123456789012345678901234567890123456789012345678901234"
    );
}

#[tokio::test]
async fn test_get_all_postage_batches() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex("/stamps"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "batchID": "0x1234567890123456789012345678901234567890123456789012345678901234",
                "utilization": 0,
                "usable": true,
                "label": "string",
                "depth": 0,
                "amount": "1000000000000000000",
                "bucketDepth": 0,
                "blockNumber": 0,
                "immutableFlag": true,
                "exists": true,
                "batchTTL": 0
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = BeeDebugStampsClient::new(&mock_server.uri()).unwrap();
    let result = client.get_all_postage_batches().await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.len(), 1);
    assert_eq!(
        response[0].batch_id,
        "0x1234567890123456789012345678901234567890123456789012345678901234"
    );
}

#[tokio::test]
async fn test_get_postage_batch() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex("/stamps/(.*)"))
        .respond_with(ResponseTemplate::new(200).set_body_json(PostageBatch {
            batch_id:
                "0x1234567890123456789012345678901234567890123456789012345678901234".to_string(),
            utilization: 100,
            usable: true,
            label: Some("test_label".to_string()).unwrap(),
            depth: 16,
            amount: "1000000000000000000".to_string(),
            bucket_depth: 16,
            block_number: 12345,
            immutable_flag: true,
            exists: true,
            batch_ttl: 3600,
        }))
        .mount(&mock_server)
        .await;

    let client = BeeDebugStampsClient::new(&mock_server.uri()).unwrap();
    let result = client
        .get_postage_batch("0x1234567890123456789012345678901234567890123456789012345678901234")
        .await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(
        response.batch_id,
        "0x1234567890123456789012345678901234567890123456789012345678901234"
    );
}

#[tokio::test]
async fn test_get_postage_batch_buckets() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex("/stamps/(.*)/buckets"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "depth": 16,
            "bucketDepth": 16,
            "bucketUpperBound": 100,
            "buckets": [
                {
                    "bucketId": 0,
                    "collisions": 0
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = BeeDebugStampsClient::new(&mock_server.uri()).unwrap();
    let result = client
        .get_postage_batch_buckets(
            "0x1234567890123456789012345678901234567890123456789012345678901234",
        )
        .await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.depth, 16);
}

#[tokio::test]
async fn test_create_postage_batch() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path_regex("/stamps/(.*)/(.*)"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "batchID": "0x1234567890123456789012345678901234567890123456789012345678901234"
        })))
        .mount(&mock_server)
        .await;

    let client = BeeDebugStampsClient::new(&mock_server.uri()).unwrap();
    let result = client
        .create_postage_batch("1000000000000000000", 16, None, None, None)
        .await;
    assert!(result.is_ok());
    let batch_id = result.unwrap();
    assert_eq!(
        batch_id,
        "0x1234567890123456789012345678901234567890123456789012345678901234"
    );
}

#[tokio::test]
async fn test_top_up_batch() {
    let mock_server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path_regex("/stamps/topup/(.*)/(.*)"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transactionHash": "0x1234567890123456789012345678901234567890123456789012345678901234"
        })))
        .mount(&mock_server)
        .await;

    let client = BeeDebugStampsClient::new(&mock_server.uri()).unwrap();
    let result = client
        .top_up_batch(
            "0x1234567890123456789012345678901234567890123456789012345678901234",
            "1000000000000000000",
        )
        .await;
    assert!(result.is_ok());
    let tx_hash = result.unwrap();
    assert_eq!(
        tx_hash,
        "0x1234567890123456789012345678901234567890123456789012345678901234"
    );
}

#[tokio::test]
async fn test_dilute_batch() {
    let mock_server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path_regex("/stamps/dilute/(.*)/(.*)"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "transactionHash": "0x1234567890123456789012345678901234567890123456789012345678901234"
        })))
        .mount(&mock_server)
        .await;

    let client = BeeDebugStampsClient::new(&mock_server.uri()).unwrap();
    let result = client
        .dilute_batch(
            "0x1234567890123456789012345678901234567890123456789012345678901234",
            17,
        )
        .await;
    assert!(result.is_ok());
    let tx_hash = result.unwrap();
    assert_eq!(
        tx_hash,
        "0x1234567890123456789012345678901234567890123456789012345678901234"
    );
}
