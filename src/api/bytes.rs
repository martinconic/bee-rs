use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

const ENDPOINT: &str = "bytes";

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResult {
    pub reference: String,
    #[serde(rename = "tagUid")]
    pub tag_uid: Option<u32>,
    #[serde(rename = "historyAddress")]
    pub history_address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UploadOptions {
    pub act: Option<bool>,
    #[serde(rename = "actHistoryAddress")]
    pub act_history_address: Option<String>,
    pub pin: Option<bool>,
    pub encrypt: Option<bool>,
    pub tag: Option<u32>,
    pub deferred: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RedundantUploadOptions {
    #[serde(flatten)]
    pub upload_options: UploadOptions,
    #[serde(rename = "redundancyLevel")]
    pub redundancy_level: Option<u8>,
}

pub async fn upload(
    client: &Client,
    base_url: &str,
    data: Vec<u8>,
    postage_batch_id: &str,
    options: Option<RedundantUploadOptions>,
) -> Result<UploadResult, Error> {
    let url = format!("{}/{}", base_url, ENDPOINT);
    let mut request_builder = client.post(&url).body(data);

    request_builder = request_builder.header("content-type", "application/octet-stream");
    request_builder = request_builder.header("swarm-postage-batch-id", postage_batch_id);

    if let Some(opts) = options {
        if let Some(act) = opts.upload_options.act {
            request_builder = request_builder.header("swarm-act", act.to_string());
        }
        if let Some(act_history_address) = opts.upload_options.act_history_address {
            request_builder = request_builder.header("swarm-act-history-address", act_history_address);
        }
        if let Some(pin) = opts.upload_options.pin {
            request_builder = request_builder.header("swarm-pin", pin.to_string());
        }
        if let Some(encrypt) = opts.upload_options.encrypt {
            request_builder = request_builder.header("swarm-encrypt", encrypt.to_string());
        }
        if let Some(tag) = opts.upload_options.tag {
            request_builder = request_builder.header("swarm-tag", tag.to_string());
        }
        if let Some(deferred) = opts.upload_options.deferred {
            request_builder = request_builder.header("swarm-deferred", deferred.to_string());
        }
        if let Some(redundancy_level) = opts.redundancy_level {
            request_builder = request_builder.header("swarm-redundancy-level", redundancy_level.to_string());
        }
    }

    let response = request_builder.send().await?;

    // Check for swarm-tag and swarm-act-history-address headers
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ReferenceInformation {
    #[serde(rename = "contentLength")]
    pub content_length: u64,
}

pub async fn head(
    client: &Client,
    base_url: &str,
    reference: &str,
) -> Result<ReferenceInformation, Error> {
    let url = format!("{}/{}/{}", base_url, ENDPOINT, reference);
    let response = client.head(&url).send().await?;

    let content_length = response
        .headers()
        .get("content-length")
        .and_then(|value| value.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);

    Ok(ReferenceInformation { content_length })
}

pub async fn download(
    client: &Client,
    base_url: &str,
    resource: &str,
    options: Option<DownloadOptions>,
) -> Result<Vec<u8>, Error> {
    let url = format!("{}/{}/{}", base_url, ENDPOINT, resource);
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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DownloadOptions {
    #[serde(rename = "redundancyStrategy")]
    pub redundancy_strategy: Option<u8>,
    pub fallback: Option<bool>,
    #[serde(rename = "timeoutMs")]
    pub timeout_ms: Option<u32>,
    #[serde(rename = "actPublisher")]
    pub act_publisher: Option<String>,
    #[serde(rename = "actHistoryAddress")]
    pub act_history_address: Option<String>,
    #[serde(rename = "actTimestamp")]
    pub act_timestamp: Option<u64>,
}

// downloadReadable is not directly translatable to a simple Rust function returning a stream
// as reqwest::Response already provides a stream of bytes. The `download` function above
// effectively covers the use case of downloading the entire content.
// If a streaming interface is strictly required, it would involve returning `reqwest::Response`
// and letting the caller handle the stream, or using a custom stream type.
