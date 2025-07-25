use bee_rs::api::envelope::post_envelope;
use std::env;

const BEE_API_URL: &str = "http://localhost:1633";

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    // Replace with a valid postage batch ID and reference
    let postage_batch_id = "36b7efd913ca4cf880b8eeac5093fa27b0825906c600685b6abdd6566e6cfe8f";
    let reference = "36b7efd913ca4cf880b8eeac5093fa27b0825906c600685b6abdd6566e6cfe8f";

    match post_envelope(&client, BEE_API_URL, postage_batch_id, reference).await {
        Ok(envelope) => println!("Envelope: {:#?}", envelope),
        Err(e) => eprintln!("Error posting envelope: {}", e),
    }
}
