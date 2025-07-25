use reqwest::{Client, Error as ReqwestError};
use serde::{Deserialize, Serialize};
use url::{Url, ParseError as UrlParseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BeeDebugSettlementsError {
    #[error("Request error: {0}")]
    Reqwest(#[from] ReqwestError),
    #[error("URL parse error: {0}")]
    UrlParse(#[from] UrlParseError),
    #[error("Deserialization error: {0}")]
    Deserialization(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settlements {
    pub peer: String,
    pub received: String,
    pub sent: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllSettlements {
    pub total_received: String,
    pub total_sent: String,
    pub settlements: Vec<Settlements>,
}

pub struct BeeDebugSettlementsClient {
    client: Client,
    base_url: Url,
}

impl BeeDebugSettlementsClient {
    pub fn new(base_url: &str) -> Result<Self, UrlParseError> {
        let base_url = Url::parse(base_url)?;
        let client = Client::new();
        Ok(Self { client, base_url })
    }

    pub async fn get_settlements(&self, peer: &str) -> Result<Settlements, BeeDebugSettlementsError> {
        let url = self.base_url.join(&format!("settlements/{}", peer))?;
        let response = self.client.get(url).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }

    pub async fn get_all_settlements(&self) -> Result<AllSettlements, BeeDebugSettlementsError> {
        let url = self.base_url.join("settlements")?;
        let response = self.client.get(url).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }
}
