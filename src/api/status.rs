use reqwest::{Client, Error as ReqwestError};
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BeeClientError {
    #[error("Request error: {0}")]
    Reqwest(#[from] ReqwestError),
    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),
}

pub async fn check_connection(
    client: &Client,
    base_url: &str,
) -> Result<(), ReqwestError> {
    client.get(base_url).send().await?.error_for_status()?;
    Ok(())
}

pub async fn is_gateway(
    client: &Client,
    base_url: &str,
) -> Result<bool, ReqwestError> {
    #[derive(Deserialize)]
    struct GatewayResponse {
        gateway: bool,
    }

    let url = format!("{}/gateway", base_url);
    let response = client.get(&url).send().await?.error_for_status()?;
    let gateway_response: GatewayResponse = response.json().await?;
    Ok(gateway_response.gateway)
}