use crate::{disasm::disasm, opcodes::Opcode};
use std::collections::HashMap;

pub fn selectors_from_bytecode(code_hex: &str) -> Vec<String> {
    let bytecode = disasm(code_hex);

    let mut potential_dests: HashMap<usize, &str> = HashMap::new();
    let mut jumpdests: Vec<usize> = Vec::new();

    for inst in bytecode {
        match inst.opcode {
            Opcode::Push4 => {
                let hex_value = inst.push_value.unwrap();
                let offset = usize::from_str_radix(hex_value, 16).unwrap_or_default();

                if !potential_dests.contains_key(&offset) {
                    potential_dests.insert(offset, hex_value);
                }
            }
            Opcode::Jumpdest => {
                jumpdests.push(inst.offset);
            }
            _ => {}
        }
    }

    let selectors: Vec<String> = potential_dests
        .into_iter()
        // .filter(|(offset, _)| jumpdests.contains(offset))
        .map(|(_, hex_value)| format!("0x{hex_value}"))
        .collect();

    selectors
}
