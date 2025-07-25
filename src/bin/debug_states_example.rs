use bee_rs::api::debug::states::{BeeDebugStatesClient, ChainState, ReserveState, WalletBalance};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bee_debug_states_client = BeeDebugStatesClient::new("http://localhost:1633")?;

    println!("Getting reserve state...");
    match bee_debug_states_client.get_reserve_state().await {
        Ok(reserve_state) => println!("Reserve State: {:#?}", reserve_state),
        Err(e) => println!("Failed to get reserve state: {}", e),
    }

    println!("\nGetting chain state...");
    match bee_debug_states_client.get_chain_state().await {
        Ok(chain_state) => println!("Chain State: {:#?}", chain_state),
        Err(e) => println!("Failed to get chain state: {}", e),
    }

    println!("\nGetting wallet balance...");
    match bee_debug_states_client.get_wallet_balance().await {
        Ok(wallet_balance) => println!("Wallet Balance: {:#?}", wallet_balance),
        Err(e) => println!("Failed to get wallet balance: {}", e),
    }

    Ok(())
}