use crate::{utils, Selector};

const ERC20_DEFAULT_FUNCS: [&'static str; 6] = [
    "0xdd62ed3e", // allowance(address,address)
    "0x095ea7b3", // approve(address,uint256)
    "0x70a08231", // balanceOf(address)
    "0x18160ddd", // totalSupply()
    "0xa9059cbb", // transfer(address,uint256)
    "0x23b872dd", // transferFrom(address,address,uint256)
];

const ERC721_DEFAULT_FUNCS: [&'static str; 9] = [
    "0x70a08231", // balanceOf(address)
    "0x6352211e", // ownerOf(uint256)
    "0xb88d4fde", // safeTransferFrom(address,address,uint256,bytes)
    "0x42842e0e", // safeTransferFrom(address,address,uint256)
    "0x23b872dd", // transferFrom(address,address,uint256)
    "0x095ea7b3", // approve(address,uint256)
    "0xa22cb465", // setApprovalForAll(address,bool)
    "0x081812fc", // getApproved(uint256),
    "0xe985e9c5", // isApprovedForAll(address,address)
];

/// Available contract types to detect based on ERC interfaces.
/// Types will be added as needed (You are welcome to make a PR).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ContractType {
    /// ERC20 - Typical token contract
    ERC20,
    /// ERC721 - Typical NFT contract
    ERC721,
    /// Any other contract
    ANY,
}

impl ContractType {
    pub fn from_selectors(selectors: &Vec<Selector>) -> Self {
        if has_all_selectors(&ERC20_DEFAULT_FUNCS, selectors) {
            Self::ERC20
        } else if has_all_selectors(&ERC721_DEFAULT_FUNCS, selectors) {
            Self::ERC721
        } else {
            Self::ANY
        }
    }
}

/// Detects Contract Type (ERC20, ERC721, or ANY) using selectors extracted from bytecode
/// # Arguments
/// * `selectors` - A reference to vector of raw selectors
pub fn contract_type_from_selectors(selectors: &Vec<Selector>) -> ContractType {
    ContractType::from_selectors(selectors)
}

fn has_all_selectors(erc_defaults: &[&str], selectors: &Vec<Selector>) -> bool {
    let erc_defaults_raw: Vec<_> = erc_defaults.iter().map(str_selector_to_raw).collect();

    let matched_selectors_count = selectors
        .into_iter()
        .filter(|s| erc_defaults_raw.contains(s))
        .count();

    matched_selectors_count == erc_defaults.len()
}

fn str_selector_to_raw(s: &&str) -> Selector {
    utils::hex_to_bytes(s).unwrap().try_into().unwrap()
}
