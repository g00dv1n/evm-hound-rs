use std::num::ParseIntError;

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x?}", b)).collect()
}

pub fn hex_to_bytes(hex_str: &str) -> Result<Vec<u8>, ParseIntError> {
    let hex_str = if hex_str.starts_with("0x") {
        &hex_str[2..]
    } else {
        &hex_str
    };

    (0..hex_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex_str[i..i + 2], 16))
        .collect()
}
