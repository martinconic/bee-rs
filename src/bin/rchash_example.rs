use bee_rs::api::rchash::rchash;
use std::env;

const BEE_API_URL: &str = "http://localhost:1633";

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    // Example: Call rchash
    let depth = 10;
    let anchor1 = "0x1234567890123456789012345678901234567890"; // Replace with valid anchor
    let anchor2 = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd"; // Replace with valid anchor

    match rchash(&client, BEE_API_URL, depth, anchor1, anchor2).await {
        Ok(duration) => println!("Rchash duration: {} seconds", duration),
        Err(e) => eprintln!("Error calling rchash: {}", e),
    }
}
