use reqwest::{Client, Error as ReqwestError};
use serde::{Deserialize, Serialize};
use url::{Url, ParseError as UrlParseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BeeDebugClientError {
    #[error("Request error: {0}")]
    Reqwest(#[from] ReqwestError),
    #[error("URL parse error: {0}")]
    UrlParse(#[from] UrlParseError),
    #[error("Invalid Bee mode: {0}")]
    InvalidBeeMode(String),
    #[error("Semver parse error: {0}")]
    Semver(#[from] semver::Error),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum BeeMode {
    Full,
    Light,
    UltraLight,
    Dev,
}

impl std::str::FromStr for BeeMode {
    type Err = BeeDebugClientError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "full" => Ok(BeeMode::Full),
            "light" => Ok(BeeMode::Light),
            "ultra-light" => Ok(BeeMode::UltraLight),
            "dev" => Ok(BeeMode::Dev),
            _ => Err(BeeDebugClientError::InvalidBeeMode(s.to_string())),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugStatus {
    pub overlay: String,
    pub proximity: u32,
    pub bee_mode: BeeMode,
    pub reserve_size: u64,
    pub reserve_size_within_radius: u64,
    pub pullsync_rate: u32,
    pub storage_radius: u32,
    pub connected_peers: u32,
    pub neighborhood_size: u32,
    pub batch_commitment: u64,
    pub is_reachable: bool,
    pub last_synced_block: u32,
    pub committed_depth: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Health {
    pub status: String,
    pub version: String,
    #[serde(rename = "apiVersion")]
    pub api_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Readiness {
    pub status: String,
    pub version: String,
    #[serde(rename = "apiVersion")]
    pub api_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeInfo {
    pub bee_mode: BeeMode,
    pub chequebook_enabled: bool,
    pub swap_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BeeVersions {
    pub supported_bee_version: String,
    pub supported_bee_api_version: String,
    pub bee_version: String,
    pub bee_api_version: String,
}

pub struct BeeDebugClient {
    client: Client,
    base_url: Url,
}

impl BeeDebugClient {
    pub fn new(base_url: &str) -> Result<Self, UrlParseError> {
        let base_url = Url::parse(base_url)?;
        let client = Client::new();
        Ok(Self { client, base_url })
    }

    pub async fn get_debug_status(&self) -> Result<DebugStatus, BeeDebugClientError> {
        let url = self.base_url.join("status")?;
        let response = self.client.get(url).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }

    pub async fn get_health(&self) -> Result<Health, BeeDebugClientError> {
        let url = self.base_url.join("health")?;
        let response = self.client.get(url).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }

    pub async fn get_readiness(&self) -> Result<Readiness, BeeDebugClientError> {
        let url = self.base_url.join("readiness")?;
        let response = self.client.get(url).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }

    pub async fn get_node_info(&self) -> Result<NodeInfo, BeeDebugClientError> {
        let url = self.base_url.join("node")?;
        let response = self.client.get(url).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }

    pub async fn is_supported_exact_version(&self) -> Result<bool, BeeDebugClientError> {
        const SUPPORTED_BEE_VERSION_EXACT: &str = "2.4.0-390a402e";
        let health = self.get_health().await?;
        Ok(health.version == SUPPORTED_BEE_VERSION_EXACT)
    }

    pub async fn is_supported_api_version(&self) -> Result<bool, BeeDebugClientError> {
        const SUPPORTED_API_VERSION: &str = "7.2.0";
        let health = self.get_health().await?;
        let api_version_major = semver::Version::parse(&health.api_version)?.major;
        let supported_api_version_major = semver::Version::parse(SUPPORTED_API_VERSION)?.major;
        Ok(api_version_major == supported_api_version_major)
    }

    pub async fn get_versions(&self) -> Result<BeeVersions, BeeDebugClientError> {
        const SUPPORTED_BEE_VERSION_EXACT: &str = "2.4.0-390a402e";
        const SUPPORTED_API_VERSION: &str = "7.2.0";
        let health = self.get_health().await?;

        Ok(BeeVersions {
            supported_bee_version: SUPPORTED_BEE_VERSION_EXACT.to_string(),
            supported_bee_api_version: SUPPORTED_API_VERSION.to_string(),
            bee_version: health.version,
            bee_api_version: health.api_version,
        })
    }
}