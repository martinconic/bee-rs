use crate::common::test_utils::spawn_mock_bee_with_warps;
use bee_rs::api::soc::upload;
use bee_rs::api::bytes::UploadOptions;
use warp::{http::HeaderValue, Filter};

mod common;

#[tokio::test]
async fn test_upload_soc() {
    let owner = "0x1234567890123456789012345678901234567890";
    let identifier = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd";
    let signature = "0x112233445566778899aabbccddeeff00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff00";
    let data = vec![1, 2, 3, 4, 5];
    let postage_batch_id = "test_batch_id";
    let expected_reference = "36b7efd913ca4cf880b8eeac5093fa27b0825906c600685b6abdd6566e6cfe8f";
    let expected_tag_uid = 123;
    let expected_history_address = "some_history_address_soc";

    let mock_server = spawn_mock_bee_with_warps(vec![
        warp::post()
            .and(warp::path!("soc" / String / String))
            .and(warp::header::exact("content-type", "application/octet-stream"))
            .and(warp::header::exact("swarm-postage-batch-id", postage_batch_id))
            .and(warp::query::exact("sig", signature))
            .and(warp::header::optional::<String>("swarm-act"))
            .and(warp::header::optional::<String>("swarm-act-history-address"))
            .and(warp::header::optional::<String>("swarm-pin"))
            .and(warp::header::optional::<String>("swarm-encrypt"))
            .and(warp::header::optional::<String>("swarm-tag"))
            .and(warp::header::optional::<String>("swarm-deferred"))
            .and(warp::body::bytes())
            .map(move |owner_param: String, identifier_param: String, act: Option<String>, act_history_address: Option<String>, pin: Option<String>, encrypt: Option<String>, tag: Option<String>, deferred: Option<String>, body: bytes::Bytes| {
                assert_eq!(owner_param, owner);
                assert_eq!(identifier_param, identifier);
                assert_eq!(body.to_vec(), data);
                if let Some(act_val) = act { assert_eq!(act_val, "true"); }
                if let Some(act_history_address_val) = act_history_address { assert_eq!(act_history_address_val, "some_history_address_soc"); }
                if let Some(pin_val) = pin { assert_eq!(pin_val, "true"); }
                if let Some(encrypt_val) = encrypt { assert_eq!(encrypt_val, "true"); }
                if let Some(tag_val) = tag { assert_eq!(tag_val, "123"); }
                if let Some(deferred_val) = deferred { assert_eq!(deferred_val, "false"); }

                warp::reply::with_headers(
                    warp::reply::json(&serde_json::json!({ "reference": expected_reference })),
                    {
                        let mut headers = warp::http::HeaderMap::new();
                        headers.insert("swarm-tag", HeaderValue::from_str(&expected_tag_uid.to_string()).unwrap());
                        headers.insert("swarm-act-history-address", HeaderValue::from_str(expected_history_address).unwrap());
                        headers
                    },
                )
            }),
    ])
    .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();
    let options = Some(UploadOptions {
        act: Some(true),
        act_history_address: Some("some_history_address_soc".to_string()),
        pin: Some(true),
        encrypt: Some(true),
        tag: Some(expected_tag_uid),
        deferred: Some(false),
    });

    let result = upload(
        &client,
        base_url,
        owner,
        identifier,
        signature,
        data,
        postage_batch_id,
        options,
    )
    .await;

    assert!(result.is_ok());
    let upload_result = result.unwrap();
    assert_eq!(upload_result.reference, expected_reference);
    assert_eq!(upload_result.tag_uid, Some(expected_tag_uid));
    assert_eq!(upload_result.history_address, Some(expected_history_address.to_string()));
}
