use bee_rs::api::gsoc::send;
use bee_rs::api::bytes::UploadOptions;
use std::env;

const BEE_API_URL: &str = "http://localhost:1633";

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    // Example: Send a GSOC
    let soc_data = vec![0; 4096]; // Example GSOC data (4096 bytes)
    let postage_batch_id = "36b7efd913ca4cf880b8eeac5093fa27b0825906c600685b6abdd6566e6cfe8f"; // Replace with a valid batch ID
    let options = Some(UploadOptions {
        pin: Some(true),
        ..Default::default()
    });

    match send(
        &client,
        BEE_API_URL,
        soc_data.clone(),
        postage_batch_id,
        options,
    )
    .await
    {
        Ok(reference) => println!("GSOC Sent, Reference: {}", reference),
        Err(e) => eprintln!("Error sending GSOC: {}", e),
    }
}
