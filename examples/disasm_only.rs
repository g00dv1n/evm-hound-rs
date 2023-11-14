use std::str::FromStr;

use ethers::{
    providers::{Http, Middleware, Provider},
    types::Address,
};
use eyre::Result;

use evm_hound::disasm;

// To Try:
// cargo run --example disasm_only

#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::<Http>::try_from("https://eth.llamarpc.com")?;
    let token_addr = Address::from_str("0xdac17f958d2ee523a2206206994597c13d831ec7")?;

    let code = provider.get_code(token_addr, None).await?;

    let parsed_bytecode = disasm(&code);

    // Print first 50 instructions
    for inst in parsed_bytecode.iter().take(50) {
        println!("{inst}")
    }

    Ok(())
}
