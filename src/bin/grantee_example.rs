use bee_rs::api::grantee::{create_grantees, get_grantees, patch_grantees};
use std::env;

const BEE_API_URL: &str = "http://localhost:1633";

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    // Example: Get grantees
    let reference_to_get = "36b7efd913ca4cf880b8eeac5093fa27b0825906c600685b6abdd6566e6cfe8f"; // Replace with a valid reference
    match get_grantees(&client, BEE_API_URL, reference_to_get).await {
        Ok(grantees_result) => println!("Grantees for {}: {:#?}", reference_to_get, grantees_result),
        Err(e) => eprintln!("Error getting grantees: {}", e),
    }

    // Example: Create grantees
    let postage_batch_id_create = "36b7efd913ca4cf880b8eeac5093fa27b0825906c600685b6abdd6566e6cfe8f"; // Replace with a valid batch ID
    let grantees_to_create = vec![
        "0x1111111111111111111111111111111111111111".to_string(),
        "0x2222222222222222222222222222222222222222".to_string(),
    ];
    match create_grantees(&client, BEE_API_URL, postage_batch_id_create, grantees_to_create).await {
        Ok(result) => println!("Create Grantees Result: {:#?}", result),
        Err(e) => eprintln!("Error creating grantees: {}", e),
    }

    // Example: Patch grantees
    let postage_batch_id_patch = "36b7efd913ca4cf880b8eeac5093fa27b0825906c600685b6abdd6566e6cfe8f"; // Replace with a valid batch ID
    let reference_to_patch = "36b7efd913ca4cf880b8eeac5093fa27b0825906c600685b6abdd6566e6cfe8f"; // Replace with a valid reference
    let history_reference_patch = "36b7efd913ca4cf880b8eeac5093fa27b0825906c600685b6abdd6566e6cfe8f"; // Replace with a valid history reference
    let add_grantees = Some(vec!["0x3333333333333333333333333333333333333333".to_string()]);
    let revoke_grantees = None;
    match patch_grantees(
        &client,
        BEE_API_URL,
        postage_batch_id_patch,
        reference_to_patch,
        history_reference_patch,
        add_grantees,
        revoke_grantees,
    )
    .await
    {
        Ok(result) => println!("Patch Grantees Result: {:#?}", result),
        Err(e) => eprintln!("Error patching grantees: {}", e),
    }
}
