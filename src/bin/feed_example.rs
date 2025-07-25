use bee_rs::api::feed::{create_feed_manifest, fetch_latest_feed_update, probe_feed, FeedUpdateOptions};
use bee_rs::api::bytes::UploadOptions;
use std::env;

const BEE_API_URL: &str = "http://localhost:1633";

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    let owner = "0x1234567890123456789012345678901234567890"; // Replace with a valid owner address
    let topic = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd"; // Replace with a valid topic
    let postage_batch_id = "36b7efd913ca4cf880b8eeac5093fa27b0825906c600685b6abdd6566e6cfe8f"; // Replace with a valid batch ID

    // Example: Create feed manifest
    let create_options = Some(UploadOptions {
        pin: Some(true),
        ..Default::default()
    });
    match create_feed_manifest(&client, BEE_API_URL, owner, topic, postage_batch_id, create_options).await {
        Ok(reference) => println!("Created Feed Manifest with Reference: {}", reference),
        Err(e) => eprintln!("Error creating feed manifest: {}", e),
    }

    // Example: Fetch latest feed update
    let fetch_options = Some(FeedUpdateOptions {
        has_timestamp: Some(true),
        ..Default::default()
    });
    match fetch_latest_feed_update(&client, BEE_API_URL, owner, topic, fetch_options).await {
        Ok(payload_result) => println!("Fetched Latest Feed Update: {:#?}", payload_result),
        Err(e) => eprintln!("Error fetching latest feed update: {}", e),
    }

    // Example: Probe feed
    match probe_feed(&client, BEE_API_URL, owner, topic).await {
        Ok(headers) => println!("Probed Feed Headers: {:#?}", headers),
        Err(e) => eprintln!("Error probing feed: {}", e),
    }
}
