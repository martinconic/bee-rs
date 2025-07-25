use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

use crate::api::bytes::{DownloadOptions, UploadOptions, UploadResult};

const ENDPOINT: &str = "chunks";

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvelopeWithBatchId {
    pub issuer: Vec<u8>,
    pub index: Vec<u8>,
    pub timestamp: Vec<u8>,
    pub signature: Vec<u8>,
    #[serde(rename = "batchId")]
    pub batch_id: String,
}

pub async fn upload(
    client: &Client,
    base_url: &str,
    data: Vec<u8>,
    postage_batch_id: &str,
    options: Option<UploadOptions>,
) -> Result<UploadResult, Error> {
    let url = format!("{}/{}", base_url, ENDPOINT);
    let mut request_builder = client.post(&url).body(data);

    request_builder = request_builder.header("content-type", "application/octet-stream");
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

    let tag_uid = response
        .headers()
        .get("swarm-tag")
        .and_then(|value| value.to_str().ok())
        .and_then(|s| s.parse::<u32>().ok());

    let history_address = response
        .headers()
        .get("swarm-act-history-address")
        .and_then(|value| value.to_str().ok())
        .map(|s| s.to_string());

    let mut upload_result: UploadResult = response.json().await?;
    upload_result.tag_uid = tag_uid;
    upload_result.history_address = history_address;

    Ok(upload_result)
}

pub async fn download(
    client: &Client,
    base_url: &str,
    reference: &str,
    options: Option<DownloadOptions>,
) -> Result<Vec<u8>, Error> {
    let url = format!("{}/{}/{}", base_url, ENDPOINT, reference);
    let mut request_builder = client.get(&url);

    if let Some(opts) = options {
        if let Some(redundancy_strategy) = opts.redundancy_strategy {
            request_builder = request_builder.header("swarm-redundancy-strategy", redundancy_strategy.to_string());
        }
        if let Some(fallback) = opts.fallback {
            request_builder = request_builder.header("swarm-fallback", fallback.to_string());
        }
        if let Some(timeout_ms) = opts.timeout_ms {
            request_builder = request_builder.header("swarm-timeout", timeout_ms.to_string());
        }
        if let Some(act_publisher) = opts.act_publisher {
            request_builder = request_builder.header("swarm-act-publisher", act_publisher);
        }
        if let Some(act_history_address) = opts.act_history_address {
            request_builder = request_builder.header("swarm-act-history-address", act_history_address);
        }
        if let Some(act_timestamp) = opts.act_timestamp {
            request_builder = request_builder.header("swarm-act-timestamp", act_timestamp.to_string());
        }
    }

    let response = request_builder.send().await?;
    let bytes = response.bytes().await?;

    Ok(bytes.to_vec())
}
