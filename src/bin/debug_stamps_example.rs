
// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will create a new postage batch, top it up, and dilute it.
//!
//! `cargo run --example debug_stamps_example --release`

use bee_rs::api::debug::stamps::BeeDebugStampsClient;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    // The node API endpoint
    let bee_debug_url = env::var("BEE_DEBUG_URL").unwrap_or_else(|_| "http://localhost:1635".to_string());
    let client = BeeDebugStampsClient::new(&bee_debug_url).unwrap();

    // Create a new postage batch
    let amount = "10000";
    let depth = 20;
    let gas_price = None;
    let immutable_flag = None;
    let label = Some("my-batch");

    match client.create_postage_batch(amount, depth, gas_price, immutable_flag, label).await {
        Ok(batch_id) => {
            println!("Created postage batch with ID: {}", batch_id);

            // Top up the batch
            let top_up_amount = "5000";
            match client.top_up_batch(&batch_id, top_up_amount).await {
                Ok(new_batch_id) => println!("Topped up batch, new ID: {}", new_batch_id),
                Err(e) => eprintln!("Error topping up batch: {}", e),
            }

            // Dilute the batch
            let new_depth = 21;
            match client.dilute_batch(&batch_id, new_depth).await {
                Ok(new_batch_id) => println!("Diluted batch, new ID: {}", new_batch_id),
                Err(e) => eprintln!("Error diluting batch: {}", e),
            }
        }
        Err(e) => eprintln!("Error creating postage batch: {}", e),
    }
}
