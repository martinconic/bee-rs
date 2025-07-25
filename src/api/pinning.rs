use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const PINNING_ENDPOINT: &str = "pins";

#[derive(Debug, Deserialize)]
pub struct Pin {
    pub reference: String,
}

#[derive(Debug, Deserialize)]
pub struct AllPinsResponse {
    pub references: Option<Vec<String>>,
}

pub async fn pin(
    client: &Client,
    base_url: &str,
    reference: &str,
) -> Result<(), Error> {
    let url = format!("{}/{}/{}", base_url, PINNING_ENDPOINT, reference);
    client.post(&url).send().await?.error_for_status()?;
    Ok(())
}

pub async fn unpin(
    client: &Client,
    base_url: &str,
    reference: &str,
) -> Result<(), Error> {
    let url = format!("{}/{}/{}", base_url, PINNING_ENDPOINT, reference);
    client.delete(&url).send().await?.error_for_status()?;
    Ok(())
}

pub async fn get_pin(
    client: &Client,
    base_url: &str,
    reference: &str,
) -> Result<Pin, Error> {
    let url = format!("{}/{}/{}", base_url, PINNING_ENDPOINT, reference);
    let response = client.get(&url).send().await?;
    response.json().await
}

pub async fn get_all_pins(
    client: &Client,
    base_url: &str,
) -> Result<Vec<String>, Error> {
    let url = format!("{}/{}", base_url, PINNING_ENDPOINT);
    let response = client.get(&url).send().await?;
    let all_pins_response: AllPinsResponse = response.json().await?;
    Ok(all_pins_response.references.unwrap_or_default())
}
