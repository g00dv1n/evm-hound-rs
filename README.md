# EVM Hound

A Minimalistic Rust library to extract all potential function selectors from EVM bytecode without source code.

## Installation

`$ cargo add evm_hound`

## Usage

```rust
// examples/basic.rs
use ethers::{
    providers::{Http, Middleware, Provider},
    types::Address,
};
use eyre::Result;

use evm_hound::{selectors_from_bytecode, string_selectors_from_bytecode};

// To Try:
// cargo run --example basic

#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::<Http>::try_from("https://rpc.flashbots.net/fast")?;
    let token_addr: Address = "0xdac17f958d2ee523a2206206994597c13d831ec7".parse()?;

    let code = provider.get_code(token_addr, None).await?;

    let raw_selectors = selectors_from_bytecode(&code);
    let string_selectors = string_selectors_from_bytecode(&code);

    println!("found {} selectors", raw_selectors.len());
    println!("{raw_selectors:?}");
    println!("{string_selectors:?}");

    Ok(())
}
```

Also, EVM Hound can detect basic contract types using extracted selectors:

```rust
pub enum ContractType {
    /// ERC20 - Typical token contract
    ERC20,
    /// ERC721 - Typical NFT contract
    ERC721,
    /// Any other contract
    ANY,
}
```

Check out example at [examples/contract_type.rs](examples/contract_type.rs).

### Credits

Made for [Hackers.tools Trading Simulator](https://hackers.tools/) to search/bruteforce for potential methods that start trading.
