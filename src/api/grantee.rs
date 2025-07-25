use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const GRANTEE_ENDPOINT: &str = "grantee";

#[derive(Debug, Deserialize)]
pub struct GetGranteesResult {
    // status and statusText are not directly mapped from HTTP response in reqwest, 
    // but can be inferred from the Result<T, E> and response.status()
    pub grantees: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct GranteesResult {
    pub reference: String,
    #[serde(rename = "historyref")]
    pub history_reference: String,
}

#[derive(Debug, Serialize)]
pub struct CreateGranteesPayload {
    pub grantees: Vec<String>,
}

#[derive(Debug, Serialize, Default)]
pub struct PatchGranteesPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke: Option<Vec<String>>,
}

pub async fn get_grantees(
    client: &Client,
    base_url: &str,
    reference: &str,
) -> Result<GetGranteesResult, Error> {
    let url = format!("{}/{}/{}", base_url, GRANTEE_ENDPOINT, reference);
    let response = client.get(&url).send().await?;
    response.json().await
}

pub async fn create_grantees(
    client: &Client,
    base_url: &str,
    postage_batch_id: &str,
    grantees: Vec<String>,
) -> Result<GranteesResult, Error> {
    let url = format!("{}/{}", base_url, GRANTEE_ENDPOINT);
    let payload = CreateGranteesPayload { grantees };
    let response = client
        .post(&url)
        .header("swarm-postage-batch-id", postage_batch_id)
        .json(&payload)
        .send()
        .await?;
    response.json().await
}

pub async fn patch_grantees(
    client: &Client,
    base_url: &str,
    postage_batch_id: &str,
    reference: &str,
    history_reference: &str,
    add_grantees: Option<Vec<String>>,
    revoke_grantees: Option<Vec<String>>,
) -> Result<GranteesResult, Error> {
    let url = format!("{}/{}/{}", base_url, GRANTEE_ENDPOINT, reference);
    let payload = PatchGranteesPayload {
        add: add_grantees,
        revoke: revoke_grantees,
    };
    let response = client
        .patch(&url)
        .header("swarm-postage-batch-id", postage_batch_id)
        .header("swarm-act-history-address", history_reference)
        .json(&payload)
        .send()
        .await?;
    response.json().await
}
