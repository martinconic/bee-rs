
// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will re-upload a chunk to the network.
//!
//! `cargo run --example stewardship_example --release`

use bee_rs::bee::Bee;
use std::env;

const UNKNOWN_REFERENCE: &str = "1000000000000000000000000000000000000000000000000000000000000000";

#[tokio::main]
async fn main() {
    // The node API endpoint
    let bee_url = env::var("BEE_URL").unwrap_or_else(|_| "http://localhost:1633".to_string());
    let bee = Bee::new(&bee_url);

    // Re-upload the chunk
    if let Err(error) = bee.stewardship().reupload(UNKNOWN_REFERENCE).await {
        println!("Error: {}", error);
    }

    // Check if the chunk is retrievable
    match bee.stewardship().is_retrievable(UNKNOWN_REFERENCE).await {
        Ok(is_retrievable) => {
            if is_retrievable {
                println!("Chunk is retrievable");
            } else {
                println!("Chunk is not retrievable");
            }
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}
