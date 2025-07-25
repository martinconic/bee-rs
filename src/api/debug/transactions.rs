use reqwest::{Client, Error as ReqwestError};
use serde::{Deserialize, Serialize};
use url::{Url, ParseError as UrlParseError};
use thiserror::Error;
use std::collections::HashMap;

#[derive(Error, Debug)]
pub enum BeeDebugTransactionsError {
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
pub struct TransactionInfo {
    pub transaction_hash: String,
    #[serde(rename = "to")]
    pub to_address: String,
    pub nonce: u64,
    pub gas_price: String,
    pub gas_limit: u64,
    pub data: String,
    pub created: String,
    pub description: String,
    pub value: String,
}

pub struct BeeDebugTransactionsClient {
    client: Client,
    base_url: Url,
}

impl BeeDebugTransactionsClient {
    pub fn new(base_url: &str) -> Result<Self, UrlParseError> {
        let base_url = Url::parse(base_url)?;
        let client = Client::new();
        Ok(Self { client, base_url })
    }

    pub async fn get_all_transactions(&self) -> Result<Vec<TransactionInfo>, BeeDebugTransactionsError> {
        let url = self.base_url.join("transactions")?;
        let response = self.client.get(url).send().await?.error_for_status()?;
        let body: HashMap<String, Vec<TransactionInfo>> = response.json().await?;
        Ok(body.get("pendingTransactions").cloned().unwrap_or_default())
    }

    pub async fn get_transaction(&self, transaction_hash: &str) -> Result<TransactionInfo, BeeDebugTransactionsError> {
        let url = self.base_url.join(&format!("transactions/{}", transaction_hash))?;
        let response = self.client.get(url).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }

    pub async fn rebroadcast_transaction(&self, transaction_hash: &str) -> Result<String, BeeDebugTransactionsError> {
        let url = self.base_url.join(&format!("transactions/{}", transaction_hash))?;
        let response = self.client.post(url).send().await?.error_for_status()?;
        let body: serde_json::Value = response.json().await?;
        Ok(body["transactionHash"].as_str().unwrap_or_default().to_string())
    }

    pub async fn cancel_transaction(&self, transaction_hash: &str, gas_price: Option<&str>) -> Result<String, BeeDebugTransactionsError> {
        let url = self.base_url.join(&format!("transactions/{}", transaction_hash))?;
        let mut request = self.client.delete(url);

        let mut headers = HashMap::new();
        if let Some(gp) = gas_price {
            headers.insert("gas-price", gp);
        }
        // reqwest::header::HeaderMap does not implement From<HashMap<String, String>> directly
        let mut header_map = reqwest::header::HeaderMap::new();
        for (key, value) in headers {
            header_map.insert(reqwest::header::HeaderName::from_bytes(key.as_bytes()).map_err(|e| BeeDebugTransactionsError::Header(format!("Invalid header name: {}", e)))?,
                              reqwest::header::HeaderValue::from_str(value).map_err(|e| BeeDebugTransactionsError::Header(format!("Invalid header value: {}", e)))?);
        }
        request = request.headers(header_map);

        let response = request.send().await?.error_for_status()?;
        let body: serde_json::Value = response.json().await?;
        Ok(body["transactionHash"].as_str().unwrap_or_default().to_string())
    }
}
