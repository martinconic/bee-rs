use bee_rs::api::pinning::{get_all_pins, get_pin, pin, unpin};
use std::env;

const BEE_API_URL: &str = "http://localhost:1633";

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    let test_reference = "36b7efd913ca4cf880b8eeac5093fa27b0825906c600685b6abdd6566e6cfe8f"; // Replace with a valid reference

    // Example: Pin a reference
    match pin(&client, BEE_API_URL, test_reference).await {
        Ok(_) => println!("Successfully pinned: {}", test_reference),
        Err(e) => eprintln!("Error pinning {}: {}", test_reference, e),
    }

    // Example: Get pin status
    match get_pin(&client, BEE_API_URL, test_reference).await {
        Ok(pin_info) => println!("Pin Info for {}: {:#?}", test_reference, pin_info),
        Err(e) => eprintln!("Error getting pin info for {}: {}", test_reference, e),
    }

    // Example: Get all pins
    match get_all_pins(&client, BEE_API_URL).await {
        Ok(all_pins) => println!("All Pinned References: {:#?}", all_pins),
        Err(e) => eprintln!("Error getting all pins: {}", e),
    }

    // Example: Unpin a reference
    match unpin(&client, BEE_API_URL, test_reference).await {
        Ok(_) => println!("Successfully unpinned: {}", test_reference),
        Err(e) => eprintln!("Error unpinning {}: {}", test_reference, e),
    }
}
