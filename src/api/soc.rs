use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

use crate::api::bytes::UploadOptions;

const SOC_ENDPOINT: &str = "soc";

pub async fn upload(
    client: &Client,
    base_url: &str,
    owner: &str,
    identifier: &str,
    signature: &str,
    data: Vec<u8>,
    postage_batch_id: &str,
    options: Option<UploadOptions>,
) -> Result<UploadResult, Error> {
    let url = format!("{}/{}/{}/{}", base_url, SOC_ENDPOINT, owner, identifier);
    let mut request_builder = client.post(&url).body(data);

    request_builder = request_builder.header("content-type", "application/octet-stream");
    request_builder = request_builder.header("swarm-postage-batch-id", postage_batch_id);
    request_builder = request_builder.query(&[("sig", signature)]);

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

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResult {
    pub reference: String,
    #[serde(rename = "tagUid")]
    pub tag_uid: Option<u32>,
    #[serde(rename = "historyAddress")]
    pub history_address: Option<String>,
}
