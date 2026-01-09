use alloy::{
    primitives::{address, utils::{format_ether}},
    providers::{Provider, ProviderBuilder},
};
use std::error::Error;

 
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Set up the HTTP transport which is consumed by the RPC client.
    let rpc_url = "https://arbitrum-sepolia.drpc.org".parse()?;
 
    // Create a provider with the HTTP transport using the `reqwest` crate.
    let provider = ProviderBuilder::new().connect_http(rpc_url); 
    let address = address!("0x1c01ED1Df6c008B3c9f540f91694804d7a6b7e70");
    let balance = provider.get_balance(address).await?;
    let eth_balance = format_ether(balance);
    println!("last block number: {eth_balance}");

    Ok(())
}