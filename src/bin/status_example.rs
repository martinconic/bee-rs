use bee_rs::api::status::{check_connection, is_gateway};
use std::env;

const BEE_API_URL: &str = "http://localhost:1633";

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    // Example: Check connection
    match check_connection(&client, BEE_API_URL).await {
        Ok(_) => println!("Successfully connected to Bee node."),
        Err(e) => eprintln!("Error checking connection: {}", e),
    }

    // Example: Check if it's a gateway
    match is_gateway(&client, BEE_API_URL).await {
        Ok(is_gw) => println!("Is gateway: {}", is_gw),
        Err(e) => eprintln!("Error checking if it's a gateway: {}", e),
    }
}