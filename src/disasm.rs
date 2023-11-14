use crate::opcodes::{Bytecode, Instruction, Opcode};

/// Returns parsed bytecode instructions
/// # Arguments
/// * `code` - A slice of bytes which represents contract bytecode
pub fn disasm(code: &[u8]) -> Bytecode {
    let mut code_offset = 0;

    let mut bytecode: Bytecode = Vec::new();

    while code_offset < code.len() {
        let opcode = Opcode::from(code[code_offset]);

        let push_value_size = opcode.push_value_size();

        let push_value = if opcode.is_value_push() {
            let start_i = code_offset + 1;

            // handle out of bounds edge case
            let end_i = if start_i + push_value_size > code.len() {
                code.len()
            } else {
                start_i + push_value_size
            };

            Some(&code[start_i..end_i])
        } else {
            None
        };

        bytecode.push(Instruction {
            opcode,
            push_value,
            offset: code_offset,
        });

        code_offset += 1 + push_value_size;
    }

    bytecode
}
