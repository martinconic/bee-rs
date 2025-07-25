use bee_rs::api::debug::stamps::{BeeDebugStampsClient, GlobalPostageBatch, PostageBatch, PostageBatchBuckets, PostageBatchBucket};
use warp::Filter;
use serde_json;
use std::collections::HashMap;

#[tokio::test]
async fn test_get_global_postage_batches() {
    let route = warp::path!("batches").map(|| {
        warp::reply::json(&serde_json::json!([
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
        ]))
    });
    let (addr, server) = warp::serve(route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugStampsClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_global_postage_batches().await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.len(), 1);
    assert_eq!(response[0].batch_id, "0x1234567890123456789012345678901234567890123456789012345678901234");
}

#[tokio::test]
async fn test_get_all_postage_batches() {
    let route = warp::path!("stamps").map(|| {
        warp::reply::json(&serde_json::json!([
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
        ]))
    });
    let (addr, server) = warp::serve(route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugStampsClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_all_postage_batches().await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.len(), 1);
    assert_eq!(response[0].batch_id, "0x1234567890123456789012345678901234567890123456789012345678901234");
}

#[tokio::test]
async fn test_get_postage_batch() {
    let route = warp::path!("stamps" / String).map(|batch_id: String| {
        warp::reply::json(&PostageBatch {
            batch_id,
            utilization: 100,
            usable: true,
            label: Some("test_label".to_string()),
            depth: 16,
            amount: "1000000000000000000".to_string(),
            bucket_depth: 16,
            block_number: 12345,
            immutable_flag: true,
            exists: true,
            batch_ttl: 3600,
        })
    });
    let (addr, server) = warp::serve(route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugStampsClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_postage_batch("0x1234567890123456789012345678901234567890123456789012345678901234").await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.batch_id, "0x1234567890123456789012345678901234567890123456789012345678901234");
}

#[tokio::test]
async fn test_get_postage_batch_buckets() {
    let route = warp::path!("stamps" / String / "buckets").map(|_batch_id: String| {
        warp::reply::json(&PostageBatchBuckets {
            depth: 16,
            bucket_depth: 16,
            bucket_upper_bound: 100,
            buckets: vec![
                PostageBatchBucket {
                    bucket_id: 0,
                    collisions: 0,
                },
            ],
        })
    });
    let (addr, server) = warp::serve(route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugStampsClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.get_postage_batch_buckets("0x1234567890123456789012345678901234567890123456789012345678901234").await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.depth, 16);
}

#[tokio::test]
async fn test_create_postage_batch() {
    let route = warp::path!("stamps" / String / u32).and(warp::post()).and(warp::query::query()).and(warp::header::optional("gas-price")).and(warp::header::optional("immutable")).map(|_amount: String, _depth: u32, _query: HashMap<String, String>, _gas_price: Option<String>, _immutable: Option<String>| {
        warp::reply::json(&serde_json::json!({
            "batchID": "0x1234567890123456789012345678901234567890123456789012345678901234"
        }))
    });
    let (addr, server) = warp::serve(route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugStampsClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.create_postage_batch("1000000000000000000", 16, None, None, None).await;
    assert!(result.is_ok());
    let batch_id = result.unwrap();
    assert_eq!(batch_id, "0x1234567890123456789012345678901234567890123456789012345678901234");
}

#[tokio::test]
async fn test_top_up_batch() {
    let route = warp::path!("stamps" / "topup" / String / String).and(warp::patch()).map(|_id: String, _amount: String| {
        warp::reply::json(&serde_json::json!({
            "batchID": "0x1234567890123456789012345678901234567890123456789012345678901234"
        }))
    });
    let (addr, server) = warp::serve(route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugStampsClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.top_up_batch("0x1234567890123456789012345678901234567890123456789012345678901234", "1000000000000000000").await;
    assert!(result.is_ok());
    let batch_id = result.unwrap();
    assert_eq!(batch_id, "0x1234567890123456789012345678901234567890123456789012345678901234");
}

#[tokio::test]
async fn test_dilute_batch() {
    let route = warp::path!("stamps" / "dilute" / String / u32).and(warp::patch()).map(|_id: String, _depth: u32| {
        warp::reply::json(&serde_json::json!({
            "batchID": "0x1234567890123456789012345678901234567890123456789012345678901234"
        }))
    });
    let (addr, server) = warp::serve(route).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let client = BeeDebugStampsClient::new(&format!("http://{}:{}", addr.ip(), addr.port())).unwrap();
    let result = client.dilute_batch("0x1234567890123456789012345678901234567890123456789012345678901234", 17).await;
    assert!(result.is_ok());
    let batch_id = result.unwrap();
    assert_eq!(batch_id, "0x1234567890123456789012345678901234567890123456789012345678901234");
}