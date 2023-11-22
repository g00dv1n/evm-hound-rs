//! ## EVM Hound
//! A Minimalistic Rust library to extract all potential function selectors from EVM bytecode without source code.
//!
//! For examples and more check out [the github repo](https://github.com/g00dv1n/evm-hound-rs).
//!
//! ### Use cases:
//! Made for [Hackers.tools Trading Simulator](https://hackers.tools/) to search/bruteforce for potential methods that start trading.

mod contract_types;
mod disasm;
mod opcodes;
mod selectors;
mod utils;

pub use contract_types::*;
pub use disasm::*;
pub use opcodes::*;
pub use selectors::*;

#[cfg(test)]
mod tests {
    use crate::contract_types::ContractType;

    use super::*;
    use std::{fs::File, io::Read};

    fn load_bytecode_from_file() -> Vec<u8> {
        let mut file = File::open("testdata/bytecode").unwrap();

        let mut code = Vec::new();

        let _ = file.read_to_end(&mut code);

        code
    }

    #[test]
    fn test_selectors_from_bytecode() {
        let code = load_bytecode_from_file();

        let selectors = selectors_from_bytecode(&code);

        let expected_selectors = [
            [6, 253, 222, 3],
            [7, 83, 195, 12],
            [9, 94, 167, 179],
            [14, 19, 107, 25],
            [14, 203, 147, 192],
            [24, 22, 13, 221],
            [35, 184, 114, 221],
            [38, 151, 110, 63],
            [39, 226, 53, 227],
            [49, 60, 229, 103],
            [53, 57, 7, 20],
            [62, 170, 248, 107],
            [63, 75, 168, 58],
            [89, 191, 26, 190],
            [92, 101, 129, 101],
            [92, 151, 90, 187],
            [112, 160, 130, 49],
            [132, 86, 203, 89],
            [137, 61, 32, 232],
            [141, 165, 203, 91],
            [149, 216, 155, 65],
            [169, 5, 156, 187],
            [192, 50, 76, 119],
            [204, 135, 43, 102],
            [219, 0, 106, 117],
            [221, 98, 237, 62],
            [221, 100, 79, 114],
            [228, 125, 96, 96],
            [228, 153, 125, 197],
            [229, 181, 1, 154],
            [242, 253, 227, 139],
            [243, 189, 194, 40],
        ];

        assert_eq!(selectors, expected_selectors);
    }

    #[test]
    fn test_string_selectors_from_bytecode() {
        let code = load_bytecode_from_file();

        let selectors = string_selectors_from_bytecode(&code);

        let expected_selectors = [
            "0x06fdde03",
            "0x0753c30c",
            "0x095ea7b3",
            "0x0e136b19",
            "0x0ecb93c0",
            "0x18160ddd",
            "0x23b872dd",
            "0x26976e3f",
            "0x27e235e3",
            "0x313ce567",
            "0x35390714",
            "0x3eaaf86b",
            "0x3f4ba83a",
            "0x59bf1abe",
            "0x5c658165",
            "0x5c975abb",
            "0x70a08231",
            "0x8456cb59",
            "0x893d20e8",
            "0x8da5cb5b",
            "0x95d89b41",
            "0xa9059cbb",
            "0xc0324c77",
            "0xcc872b66",
            "0xdb006a75",
            "0xdd62ed3e",
            "0xdd644f72",
            "0xe47d6060",
            "0xe4997dc5",
            "0xe5b5019a",
            "0xf2fde38b",
            "0xf3bdc228",
        ]
        .map(|s| String::from(s));

        assert_eq!(selectors, expected_selectors);
    }

    #[test]
    fn test_contract_type_from_bytecode() {
        let code = load_bytecode_from_file();

        let selectors = selectors_from_bytecode(&code);
        let contract_type = contract_type_from_selectors(&selectors);

        assert_eq!(contract_type, ContractType::ERC20);
    }
}
