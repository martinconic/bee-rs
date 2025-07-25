// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_rs::bee::Bee;
use wiremock::{matchers::method, Mock, MockServer, ResponseTemplate};

const UNKNOWN_REFERENCE: &str = "1000000000000000000000000000000000000000000000000000000000000000";

#[tokio::test]
async fn test_stewardship_reupload() {
    let mock_server = MockServer::start().await;
    let bee = Bee::new(&mock_server.uri());

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    assert!(bee.stewardship().reupload(UNKNOWN_REFERENCE).await.is_ok());
}

#[tokio::test]
async fn test_stewardship_is_retrievable() {
    let mock_server = MockServer::start().await;
    let bee = Bee::new(&mock_server.uri());

    Mock::given(method("PUT"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    assert!(bee.stewardship().is_retrievable(UNKNOWN_REFERENCE).await.unwrap());
}

#[tokio::test]
async fn test_stewardship_is_not_retrievable() {
    let mock_server = MockServer::start().await;
    let bee = Bee::new(&mock_server.uri());

    Mock::given(method("PUT"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    assert!(!bee.stewardship().is_retrievable(UNKNOWN_REFERENCE).await.unwrap());
}