use bee_rs::api::debug::balance::BeeDebugBalanceClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bee_debug_balance_client = BeeDebugBalanceClient::new("http://localhost:1633")?;

    println!("Getting all balances...");
    let all_balances_result = bee_debug_balance_client.get_all_balances().await;
    let mut example_peer_address = "0x1234567890123456789012345678901234567890123456789012345678901234".to_string();

    match all_balances_result {
        Ok(balances) => {
            println!("All Balances: {:#?}", balances);
            if let Some(peer_balance) = balances.balances.first() {
                example_peer_address = peer_balance.peer.clone();
            }
        }
        Err(e) => println!("Failed to get all balances: {}", e),
    }

    println!("\nGetting peer balance (example peer: {})...", example_peer_address);
    match bee_debug_balance_client.get_peer_balance(&example_peer_address).await {
        Ok(peer_balance) => println!("Peer Balance: {:#?}", peer_balance),
        Err(e) => println!("Failed to get peer balance: {}", e),
    }

    println!("\nGetting past due consumption balances...");
    match bee_debug_balance_client.get_past_due_consumption_balances().await {
        Ok(balances) => println!("Past Due Consumption Balances: {:#?}", balances),
        Err(e) => println!("Failed to get past due consumption balances: {}", e),
    }

    println!("\nGetting past due consumption peer balance (example peer: {})...", example_peer_address);
    match bee_debug_balance_client.get_past_due_consumption_peer_balance(&example_peer_address).await {
        Ok(peer_balance) => println!("Past Due Consumption Peer Balance: {:#?}", peer_balance),
        Err(e) => println!("Failed to get past due consumption peer balance: {}", e),
    }

    Ok(())
}