use bee_rs::api::debug::transactions::BeeDebugTransactionsClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bee_debug_transactions_client = BeeDebugTransactionsClient::new("http://localhost:1633")?;

    println!("Getting all transactions...");
    match bee_debug_transactions_client.get_all_transactions().await {
        Ok(transactions) => println!("All Transactions: {:#?}", transactions),
        Err(e) => println!("Failed to get all transactions: {}", e),
    }

    // To get a specific transaction or rebroadcast/cancel, you need a valid transaction hash.
    // You can get one from the `get_all_transactions()` call if there are pending transactions.
    let example_transaction_hash = "0x1234567890123456789012345678901234567890123456789012345678901234";

    println!("\nGetting transaction {}...", example_transaction_hash);
    match bee_debug_transactions_client.get_transaction(example_transaction_hash).await {
        Ok(transaction) => println!("Transaction: {:#?}", transaction),
        Err(e) => println!("Failed to get transaction: {}", e),
    }

    // Note: rebroadcast_transaction and cancel_transaction modify state
    // and are not included in this example for safety and simplicity.

    Ok(())
}
