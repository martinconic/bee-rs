use bee_rs::api::bzz::{download_file, upload_file, FileUploadOptions};
use bee_rs::api::bytes::{DownloadOptions, RedundantUploadOptions, UploadOptions};
use std::env;

const BEE_API_URL: &str = "http://localhost:1633";

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    // Example: Upload a file
    let file_data = b"Hello, Bee Bzz!".to_vec();
    let postage_batch_id = "36b7efd913ca4cf880b8eeac5093fa27b0825906c600685b6abdd6566e6cfe8f"; // Replace with a valid batch ID
    let file_name = Some("my_test_file.txt");
    let upload_options = Some(FileUploadOptions {
        redundant_upload_options: RedundantUploadOptions {
            upload_options: UploadOptions {
                pin: Some(true),
                ..Default::default()
            },
            ..Default::default()
        },
        content_type: Some("text/plain".to_string()),
        ..Default::default()
    });

    match upload_file(
        &client,
        BEE_API_URL,
        file_data.clone(),
        postage_batch_id,
        file_name,
        upload_options,
    )
    .await
    {
        Ok(result) => println!("File Upload Result: {:#?}", result),
        Err(e) => eprintln!("Error uploading file: {}", e),
    }

    // Example: Download a file
    let reference_to_download = "36b7efd913ca4cf880b8eeac5093fa27b0825906c600685b6abdd6566e6cfe8f"; // Replace with a valid reference
    let download_path = Some("my_test_file.txt"); // Optional path if it's a collection
    let download_options = Some(DownloadOptions {
        redundancy_strategy: Some(1),
        ..Default::default()
    });

    match download_file(
        &client,
        BEE_API_URL,
        reference_to_download,
        download_path,
        download_options,
    )
    .await
    {
        Ok(file_data) => {
            println!("Downloaded File Data: {:#?}", file_data);
            if let Ok(text) = String::from_utf8(file_data.data) {
                println!("Downloaded File Content: {}", text);
            }
        }
        Err(e) => eprintln!("Error downloading file: {}", e),
    }
}
