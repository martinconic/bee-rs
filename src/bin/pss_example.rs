use bee_rs::api::pss::send;
use std::env;

const BEE_API_URL: &str = "http://localhost:1633";

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    // Example: Send a PSS message
    let topic = "test-topic";
    let target = "0000000000000000"; // 8-byte target
    let message = b"Hello from PSS!".to_vec();
    let postage_batch_id = "36b7efd913ca4cf880b8eeac5093fa27b0825906c600685b6abdd6566e6cfe8f"; // Replace with a valid batch ID
    let recipient = None; // Some("0x1234567890123456789012345678901234567890"); // Optional recipient public key

    match send(
        &client,
        BEE_API_URL,
        topic,
        target,
        message,
        postage_batch_id,
        recipient,
    )
    .await
    {
        Ok(_) => println!("PSS message sent successfully!"),
        Err(e) => eprintln!("Error sending PSS message: {}", e),
    }
}
