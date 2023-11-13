mod disasm;
mod opcodes;
mod selectors;
mod utils;

pub use disasm::*;
pub use selectors::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::Read};

    fn load_bytecode_from_file() -> Vec<u8> {
        let file_path = "testdata/0x7ae075546e8042dc263fa0eb6519ce0a04eabb93";
        let mut file = File::open(file_path).unwrap();

        let mut code = Vec::new();

        let _ = file.read_to_end(&mut code);

        code
    }

    #[test]
    fn test_selectors_from_bytecode() {
        let code = load_bytecode_from_file();

        let selectors = selectors_from_bytecode(&code);

        let expected_selectors = [
            [221, 98, 237, 62],
            [234, 22, 68, 213],
            [242, 253, 227, 139],
            [245, 12, 124, 41],
            [191, 215, 146, 132],
            [195, 200, 205, 128],
            [196, 146, 240, 70],
            [206, 79, 80, 97],
            [149, 216, 155, 65],
            [152, 165, 195, 21],
            [162, 169, 87, 187],
            [169, 5, 156, 187],
            [127, 47, 237, 220],
            [141, 165, 203, 91],
            [143, 154, 85, 192],
            [112, 160, 130, 49],
            [113, 80, 24, 166],
            [116, 1, 14, 206],
            [125, 29, 180, 165],
            [49, 60, 229, 103],
            [73, 189, 90, 94],
            [109, 138, 168, 248],
            [111, 195, 234, 236],
            [24, 22, 13, 221],
            [24, 87, 15, 136],
            [35, 184, 114, 221],
            [47, 214, 137, 227],
            [6, 253, 222, 3],
            [9, 94, 167, 179],
            [22, 148, 80, 94],
        ];

        assert_eq!(selectors, expected_selectors);
    }

    #[test]
    fn test_string_selectors_from_bytecode() {
        let code = load_bytecode_from_file();

        let selectors = string_selectors_from_bytecode(&code);

        let expected_selectors = [
            "0xdd62ed3e",
            "0xea1644d5",
            "0xf2fde38b",
            "0xf50c7c29",
            "0xbfd79284",
            "0xc3c8cd80",
            "0xc492f046",
            "0xce4f5061",
            "0x95d89b41",
            "0x98a5c315",
            "0xa2a957bb",
            "0xa9059cbb",
            "0x7f2feddc",
            "0x8da5cb5b",
            "0x8f9a55c0",
            "0x70a08231",
            "0x715018a6",
            "0x74010ece",
            "0x7d1db4a5",
            "0x313ce567",
            "0x49bd5a5e",
            "0x6d8aa8f8",
            "0x6fc3eaec",
            "0x18160ddd",
            "0x18570f88",
            "0x23b872dd",
            "0x2fd689e3",
            "0x06fdde03",
            "0x095ea7b3",
            "0x1694505e",
        ]
        .map(|s| String::from(s));

        assert_eq!(selectors, expected_selectors);
    }
}
