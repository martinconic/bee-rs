use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

const ENVELOPE_ENDPOINT: &str = "envelope";

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvelopeWithBatchId {
    pub issuer: Vec<u8>,
    pub index: Vec<u8>,
    pub timestamp: Vec<u8>,
    pub signature: Vec<u8>,
    #[serde(rename = "batchId")]
    pub batch_id: String,
}

pub async fn post_envelope(
    client: &Client,
    base_url: &str,
    postage_batch_id: &str,
    reference: &str,
) -> Result<EnvelopeWithBatchId, Error> {
    let url = format!("{}/{}/{}", base_url, ENVELOPE_ENDPOINT, reference);
    let mut request_builder = client.post(&url);

    request_builder = request_builder.header("swarm-postage-batch-id", postage_batch_id);

    let response = request_builder.send().await?;
    let mut envelope: EnvelopeWithBatchId = response.json().await?;

    // The batchId is passed as a parameter, so we set it directly
    envelope.batch_id = postage_batch_id.to_string();

    Ok(envelope)
}
