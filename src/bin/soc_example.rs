use bee_rs::api::soc::upload;
use bee_rs::api::bytes::UploadOptions;
use std::env;

const BEE_API_URL: &str = "http://localhost:1633";

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    // Example: Upload a SOC
    let owner = "0x1234567890123456789012345678901234567890"; // Replace with a valid owner address
    let identifier = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd"; // Replace with a valid identifier
    let signature = "0x112233445566778899aabbccddeeff00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff00"; // Replace with a valid signature
    let data = vec![0; 4096]; // Example SOC data (4096 bytes)
    let postage_batch_id = "36b7efd913ca4cf880b8eeac5093fa27b0825906c600685b6abdd6566e6cfe8f"; // Replace with a valid batch ID
    let options = Some(UploadOptions {
        pin: Some(true),
        ..Default::default()
    });

    match upload(
        &client,
        BEE_API_URL,
        owner,
        identifier,
        signature,
        data,
        postage_batch_id,
        options,
    )
    .await
    {
        Ok(result) => println!("SOC Upload Result: {:#?}", result),
        Err(e) => eprintln!("Error uploading SOC: {}", e),
    }
}
