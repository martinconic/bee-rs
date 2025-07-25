use reqwest::{Client, Error as ReqwestError};
use serde::{Deserialize, Serialize};
use url::{Url, ParseError as UrlParseError};
use thiserror::Error;
use std::collections::HashMap;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::Value;

#[derive(Error, Debug)]
pub enum BeeDebugConnectivityError {
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
pub struct NodeAddresses {
    pub overlay: String,
    pub underlay: Vec<String>,
    pub ethereum: String,
    pub public_key: String,
    pub pss_public_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Peer {
    pub address: String,
    #[serde(default)]
    pub full_node: Option<bool>,
    #[serde(default)]
    pub metrics: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemovePeerResponse {
    pub message: String,
    pub code: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Bin {
    pub population: u32,
    pub connected: u32,
    pub disconnected_peers: Vec<Peer>,
    pub connected_peers: Vec<Peer>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Topology {
    pub base_addr: String,
    pub population: u32,
    pub connected: u32,
    pub timestamp: String,
    pub nn_low_watermark: u32,
    pub depth: u32,
    pub reachability: String,
    pub network_availability: String,
    pub bins: HashMap<String, Bin>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PingResponse {
    pub rtt: String,
}

pub struct BeeDebugConnectivityClient {
    client: Client,
    base_url: Url,
}

impl BeeDebugConnectivityClient {
    pub fn new(base_url: &str) -> Result<Self, UrlParseError> {
        let base_url = Url::parse(base_url)?;
        let client = Client::new();
        Ok(Self { client, base_url })
    }

    pub async fn get_node_addresses(&self) -> Result<NodeAddresses, BeeDebugConnectivityError> {
        let url = self.base_url.join("addresses")?;
        let response = self.client.get(url).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }

    pub async fn get_peers(&self) -> Result<Vec<Peer>, BeeDebugConnectivityError> {
        let url = self.base_url.join("peers")?;
        let response = self.client.get(url).send().await?.error_for_status()?;
        let body: HashMap<String, Vec<Peer>> = response.json().await?;
        Ok(body.get("peers").cloned().unwrap_or_default())
    }

    pub async fn get_blocklist(&self) -> Result<Vec<Peer>, BeeDebugConnectivityError> {
        let url = self.base_url.join("blocklist")?;
        let response = self.client.get(url).send().await?.error_for_status()?;
        let body: HashMap<String, Vec<Peer>> = response.json().await?;
        Ok(body.get("peers").cloned().unwrap_or_default())
    }

    pub async fn remove_peer(&self, peer_address: &str) -> Result<RemovePeerResponse, BeeDebugConnectivityError> {
        let url = self.base_url.join(&format!("peers/{}", peer_address))?;
        let response = self.client.delete(url).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }

    pub async fn get_topology(&self) -> Result<Topology, BeeDebugConnectivityError> {
        let url = self.base_url.join("topology")?;
        let response = self.client.get(url).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }

    pub async fn ping_peer(&self, peer_address: &str) -> Result<PingResponse, BeeDebugConnectivityError> {
        let url = self.base_url.join(&format!("pingpong/{}", peer_address))?;
        let response = self.client.post(url).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }
}
