use bee_rs::api::debug::status::BeeDebugClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bee_debug_client = BeeDebugClient::new("http://localhost:1633")?;

    println!("Getting debug status...");
    match bee_debug_client.get_debug_status().await {
        Ok(status) => println!("Debug Status: {:#?}", status),
        Err(e) => println!("Failed to get debug status: {}", e),
    }

    println!("\nGetting health...");
    match bee_debug_client.get_health().await {
        Ok(health) => println!("Health: {:#?}", health),
        Err(e) => println!("Failed to get health: {}", e),
    }

    println!("\nGetting readiness...");
    match bee_debug_client.get_readiness().await {
        Ok(readiness) => println!("Readiness: {:#?}", readiness),
        Err(e) => println!("Failed to get readiness: {}", e),
    }

    println!("\nGetting node info...");
    match bee_debug_client.get_node_info().await {
        Ok(node_info) => println!("Node Info: {:#?}", node_info),
        Err(e) => println!("Failed to get node info: {}", e),
    }

    println!("\nChecking supported exact version...");
    match bee_debug_client.is_supported_exact_version().await {
        Ok(is_supported) => println!("Is supported exact version: {}", is_supported),
        Err(e) => println!("Failed to check supported exact version: {}", e),
    }

    println!("\nChecking supported API version...");
    match bee_debug_client.is_supported_api_version().await {
        Ok(is_supported) => println!("Is supported API version: {}", is_supported),
        Err(e) => println!("Failed to check supported API version: {}", e),
    }

    println!("\nGetting versions...");
    match bee_debug_client.get_versions().await {
        Ok(versions) => println!("Versions: {:#?}", versions),
        Err(e) => println!("Failed to get versions: {}", e),
    }

    Ok(())
}
