use bee_rs::api::chunk::{download, upload};
use bee_rs::api::bytes::{DownloadOptions, UploadOptions};
use std::env;

const BEE_API_URL: &str = "http://localhost:1633";

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    // Example: Upload a chunk
    let chunk_data = vec![0; 4096]; // Example chunk data (4096 bytes)
    let postage_batch_id = "36b7efd913ca44f880b8eeac5093fa27b0825906c600685b6abdd6566e6cfe8f"; // Replace with a valid batch ID
    let upload_options = Some(UploadOptions {
        pin: Some(true),
        ..Default::default()
    });

    match upload(
        &client,
        BEE_API_URL,
        chunk_data.clone(),
        postage_batch_id,
        upload_options,
    )
    .await
    {
        Ok(result) => println!("Chunk Upload Result: {:#?}", result),
        Err(e) => eprintln!("Error uploading chunk: {}", e),
    }

    // Example: Download a chunk
    let reference_to_download = "36b7efd913ca44f880b8eeac5093fa27b0825906c600685b6abdd6566e6cfe8f"; // Replace with a valid chunk reference
    let download_options = Some(DownloadOptions {
        redundancy_strategy: Some(1),
        ..Default::default()
    });

    match download(
        &client,
        BEE_API_URL,
        reference_to_download,
        download_options,
    )
    .await
    {
        Ok(data) => println!("Downloaded Chunk Data (first 10 bytes): {:#?}", &data[..std::cmp::min(data.len(), 10)]),
        Err(e) => eprintln!("Error downloading chunk: {}", e),
    }
}
