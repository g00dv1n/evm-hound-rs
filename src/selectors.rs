use crate::{disasm::disasm, opcodes::Opcode, utils};

pub type Selector = [u8; 4];

/// Returns all potential function selectors from the contract deployed bytecode as `[u8; 4]`;
/// # Arguments
/// * `code` - A slice of bytes which represents contract bytecode
///
/// # Examples
/// ```ignore
/// use evm_hound::selectors_from_bytecode;
/// ...
/// let bytecode = provider.get_code(address, None).await?;
/// let selectors = selectors_from_bytecode(&code);
///
/// println!("{selectors:?}");
///
/// ```
pub fn selectors_from_bytecode(code: &[u8]) -> Vec<Selector> {
    let bytecode = disasm(code);

    let mut selectors: Vec<Selector> = Vec::new();
    let mut save_selector = |value: &[u8]| {
        if let Ok(selector) = value.try_into() {
            if !selectors.contains(&selector) {
                selectors.push(selector);
            }
        }
    };

    let mut i = 4usize;

    while i < bytecode.len() {
        let five_seq = &bytecode[i - 4..i + 1];
        i += 1;

        //  We're looking for a pattern that looks like:
        //  DUP1 PUSH4 <SELECTOR> EQ PUSH2/3 <OFFSET> JUMPI
        //  https://github.com/ethereum/solidity/blob/58811f134ac369b20c2ec1120907321edf08fff1/libsolidity/codegen/ContractCompiler.cpp#L332

        if five_seq[0].opcode == Opcode::Dup1
            && five_seq[1].opcode == Opcode::Push4
            && five_seq[2].opcode == Opcode::Eq
            && five_seq[3].opcode.is_value_push()
            && five_seq[4].opcode == Opcode::Jumpi
        {
            let value = five_seq[1].push_value.unwrap();

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
