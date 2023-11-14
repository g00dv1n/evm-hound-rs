# EVM Hound

A Minimalistic Rust library to extract all potential function selectors from EVM bytecode without source code.

## Installation

`$ cargo add evm_hound`

## Usage

Run example:

`$ cargo run --example basic`

```rust
// examples/basic.rs
use std::str::FromStr;

use ethers::{
    providers::{Http, Middleware, Provider},
    types::Address,
};
use eyre::Result;

use evm_hound::{selectors_from_bytecode, string_selectors_from_bytecode};

#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::<Http>::try_from("https://eth.llamarpc.com")?;
    let token_addr = Address::from_str("0x7ae075546e8042dC263FA0eb6519ce0a04EABB93")?;

    let code = provider.get_code(token_addr, None).await?;

    let raw_selectors = selectors_from_bytecode(&code);
    let string_selectors = string_selectors_from_bytecode(&code);

    println!("{raw_selectors:?}");
    println!("{string_selectors:?}");

    Ok(())
}
```

### Credits

Made for [Hackers.tools Trading Simulator](https://hackers.tools/) to search/bruteforce for potential methods that start trading.
