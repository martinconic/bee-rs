use reqwest::{Client, Error as ReqwestError};
use serde::{Deserialize, Serialize};
use url::{Url, ParseError as UrlParseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BeeDebugStatesError {
    #[error("Request error: {0}")]
    Reqwest(#[from] ReqwestError),
    #[error("URL parse error: {0}")]
    UrlParse(#[from] UrlParseError),
    #[error("Failed to parse number: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Failed to parse BigUint: {0}")]
    ParseBigUint(String),
    #[error("Deserialization error: {0}")]
    Deserialization(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReserveState {
    pub commitment: u64,
    pub radius: u32,
    pub storage_radius: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainState {
    pub block: u64,
    pub chain_tip: u64,
    pub total_amount: String,
    pub current_price: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalance {
    pub bzz_balance: String,
    pub native_token_balance: String,
}

pub struct BeeDebugStatesClient {
    client: Client,
    base_url: Url,
}

impl BeeDebugStatesClient {
    pub fn new(base_url: &str) -> Result<Self, UrlParseError> {
        let base_url = Url::parse(base_url)?;
        let client = Client::new();
        Ok(Self { client, base_url })
    }

    pub async fn get_reserve_state(&self) -> Result<ReserveState, BeeDebugStatesError> {
        let url = self.base_url.join("reservestate")?;
        let response = self.client.get(url).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }

    pub async fn get_chain_state(&self) -> Result<ChainState, BeeDebugStatesError> {
        let url = self.base_url.join("chainstate")?;
        let response = self.client.get(url).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }

    pub async fn get_wallet_balance(&self) -> Result<WalletBalance, BeeDebugStatesError> {
        let url = self.base_url.join("wallet")?;
        let response = self.client.get(url).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }
}