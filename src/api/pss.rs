use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const ENDPOINT: &str = "pss";

pub async fn send(
    client: &Client,
    base_url: &str,
    topic: &str,
    target: &str,
    data: Vec<u8>,
    postage_batch_id: &str,
    recipient: Option<&str>,
) -> Result<(), Error> {
    let url = format!("{}/{}/send/{}/{}", base_url, ENDPOINT, topic, target);
    let mut request_builder = client.post(&url).body(data);

    request_builder = request_builder.header("swarm-postage-batch-id", postage_batch_id);

    if let Some(r) = recipient {
        request_builder = request_builder.query(&[("recipient", r)]);
    }

    request_builder.send().await?.error_for_status()?;
    Ok(())
}

pub fn subscribe(
    _url: &str,
    _topic: &str,
    _headers: Option<std::collections::HashMap<String, String>>,
) -> Result<(), String> {
    Err("WebSocket subscriptions are not directly supported in this API module.".to_string())
}
