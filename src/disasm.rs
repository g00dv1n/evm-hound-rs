use crate::opcodes::Opcode;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Instruction<'a> {
    pub opcode: Opcode,
    pub offset: usize,
    pub push_value: Option<&'a str>,
}

impl fmt::Display for Instruction<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let opcode_str = self.opcode.op_string();

        if let Some(push_value_str) = self.push_value {
            write!(f, "{} {}", opcode_str, push_value_str)
        } else {
            write!(f, "{}", opcode_str)
        }
    }
}

pub type Bytecode<'a> = Vec<Instruction<'a>>;

pub fn disasm(hex_str: &str) -> Bytecode {
    let mut str_offset = 0;
    let mut code_offset = 0;

    let code: &str = if hex_str.starts_with("0x") {
        &hex_str[2..]
    } else {
        hex_str
    };

    let mut bytecode: Bytecode = Vec::new();

    while str_offset < code.len() {
        let opcode = Opcode::new(&code[str_offset..str_offset + 2]);

        let push_value_size = opcode.push_value_size();
        let push_value = if opcode.is_value_push() {
            Some(&code[str_offset + 2..str_offset + 2 + push_value_size * 2])
        } else {
            None
        };

        bytecode.push(Instruction {
            opcode,
            push_value,
            offset: code_offset,
        });

        str_offset += 2 + push_value_size * 2;
        code_offset += 1 + push_value_size;
    }

    bytecode
}
