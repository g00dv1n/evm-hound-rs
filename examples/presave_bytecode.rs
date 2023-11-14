use std::str::FromStr;

use ethers::{
    providers::{Http, Middleware, Provider},
    types::Address,
};
use eyre::Result;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

// To Try:
// cargo run --example presave_bytecode

#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::<Http>::try_from("https://eth.llamarpc.com")?;
    let token_addr = Address::from_str("0xdac17f958d2ee523a2206206994597c13d831ec7")?;

    let code = provider.get_code(token_addr, None).await?;

    println!("code len {}", code.len());

    let mut file = File::create("testdata/bytecode").await?;

    file.write_all(&code).await?;

    Ok(())
}
