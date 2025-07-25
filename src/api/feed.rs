use crate::error::Error;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::api::bytes::UploadOptions;

const FEED_ENDPOINT: &str = "feeds";

#[derive(Debug, Serialize, Default)]
pub struct FeedUpdateOptions {
    pub at: Option<u64>,
    pub index: Option<String>,
    #[serde(rename = "hasTimestamp")]
    pub has_timestamp: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct FeedUpdateHeaders {
    #[serde(rename = "swarm-feed-index")]
    pub feed_index: String,
    #[serde(rename = "swarm-feed-index-next")]
    pub feed_index_next: String,
}

#[derive(Debug, Deserialize)]
pub struct FeedPayloadResult {
    #[serde(flatten)]
    pub headers: FeedUpdateHeaders,
    #[serde(skip)]
    pub payload: Vec<u8>,
}

#[derive(Debug, Deserialize)]
pub struct FeedReferenceResult {
    #[serde(flatten)]
    pub headers: FeedUpdateHeaders,
    pub reference: String,
}

pub async fn create_feed_manifest(
    client: &Client,
    base_url: &str,
    owner: &str,
    topic: &str,
    postage_batch_id: &str,
    options: Option<UploadOptions>,
) -> Result<String, Error> {
    let url = format!("{}/{}/{}/{}", base_url, FEED_ENDPOINT, owner, topic);
    let mut request_builder = client.post(&url);

    request_builder = request_builder.header("swarm-postage-batch-id", postage_batch_id);

    if let Some(opts) = options {
        if let Some(act) = opts.act {
            request_builder = request_builder.header("swarm-act", act.to_string());
        }
        if let Some(act_history_address) = opts.act_history_address {
            request_builder = request_builder.header("swarm-act-history-address", act_history_address);
        }
        if let Some(pin) = opts.pin {
            request_builder = request_builder.header("swarm-pin", pin.to_string());
        }
        if let Some(encrypt) = opts.encrypt {
            request_builder = request_builder.header("swarm-encrypt", encrypt.to_string());
        }
        if let Some(tag) = opts.tag {
            request_builder = request_builder.header("swarm-tag", tag.to_string());
        }
        if let Some(deferred) = opts.deferred {
            request_builder = request_builder.header("swarm-deferred", deferred.to_string());
        }
    }

    let response = request_builder.send().await?;
    let body: HashMap<String, String> = response.json().await?;

    body.get("reference")
        .cloned()
        .ok_or_else(|| Error::Custom("Missing reference in response".to_string()))
}

pub async fn fetch_latest_feed_update(
    client: &Client,
    base_url: &str,
    owner: &str,
    topic: &str,
    options: Option<FeedUpdateOptions>,
) -> Result<FeedPayloadResult, Error> {
    let url = format!("{}/{}/{}/{}", base_url, FEED_ENDPOINT, owner, topic);
    let mut request_builder = client.get(&url);

    if let Some(opts) = options {
        request_builder = request_builder.query(&opts);
    }

    let response = request_builder.send().await?;

    let headers = response.headers();
    let feed_index = headers
        .get("swarm-feed-index")
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| Error::Custom("Missing swarm-feed-index header".to_string()))?
        .to_string();
    let feed_index_next = headers
        .get("swarm-feed-index-next")
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| Error::Custom("Missing swarm-feed-index-next header".to_string()))?
        .to_string();

    let payload = response.bytes().await?.to_vec();

    Ok(FeedPayloadResult {
        headers: FeedUpdateHeaders {
            feed_index,
            feed_index_next,
        },
        payload,
    })
}

pub async fn probe_feed(
    client: &Client,
    base_url: &str,
    owner: &str,
    topic: &str,
) -> Result<FeedUpdateHeaders, Error> {
    let url = format!("{}/{}/{}/{}", base_url, FEED_ENDPOINT, owner, topic);
    let request_builder = client.get(&url).header("Swarm-Only-Root-Chunk", "true");

    let response = request_builder.send().await?;

    let headers = response.headers();
    let feed_index = headers
        .get("swarm-feed-index")
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| Error::Custom("Missing swarm-feed-index header".to_string()))?
        .to_string();
    let feed_index_next = headers
        .get("swarm-feed-index-next")
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| Error::Custom("Missing swarm-feed-index-next header".to_string()))?
        .to_string();

    Ok(FeedUpdateHeaders {
        feed_index,
        feed_index_next,
    })
}
