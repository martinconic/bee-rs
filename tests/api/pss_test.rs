use bee_rs::api::pss::send;
use wiremock::{matchers::{method, path_regex, query_param, header}, Mock, MockServer, ResponseTemplate};
use reqwest::StatusCode;

#[tokio::test]
async fn test_send_pss() {
    let topic = "test_topic";
    let target = "test_target";
    let data = vec![1, 2, 3, 4, 5];
    let postage_batch_id = "test_batch_id";
    let recipient = Some("test_recipient");

    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path_regex(&format!("/pss/send/{}/{}", topic, target)))
        .and(header("swarm-postage-batch-id", postage_batch_id))
        .and(query_param("recipient", recipient.clone().unwrap()))
        .respond_with(ResponseTemplate::new(StatusCode::OK))
        .mount(&mock_server)
        .await;

    let client = reqwest::Client::new();
    let base_url = &mock_server.uri();

    let result = send(&client, base_url, topic, target, data, postage_batch_id, recipient.as_deref()).await;

    assert!(result.is_ok());
}
