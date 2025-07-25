use reqwest::{Client, Error as ReqwestError};
use serde::{Deserialize, Serialize};
use url::{Url, ParseError as UrlParseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BeeDebugBalanceError {
    #[error("Request error: {0}")]
    Reqwest(#[from] ReqwestError),
    #[error("URL parse error: {0}")]
    UrlParse(#[from] UrlParseError),
    #[error("Deserialization error: {0}")]
    Deserialization(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerBalance {
    pub peer: String,
    pub balance: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceResponse {
    pub balances: Vec<PeerBalance>,
}

pub struct BeeDebugBalanceClient {
    client: Client,
    base_url: Url,
}

impl BeeDebugBalanceClient {
    pub fn new(base_url: &str) -> Result<Self, UrlParseError> {
        let base_url = Url::parse(base_url)?;
        let client = Client::new();
        Ok(Self { client, base_url })
    }

    pub async fn get_all_balances(&self) -> Result<BalanceResponse, BeeDebugBalanceError> {
        let url = self.base_url.join("balances")?;
        let response = self.client.get(url).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }

    pub async fn get_peer_balance(&self, address: &str) -> Result<PeerBalance, BeeDebugBalanceError> {
        let url = self.base_url.join(&format!("balances/{}", address))?;
        let response = self.client.get(url).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }

    pub async fn get_past_due_consumption_balances(&self) -> Result<BalanceResponse, BeeDebugBalanceError> {
        let url = self.base_url.join("consumed")?;
        let response = self.client.get(url).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }

    pub async fn get_past_due_consumption_peer_balance(&self, address: &str) -> Result<PeerBalance, BeeDebugBalanceError> {
        let url = self.base_url.join(&format!("consumed/{}", address))?;
        let response = self.client.get(url).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }
}
