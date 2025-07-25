use reqwest::{Client, Error as ReqwestError};
use serde::{Deserialize, Serialize};
use url::{Url, ParseError as UrlParseError};
use thiserror::Error;
use std::collections::HashMap;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::Value;

#[derive(Error, Debug)]
pub enum BeeDebugStampsError {
    #[error("Request error: {0}")]
    Reqwest(#[from] ReqwestError),
    #[error("URL parse error: {0}")]
    UrlParse(#[from] UrlParseError),
    #[error("Deserialization error: {0}")]
    Deserialization(String),
    #[error("Header error: {0}")]
    Header(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GlobalPostageBatch {
    pub batch_id: String,
    pub value: String,
    pub start: u32,
    pub depth: u32,
    pub bucket_depth: u32,
    pub immutable_flag: bool,
    pub batch_ttl: u32,
    pub owner: String,
    pub storage_radius: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostageBatch {
    pub batch_id: String,
    pub utilization: u32,
    pub usable: bool,
    pub label: String,
    pub depth: u32,
    pub amount: String,
    pub bucket_depth: u32,
    pub block_number: u32,
    pub immutable_flag: bool,
    pub exists: bool,
    pub batch_ttl: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostageBatchBucket {
    pub bucket_id: u32,
    pub collisions: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostageBatchBuckets {
    pub depth: u32,
    pub bucket_depth: u32,
    pub bucket_upper_bound: u32,
    pub buckets: Vec<PostageBatchBucket>,
}

pub struct BeeDebugStampsClient {
    client: Client,
    base_url: Url,
}

impl BeeDebugStampsClient {
    pub fn new(base_url: &str) -> Result<Self, UrlParseError> {
        let base_url = Url::parse(base_url)?;
        let client = Client::new();
        Ok(Self { client, base_url })
    }

    pub async fn get_global_postage_batches(&self) -> Result<Vec<GlobalPostageBatch>, BeeDebugStampsError> {
        let url = self.base_url.join("batches")?;
        let response = self.client.get(url).send().await?.error_for_status()?;
        let body: serde_json::Value = response.json().await?;
        let batches = serde_json::from_value(body["batches"].clone())
            .map_err(|e| BeeDebugStampsError::Deserialization(format!("Failed to deserialize global postage batches: {}", e)))?;
        Ok(batches)
    }

    pub async fn get_all_postage_batches(&self) -> Result<Vec<PostageBatch>, BeeDebugStampsError> {
        let url = self.base_url.join("stamps")?;
        let response = self.client.get(url).send().await?.error_for_status()?;
        let body: serde_json::Value = response.json().await?;
        let stamps = serde_json::from_value(body["stamps"].clone())
            .map_err(|e| BeeDebugStampsError::Deserialization(format!("Failed to deserialize all postage batches: {}", e)))?;
        Ok(stamps)
    }

    pub async fn get_postage_batch(&self, postage_batch_id: &str) -> Result<PostageBatch, BeeDebugStampsError> {
        let url = self.base_url.join(&format!("stamps/{}", postage_batch_id))?;
        let response = self.client.get(url).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }

    pub async fn get_postage_batch_buckets(&self, postage_batch_id: &str) -> Result<PostageBatchBuckets, BeeDebugStampsError> {
        let url = self.base_url.join(&format!("stamps/{}/buckets", postage_batch_id))?;
        let response = self.client.get(url).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }

    pub async fn create_postage_batch(&self, amount: &str, depth: u32, gas_price: Option<&str>, immutable_flag: Option<bool>, label: Option<&str>) -> Result<String, BeeDebugStampsError> {
        let url = self.base_url.join(&format!("stamps/{}/{}", amount, depth))?;
        let mut request = self.client.post(url);

        let mut headers = HashMap::new();
        if let Some(gp) = gas_price {
            headers.insert("gas-price".to_string(), gp.to_string());
        }
        if let Some(im) = immutable_flag {
            headers.insert("immutable".to_string(), im.to_string());
        }

        let mut header_map = reqwest::header::HeaderMap::new();
        for (key, value) in headers {
            header_map.insert(reqwest::header::HeaderName::from_bytes(key.as_bytes()).map_err(|e| BeeDebugStampsError::Header(format!("Invalid header name: {}", e)))?,
                              reqwest::header::HeaderValue::from_str(&value).map_err(|e| BeeDebugStampsError::Header(format!("Invalid header value: {}", e)))?);
        }
        request = request.headers(header_map);

        if let Some(l) = label {
            request = request.query(&[("label", l)]);
        }

        let response = request.send().await?.error_for_status()?;
        let body: serde_json::Value = response.json().await?;
        Ok(body["batchID"].as_str().unwrap_or_default().to_string())
    }

    pub async fn top_up_batch(&self, id: &str, amount: &str) -> Result<String, BeeDebugStampsError> {
        let url = self.base_url.join(&format!("stamps/topup/{}/{}", id, amount))?;
        let response = self.client.patch(url).send().await?.error_for_status()?;
        let body: serde_json::Value = response.json().await?;
        Ok(body["batchID"].as_str().unwrap_or_default().to_string())
    }

    pub async fn dilute_batch(&self, id: &str, depth: u32) -> Result<String, BeeDebugStampsError> {
        let url = self.base_url.join(&format!("stamps/dilute/{}/{}", id, depth))?;
        let response = self.client.patch(url).send().await?.error_for_status()?;
        let body: serde_json::Value = response.json().await?;
        Ok(body["batchID"].as_str().unwrap_or_default().to_string())
    }
}
