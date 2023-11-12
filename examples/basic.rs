use std::str::FromStr;

use ethers::{
    providers::{Http, Middleware, Provider},
    types::Address,
};
use eyre::Result;

use evm_hound::selectors_from_bytecode;

#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::<Http>::try_from("https://eth.meowrpc.com")?;
    let token_addr = Address::from_str("0x7ae075546e8042dC263FA0eb6519ce0a04EABB93")?;

    let code = provider.get_code(token_addr, None).await?;

    let func_selectors = selectors_from_bytecode(code.as_ref());

    println!("{func_selectors:?}");

    Ok(())
}
