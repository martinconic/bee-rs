use bee_rs::api::soc::upload;
use bee_rs::api::bytes::UploadOptions;
use wiremock::{matchers::{method, path_regex, header, query_param}, Mock, MockServer, ResponseTemplate};
use serde_json;

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

    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path_regex(&format!("/soc/{}/{}", owner, identifier)))
        .and(header("content-type", "application/octet-stream"))
        .and(header("swarm-postage-batch-id", postage_batch_id))
        .and(query_param("sig", signature))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "reference": expected_reference,
            "tagUid": expected_tag_uid,
            "historyAddress": expected_history_address
        })))
        .mount(&mock_server)
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
