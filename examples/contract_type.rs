use std::str::FromStr;

use ethers::{
    providers::{Http, Middleware, Provider},
    types::Address,
};
use evm_hound::{contract_type_from_selectors, selectors_from_bytecode};
use eyre::Result;

// To Try:
// cargo run --example contract_type

#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::<Http>::try_from("https://rpc.flashbots.net/fast")?;
    let token_addr = Address::from_str("0xdac17f958d2ee523a2206206994597c13d831ec7")?;

    let code = provider.get_code(token_addr, None).await?;

    let selectors = selectors_from_bytecode(&code);
    let contract_type = contract_type_from_selectors(&selectors);

    println!("found {} selectors", selectors.len());
    println!("Contract type is {contract_type:?}");

    Ok(())
}
