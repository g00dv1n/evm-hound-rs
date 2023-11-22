use crate::{disasm::disasm, opcodes::Opcode, utils};

/// Selector alias as 4 raw bytes (bytes4 in Solidity)
pub type Selector = [u8; 4];

/// Returns all potential function selectors from the contract deployed bytecode as `[u8; 4]`;
/// # Arguments
/// * `code` - A slice of bytes which represents contract bytecode
///
/// # Examples
/// ```rust
/// use ethers::{
///     providers::{Http, Middleware, Provider},
///     types::Address,
/// };
/// use eyre::Result;

/// use evm_hound::selectors_from_bytecode;

/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let provider = Provider::<Http>::try_from("https://rpc.flashbots.net/fast")?;
///     let token_addr: Address = "0xdac17f958d2ee523a2206206994597c13d831ec7".parse()?;
///
///     let code = provider.get_code(token_addr, None).await?;
///     let selectors = selectors_from_bytecode(&code);
///
///     println!("found {} selectors", selectors.len());
///     println!("{selectors:?}");
///
///     Ok(())
/// }
/// ```
pub fn selectors_from_bytecode(code: &[u8]) -> Vec<Selector> {
    let bytecode = disasm(code);

    let mut selectors: Vec<Selector> = Vec::new();
    let mut save_selector = |selector: Selector| {
        if !selectors.contains(&selector) {
            selectors.push(selector);
        }
    };

    let mut i = 4usize;

    while i < bytecode.len() {
        let five_seq = &bytecode[i - 4..i + 1];
        i += 1;

        //  MAIN SOLC PATTERN
        //  DUP1 PUSHN <SELECTOR> EQ PUSH2/3 <OFFSET> JUMPI
        //  https://github.com/ethereum/solidity/blob/58811f134ac369b20c2ec1120907321edf08fff1/libsolidity/codegen/ContractCompiler.cpp#L332
        //
        if five_seq[0].opcode == Opcode::Dup1
            && five_seq[1].opcode.is_push4_or_le()
            && five_seq[2].opcode == Opcode::Eq
            && five_seq[3].opcode.is_value_push()
            && five_seq[4].opcode == Opcode::Jumpi
        {
            let value = slice_to_selector(five_seq[1].push_value.unwrap());

            save_selector(value);

            continue;
        }

        // VYPER
        // Vyper compiler has more patterns, so we're trying to cover the most popular, but this is not a 100%
        //

        // VYPER with XOR
        // PUSHN <SELECTOR> DUP2 XOR PUSH2 <OFFSET> JUMPI
        //
        if five_seq[0].opcode.is_push4_or_le()
            && five_seq[1].opcode == Opcode::Dup2
            && five_seq[2].opcode == Opcode::Xor
            && five_seq[3].opcode.is_value_push()
            && five_seq[4].opcode == Opcode::Jumpi
        {
            let value = slice_to_selector(five_seq[0].push_value.unwrap());

            save_selector(value);

            continue;
        }

        // VYPER with MLOAD [old versions]
        // PUSHN <SELECTOR> PUSH1 00 MLOAD EQ ISZERO JUMPI
        //
        if five_seq[0].opcode.is_push4_or_le()
            && five_seq[1].opcode == Opcode::Push1
            && five_seq[2].opcode == Opcode::Mload
            && five_seq[3].opcode == Opcode::Eq
            && five_seq[4].opcode == Opcode::Iszero
        {
            let value = slice_to_selector(five_seq[0].push_value.unwrap());

            save_selector(value);
        }
    }

    selectors.into_iter().collect()
}

/// Returns all potential function selectors from the contract deployed bytecode as hex `0xa9059cbb` String
/// # Arguments
/// * `code` - A slice of bytes which represents contract bytecode
pub fn string_selectors_from_bytecode(code: &[u8]) -> Vec<String> {
    let selectors = selectors_from_bytecode(code);

    selectors
        .into_iter()
        .map(|sb| format!("0x{}", utils::bytes_to_hex(&sb)))
        .collect()
}

/// Slice to Selector type converter + Handle leading zeroes case
fn slice_to_selector(push_value: &[u8]) -> Selector {
    if push_value.len() == 4 {
        return push_value.try_into().unwrap();
    }

    // If optimized version of selector less than 4 bytes we need to add leading zeroes
    let mut selector = [0u8; 4];
    let start = 4 - push_value.len();

    for i in start..4 {
        selector[i] = push_value[i - start];
    }

    selector
}
