use bee_rs::api::debug::connectivity::BeeDebugConnectivityClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bee_debug_connectivity_client = BeeDebugConnectivityClient::new("http://localhost:1633")?;

    println!("Getting node addresses...");
    match bee_debug_connectivity_client.get_node_addresses().await {
        Ok(response) => println!("Node Addresses: {:#?}", response),
        Err(e) => println!("Failed to get node addresses: {}", e),
    }

    println!("\nGetting peers...");
    match bee_debug_connectivity_client.get_peers().await {
        Ok(response) => println!("Peers: {:#?}", response),
        Err(e) => println!("Failed to get peers: {}", e),
    }

    println!("\nGetting blocklist...");
    match bee_debug_connectivity_client.get_blocklist().await {
        Ok(response) => println!("Blocklist: {:#?}", response),
        Err(e) => println!("Failed to get blocklist: {}", e),
    }

    println!("\nGetting topology...");
    match bee_debug_connectivity_client.get_topology().await {
        Ok(response) => println!("Topology: {:#?}", response),
        Err(e) => println!("Failed to get topology: {}", e),
    }

    // To ping a peer, you need a valid peer address from your Bee node.
    // You can get one from the `get_peers()` call.
    let example_peer_address = "0x1234567890123456789012345678901234567890123456789012345678901234";
    println!("\nAttempting to ping peer {}...", example_peer_address);
    match bee_debug_connectivity_client.ping_peer(example_peer_address).await {
        Ok(response) => println!("Ping Response: {:#?}", response),
        Err(e) => println!("Failed to ping peer: {}", e),
    }

    // To remove a peer, uncomment the following lines and replace with a valid peer address.
    // println!("\nAttempting to remove peer {}...", example_peer_address);
    // match bee_debug_connectivity_client.remove_peer(example_peer_address).await {
    //     Ok(response) => println!("Remove Peer Response: {:#?}", response),
    //     Err(e) => println!("Failed to remove peer: {}", e),
    // }

    Ok(())
}