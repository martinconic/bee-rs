use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const BZZ_ENDPOINT: &str = "bzz";

// Reusing UploadResult and DownloadOptions from bytes module
use crate::api::bytes::{DownloadOptions, RedundantUploadOptions, UploadResult};

#[derive(Debug, Serialize, Default)]
pub struct FileUploadOptions {
    #[serde(flatten)]
    pub redundant_upload_options: RedundantUploadOptions,
    pub size: Option<u64>,
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
}

#[derive(Debug, Serialize, Default)]
pub struct CollectionUploadOptions {
    #[serde(flatten)]
    pub redundant_upload_options: RedundantUploadOptions,
    #[serde(rename = "indexDocument")]
    pub index_document: Option<String>,
    #[serde(rename = "errorDocument")]
    pub error_document: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FileData {
    pub name: Option<String>,
    #[serde(rename = "tagUid")]
    pub tag_uid: Option<u32>,
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    #[serde(skip)] // Data is handled separately
    pub data: Vec<u8>,
}

#[derive(Debug, Serialize)]
pub struct CollectionEntry {
    pub path: String,
    pub size: u64,
}

pub async fn upload_file(
    client: &Client,
    base_url: &str,
    data: Vec<u8>,
    postage_batch_id: &str,
    name: Option<&str>,
    options: Option<FileUploadOptions>,
) -> Result<UploadResult, Error> {
    let url = format!("{}/{}", base_url, BZZ_ENDPOINT);
    let mut request_builder = client.post(&url).body(data);

    request_builder = request_builder.header("swarm-postage-batch-id", postage_batch_id);

    if let Some(opts) = options {
        if let Some(content_type) = opts.content_type {
            request_builder = request_builder.header("content-type", content_type);
        }
        if let Some(size) = opts.size {
            request_builder = request_builder.header("content-length", size.to_string());
        }

        // Handle RedundantUploadOptions
        let redundant_opts = opts.redundant_upload_options;
        if let Some(act) = redundant_opts.upload_options.act {
            request_builder = request_builder.header("swarm-act", act.to_string());
        }
        if let Some(act_history_address) = redundant_opts.upload_options.act_history_address {
            request_builder = request_builder.header("swarm-act-history-address", act_history_address);
        }
        if let Some(pin) = redundant_opts.upload_options.pin {
            request_builder = request_builder.header("swarm-pin", pin.to_string());
        }
        if let Some(encrypt) = redundant_opts.upload_options.encrypt {
            request_builder = request_builder.header("swarm-encrypt", encrypt.to_string());
        }
        if let Some(tag) = redundant_opts.upload_options.tag {
            request_builder = request_builder.header("swarm-tag", tag.to_string());
        }
        if let Some(deferred) = redundant_opts.upload_options.deferred {
            request_builder = request_builder.header("swarm-deferred", deferred.to_string());
        }
        if let Some(redundancy_level) = redundant_opts.redundancy_level {
            request_builder = request_builder.header("swarm-redundancy-level", redundancy_level.to_string());
        }
    }

    if let Some(n) = name {
        request_builder = request_builder.query(&[("name", n)]);
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

pub async fn download_file(
    client: &Client,
    base_url: &str,
    reference: &str,
    path: Option<&str>,
    options: Option<DownloadOptions>,
) -> Result<FileData, Error> {
    let mut url = format!("{}/{}/{}", base_url, BZZ_ENDPOINT, reference);
    if let Some(p) = path {
        url = format!("{}/{}", url, p);
    }

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
    let headers = response.headers().clone();
    let data = response.bytes().await?.to_vec();
    let name = headers
        .get("swarm-file-name")
        .and_then(|value| value.to_str().ok())
        .map(|s| s.to_string());
    let tag_uid = headers
        .get("swarm-tag")
        .and_then(|value| value.to_str().ok())
        .and_then(|s| s.parse::<u32>().ok());
    let content_type = headers
        .get("content-type")
        .and_then(|value| value.to_str().ok())
        .map(|s| s.to_string());

    Ok(FileData {
        name,
        tag_uid,
        content_type,
        data,
    })
}

// TODO: Implement upload_collection which involves tar archiving
// For now, download_file_readable is not implemented as it returns a stream
