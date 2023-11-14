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
    let token_addr = Address::from_str("0x7ae075546e8042dC263FA0eb6519ce0a04EABB93")?;

    let code = provider.get_code(token_addr, None).await?;

    println!("code len {}", code.len());

    let file_path = format!("testdata/{:?}", token_addr);
    let mut file = File::create(file_path).await?;

    file.write_all(&code).await?;

    Ok(())
}
