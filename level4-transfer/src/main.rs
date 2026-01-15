use alloy::{
    network::TransactionBuilder,
    primitives::{
        address,
        utils::{format_ether, Unit},
        U256,
    },
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use dotenv::dotenv;
use std::{env, error::Error};
 
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 加载 .env 文件
    dotenv()?;
    // Initialize a signer with a private key
    let sender_private_key: String = env::var("SENDER_PRIVATE_KEY")?;
    let signer: PrivateKeySigner = sender_private_key.parse()?;
 
    // Instantiate a provider with the signer and a local anvil node
    let provider = ProviderBuilder::new() 
        .wallet(signer) 
        .connect("https://arbitrum-sepolia.drpc.org") 
        .await?;
 
    // Prepare a transaction request to send 0.001 ETH to Alice
    let alice = address!("0x51D95Ce62bBa961e5773704fDBA67616F4F678e7"); 
    let value = Unit::ETHER.wei() / U256::from(1000);
    println!("value={}",value);
    let tx = TransactionRequest::default() 
        .with_to(alice) 
        .with_value(value); 
 
    // Send the transaction and wait for the broadcast
    let pending_tx = provider.send_transaction(tx).await?; 
    println!("Pending transaction... {}", pending_tx.tx_hash());
 
    // Wait for the transaction to be included and get the receipt
    let receipt = pending_tx.get_receipt().await?; 
    println!(
        "Transaction included in block {}",
        receipt.block_number.expect("Failed to get block number")
    );
 
    println!("Transferred {:.5} ETH to {alice}", format_ether(value));
 
    Ok(())
}