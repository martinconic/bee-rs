use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

use crate::api::bytes::UploadOptions;
use crate::api::chunk::upload as upload_chunk;

// Placeholder for SingleOwnerChunk. In a real scenario, this would be a proper struct.
// For now, we'll treat it as raw bytes for the payload.
pub type SingleOwnerChunk = Vec<u8>;

const ENDPOINT: &str = "gsoc";

pub async fn send(
    client: &Client,
    base_url: &str,
    soc_data: SingleOwnerChunk,
    postage_batch_id: &str,
    options: Option<UploadOptions>,
) -> Result<String, Error> {
    // The bee-js `send` function directly calls `uploadSingleOwnerChunk`.
    // We'll map this to our `chunk::upload` function.
    let upload_result = upload_chunk(client, base_url, soc_data, postage_batch_id, options).await?;
    Ok(upload_result.reference)
}

// WebSocket subscription is not directly supported in this API module.
// This function serves as a placeholder to indicate that.
pub fn subscribe(
    _url: &str,
    _reference: &str,
    _headers: Option<std::collections::HashMap<String, String>>,
) -> Result<(), String> {
    Err("WebSocket subscriptions are not directly supported in this API module.".to_string())
}
