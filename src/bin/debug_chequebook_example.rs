use bee_rs::api::debug::chequebook::BeeDebugChequebookClient;
use bee_rs::api::debug::balance::BeeDebugBalanceClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bee_debug_chequebook_client = BeeDebugChequebookClient::new("http://localhost:1633")?;
    let bee_debug_balance_client = BeeDebugBalanceClient::new("http://localhost:1633")?;

    println!("Getting chequebook address...");
    match bee_debug_chequebook_client.get_chequebook_address().await {
        Ok(response) => println!("Chequebook Address: {:#?}", response),
        Err(e) => println!("Failed to get chequebook address: {}", e),
    }

    println!("\nGetting chequebook balance...");
    match bee_debug_chequebook_client.get_chequebook_balance().await {
        Ok(response) => println!("Chequebook Balance: {:#?}", response),
        Err(e) => println!("Failed to get chequebook balance: {}", e),
    }

    let mut example_peer_address = "0x1234567890123456789012345678901234567890123456789012345678901234".to_string();

    match bee_debug_balance_client.get_all_balances().await {
        Ok(balances) => {
            if let Some(peer_balance) = balances.balances.first() {
                example_peer_address = peer_balance.peer.clone();
            }
        }
        Err(e) => println!("Failed to get all balances for peer address selection: {}", e),
    }

    println!("\nGetting last cashout action for peer {}...", example_peer_address);
    match bee_debug_chequebook_client.get_last_cashout_action(&example_peer_address).await {
        Ok(response) => println!("Last Cashout Action: {:#?}", response),
        Err(e) => println!("Failed to get last cashout action: {}", e),
    }

    println!("\nGetting last cheques for peer {}...", example_peer_address);
    match bee_debug_chequebook_client.get_last_cheques_for_peer(&example_peer_address).await {
        Ok(response) => println!("Last Cheques for Peer: {:#?}", response),
        Err(e) => println!("Failed to get last cheques for peer: {}", e),
    }

    println!("\nGetting all last cheques...");
    match bee_debug_chequebook_client.get_last_cheques().await {
        Ok(response) => println!("All Last Cheques: {:#?}", response),
        Err(e) => println!("Failed to get all last cheques: {}", e),
    }

    // Note: cashout_last_cheque, deposit_tokens, and withdraw_tokens modify state
    // and are not included in this example for safety and simplicity.

    Ok(())
}