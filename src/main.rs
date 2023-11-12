use std::str::FromStr;

use ethers::{
    providers::{Http, Middleware, Provider},
    types::Address,
};
use eyre::Result;
use selectors::selectors_from_bytecode;

mod disasm;
mod opcodes;
mod selectors;

#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::<Http>::try_from("https://eth.meowrpc.com")?;
    let token_addr = Address::from_str("0x7ae075546e8042dC263FA0eb6519ce0a04EABB93")?;

    let code = provider.get_code(token_addr, None).await?;
    let code_string = code.to_string();

    let func_selectors = selectors_from_bytecode(&code_string);

    println!("{func_selectors:?}");

    Ok(())
}
