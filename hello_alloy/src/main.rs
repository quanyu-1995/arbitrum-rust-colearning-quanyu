use alloy::providers::{Provider, ProviderBuilder}; 
use std::error::Error;
 
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Set up the HTTP transport which is consumed by the RPC client.
    let rpc_url = "https://arbitrum-sepolia.drpc.org".parse()?;
 
    // Create a provider with the HTTP transport using the `reqwest` crate.
    let provider = ProviderBuilder::new().connect_http(rpc_url); 
 
    let last_block = provider.get_block_number().await?;

    println!("last block number: {last_block}");
    println!("Hello Web3");

    Ok(())
}