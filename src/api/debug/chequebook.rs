use reqwest::{Client, Error as ReqwestError};
use serde::{Deserialize, Serialize};
use url::{Url, ParseError as UrlParseError};
use thiserror::Error;
use std::collections::HashMap;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

#[derive(Error, Debug)]
pub enum BeeDebugChequebookError {
    #[error("Request error: {0}")]
    Reqwest(#[from] ReqwestError),
    #[error("URL parse error: {0}")]
    UrlParse(#[from] UrlParseError),
    #[error("Deserialization error: {0}")]
    Deserialization(String),
    #[error("Header error: {0}")]
    Header(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChequebookAddressResponse {
    pub chequebook_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChequebookBalanceResponse {
    pub total_balance: String,
    pub available_balance: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cheque {
    pub beneficiary: String,
    pub chequebook: String,
    pub payout: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CashoutResult {
    pub recipient: String,
    pub last_payout: String,
    pub bounced: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastCashoutActionResponse {
    pub peer: String,
    pub uncashed_amount: String,
    pub transaction_hash: Option<String>,
    pub last_cashed_cheque: Option<Cheque>,
    pub result: Option<CashoutResult>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastChequesForPeerResponse {
    pub peer: String,
    pub lastreceived: Option<Cheque>,
    pub lastsent: Option<Cheque>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastChequesResponse {
    pub lastcheques: Vec<LastChequesForPeerResponse>,
}

pub struct BeeDebugChequebookClient {
    client: Client,
    base_url: Url,
}

impl BeeDebugChequebookClient {
    pub fn new(base_url: &str) -> Result<Self, UrlParseError> {
        let base_url = Url::parse(base_url)?;
        let client = Client::new();
        Ok(Self { client, base_url })
    }

    pub async fn get_chequebook_address(&self) -> Result<ChequebookAddressResponse, BeeDebugChequebookError> {
        let url = self.base_url.join("chequebook/address")?;
        let response = self.client.get(url).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }

    pub async fn get_chequebook_balance(&self) -> Result<ChequebookBalanceResponse, BeeDebugChequebookError> {
        let url = self.base_url.join("chequebook/balance")?;
        let response = self.client.get(url).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }

    pub async fn get_last_cashout_action(&self, peer: &str) -> Result<LastCashoutActionResponse, BeeDebugChequebookError> {
        let url = self.base_url.join(&format!("chequebook/cashout/{}", peer))?;
        let response = self.client.get(url).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }

    pub async fn cashout_last_cheque(&self, peer: &str, gas_price: Option<&str>, gas_limit: Option<&str>) -> Result<String, BeeDebugChequebookError> {
        let url = self.base_url.join(&format!("chequebook/cashout/{}", peer))?;
        let mut request = self.client.post(url);

        let mut headers = HeaderMap::new();
        if let Some(gp) = gas_price {
            headers.insert(HeaderName::from_static("gas-price"), HeaderValue::from_str(gp).map_err(|e| BeeDebugChequebookError::Header(format!("Invalid gas-price header: {}", e)))?);
        }
        if let Some(gl) = gas_limit {
            headers.insert(HeaderName::from_static("gas-limit"), HeaderValue::from_str(gl).map_err(|e| BeeDebugChequebookError::Header(format!("Invalid gas-limit header: {}", e)))?);
        }

        request = request.headers(headers);

        let response = request.send().await?.error_for_status()?;
        let body: serde_json::Value = response.json().await?;
        Ok(body["transactionHash"].as_str().unwrap_or_default().to_string())
    }

    pub async fn get_last_cheques_for_peer(&self, peer: &str) -> Result<LastChequesForPeerResponse, BeeDebugChequebookError> {
        let url = self.base_url.join(&format!("chequebook/cheque/{}", peer))?;
        let response = self.client.get(url).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }

    pub async fn get_last_cheques(&self) -> Result<LastChequesResponse, BeeDebugChequebookError> {
        let url = self.base_url.join("chequebook/cheque")?;
        let response = self.client.get(url).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }

    pub async fn deposit_tokens(&self, amount: &str, gas_price: Option<&str>) -> Result<String, BeeDebugChequebookError> {
        let url = self.base_url.join("chequebook/deposit")?;
        let mut request = self.client.post(url);

        let mut params = HashMap::new();
        params.insert("amount", amount);
        request = request.query(&params);

        let mut headers = HeaderMap::new();
        if let Some(gp) = gas_price {
            headers.insert(HeaderName::from_static("gas-price"), HeaderValue::from_str(gp).map_err(|e| BeeDebugChequebookError::Header(format!("Invalid gas-price header: {}", e)))?);
        }
        request = request.headers(headers);

        let response = request.send().await?.error_for_status()?;
        let body: serde_json::Value = response.json().await?;
        Ok(body["transactionHash"].as_str().unwrap_or_default().to_string())
    }

    pub async fn withdraw_tokens(&self, amount: &str, gas_price: Option<&str>) -> Result<String, BeeDebugChequebookError> {
        let url = self.base_url.join("chequebook/withdraw")?;
        let mut request = self.client.post(url);

        let mut params = HashMap::new();
        params.insert("amount", amount);
        request = request.query(&params);

        let mut headers = HeaderMap::new();
        if let Some(gp) = gas_price {
            headers.insert(HeaderName::from_static("gas-price"), HeaderValue::from_str(gp).map_err(|e| BeeDebugChequebookError::Header(format!("Invalid gas-price header: {}", e)))?);
        }
        request = request.headers(headers);

        let response = request.send().await?.error_for_status()?;
        let body: serde_json::Value = response.json().await?;
        Ok(body["transactionHash"].as_str().unwrap_or_default().to_string())
    }
}