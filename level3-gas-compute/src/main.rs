use alloy::{
    primitives::{utils::{format_ether}},
    providers::{Provider, ProviderBuilder},
};
use std::error::Error;

 
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rpc_url = "https://arbitrum-sepolia.drpc.org".parse()?;
    let provider = ProviderBuilder::new().connect_http(rpc_url); 
    // 获取当前gas价格
    let gas_price = provider.get_gas_price().await?;
    // 基础转账 Gas 限额取 21000
    let base_transfer_gas_limit: u128 = 21000;
    // gas费(wei) = 当前gas价格 * 基础转账 Gas 限额
    let gas_fee_wei = gas_price * base_transfer_gas_limit;
    // gas费（ETH）
    let gas_fee_eth = format_ether(gas_fee_wei);
    println!("gas_fee_wei: {}; gas_fee_eth: {}", gas_fee_wei, gas_fee_eth);

    Ok(())
}