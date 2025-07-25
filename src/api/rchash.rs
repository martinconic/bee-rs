use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

const RCHASH_ENDPOINT: &str = "rchash";

#[derive(Debug, Deserialize)]
pub struct RchashResponse {
    #[serde(rename = "durationSeconds")]
    pub duration_seconds: f64,
}

pub async fn rchash(
    client: &Client,
    base_url: &str,
    depth: u32,
    anchor1: &str,
    anchor2: &str,
) -> Result<f64, Error> {
    let url = format!("{}/{}/{}/{}/{}", base_url, RCHASH_ENDPOINT, depth, anchor1, anchor2);
    let response = client.get(&url).send().await?;
    let rchash_response: RchashResponse = response.json().await?;
    Ok(rchash_response.duration_seconds)
}
