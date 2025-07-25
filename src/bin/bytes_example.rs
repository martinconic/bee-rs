use bee_rs::api::bytes::{download, head, upload, DownloadOptions, RedundantUploadOptions, UploadOptions};
use std::env;

const BEE_API_URL: &str = "http://localhost:1633";

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    // Example: Upload data
    let data_to_upload = vec![1, 2, 3, 4, 5];
    let postage_batch_id = "36b7efd913ca4cf880b8eeac5093fa27b0825906c600685b6abdd6566e6cfe8f"; // Replace with a valid batch ID
    let upload_options = Some(RedundantUploadOptions {
        upload_options: UploadOptions {
            act: Some(true),
            pin: Some(true),
            ..Default::default()
        },
        redundancy_level: Some(1),
    });

    match upload(
        &client,
        BEE_API_URL,
        data_to_upload.clone(),
        postage_batch_id,
        upload_options,
    )
    .await
    {
        Ok(result) => println!("Upload Result: {:#?}", result),
        Err(e) => eprintln!("Error uploading data: {}", e),
    }

    // Example: Head request
    let reference_to_head = "36b7efd913ca4cf880b8eeac5093fa27b0825906c600685b6abdd6566e6cfe8f"; // Replace with a valid reference
    match head(&client, BEE_API_URL, reference_to_head).await {
        Ok(info) => println!("Reference Information: {:#?}", info),
        Err(e) => eprintln!("Error getting reference information: {}", e),
    }

    // Example: Download data
    let resource_to_download = "36b7efd913ca4cf880b8eeac5093fa27b0825906c600685b6abdd6566e6cfe8f"; // Replace with a valid resource
    let download_options = Some(DownloadOptions {
        redundancy_strategy: Some(1),
        fallback: Some(true),
        ..Default::default()
    });

    match download(
        &client,
        BEE_API_URL,
        resource_to_download,
        download_options,
    )
    .await
    {
        Ok(data) => println!("Downloaded Data: {:#?}", data),
        Err(e) => eprintln!("Error downloading data: {}", e),
    }
}
