use crate::utils;
use std::fmt;

/// EVM Opcodes
/// References <https://evm.codes>
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Opcode {
    /// Halts execution.
    Stop,
    /// Addition operation
    Add,
    /// Multiplication Operation
    Mul,
    /// Subtraction Operation
    Sub,
    /// Integer Division Operation
    Div,
    /// Signed Integer Division Operation
    Sdiv,
    /// Modulo Remainder Operation
    Mod,
    /// Signed Modulo Remainder Operation
    Smod,
    /// Modulo Addition Operation
    Addmod,
    /// Modulo Multiplication Operation
    Mulmod,
    /// Exponential Operation
    Exp,
    /// Extend Length of Two's Complement Signed Integer
    Signextend,
    /// Less-than Comparison
    Lt,
    /// Greater-than Comparison
    Gt,
    /// Signed Less-than Comparison
    Slt,
    /// Signed Greater-than Comparison
    Sgt,
    /// Equality Comparison
    Eq,
    /// Not Operation
    Iszero,
    /// Bitwise AND Operation
    And,
    /// Bitwise OR Operation
    Or,
    /// Bitwise XOR Operation
    Xor,
    /// Bitwise NOT Operation
    Not,
    /// Retrieve Single Byte from Word
    Byte,
    /// Left Shift Operation
    Shl,
    /// Right Shift Operation
    Shr,
    /// Arithmetic Shift Right Operation
    Sar,
    /// Compute the Keccak-256 hash of a 32-byte word
    Sha3,
    /// Address of currently executing account
    Address,
    /// Balance of a given account
    Balance,
    /// Address of execution origination
    Origin,
    /// Address of the caller
    Caller,
    /// Value of the call
    Callvalue,
    /// Loads Calldata
    Calldataload,
    /// Size of the Calldata
    Calldatasize,
    /// Copies the Calldata to Memory
    Calldatacopy,
    /// Size of the Executing Code
    Codesize,
    /// Copies Executing Code to Memory
    Codecopy,
    /// Current Price of Gas
    Gasprice,
    /// Size of an Account's Code
    Extcodesize,
    /// Copies an Account's Code to Memory
    Extcodecopy,
    /// Size of Output Data from Previous Call
    Returndatasize,
    /// Copies Output Data from Previous Call to Memory
    Returndatacopy,
    /// Hash of a Block from the most recent 256 blocks
    Blockhash,
    /// The Current Blocks Beneficiary Address
    Coinbase,
    /// The Current Blocks Timestamp
    Timestamp,
    /// The Current Blocks Number
    Number,
    /// The Current Blocks Difficulty
    Difficulty,
    /// Pseudorandomness from the Beacon Chain
    Prevrandao,
    /// The Current Blocks Gas Limit
    Gaslimit,
    /// The Chain ID
    Chainid,
    /// Balance of the Currently Executing Account
    Selfbalance,
    /// Base Fee
    Basefee,
    /// Removes an Item from the Stack
    Pop,
    /// Loads a word from Memory
    Mload,
    /// Stores a word in Memory
    Mstore,
    /// Stores a byte in Memory
    Mstore8,
    /// Load a word from Storage
    Sload,
    /// Store a word in Storage
    Sstore,
    /// Alter the Program Counter
    Jump,
    /// Conditionally Alter the Program Counter
    Jumpi,
    /// Value of the Program Counter Before the Current Instruction
    Pc,
    /// Size of Active Memory in Bytes
    Msize,
    /// Amount of available gas including the cost of the current instruction
    Gas,
    /// Marks a valid destination for jumps
    Jumpdest,
    /// Place value 0 on stack
    Push0,
    /// Places 1 byte item on top of the stack
    Push1,
    /// Places 2 byte item on top of the stack
    Push2,
    /// Places 3 byte item on top of the stack
    Push3,
    /// Places 4 byte item on top of the stack
    Push4,
    /// Places 5 byte item on top of the stack
    Push5,
    /// Places 6 byte item on top of the stack
    Push6,
    /// Places 7 byte item on top of the stack
    Push7,
    /// Places 8 byte item on top of the stack
    Push8,
    /// Places 9 byte item on top of the stack
    Push9,
    /// Places 10 byte item on top of the stack
    Push10,
    /// Places 11 byte item on top of the stack
    Push11,
    /// Places 12 byte item on top of the stack
    Push12,
    /// Places 13 byte item on top of the stack
    Push13,
    /// Places 14 byte item on top of the stack
    Push14,
    /// Places 15 byte item on top of the stack
    Push15,
    /// Places 16 byte item on top of the stack
    Push16,
    /// Places 17 byte item on top of the stack
    Push17,
    /// Places 18 byte item on top of the stack
    Push18,
    /// Places 19 byte item on top of the stack
    Push19,
    /// Places 20 byte item on top of the stack
    Push20,
    /// Places 21 byte item on top of the stack
    Push21,
    /// Places 22 byte item on top of the stack
    Push22,
    /// Places 23 byte item on top of the stack
    Push23,
    /// Places 24 byte item on top of the stack
    Push24,
    /// Places 25 byte item on top of the stack
    Push25,
    /// Places 26 byte item on top of the stack
    Push26,
    /// Places 27 byte item on top of the stack
    Push27,
    /// Places 28 byte item on top of the stack
    Push28,
    /// Places 29 byte item on top of the stack
    Push29,
    /// Places 30 byte item on top of the stack
    Push30,
    /// Places 31 byte item on top of the stack
    Push31,
    /// Places 32 byte item on top of the stack
    Push32,
    /// Duplicates the first stack item
    Dup1,
    /// Duplicates the 2nd stack item
    Dup2,
    /// Duplicates the 3rd stack item
    Dup3,
    /// Duplicates the 4th stack item
    Dup4,
    /// Duplicates the 5th stack item
    Dup5,
    /// Duplicates the 6th stack item
    Dup6,
    /// Duplicates the 7th stack item
    Dup7,
    /// Duplicates the 8th stack item
    Dup8,
    /// Duplicates the 9th stack item
    Dup9,
    /// Duplicates the 10th stack item
    Dup10,
    /// Duplicates the 11th stack item
    Dup11,
    /// Duplicates the 12th stack item
    Dup12,
    /// Duplicates the 13th stack item
    Dup13,
    /// Duplicates the 14th stack item
    Dup14,
    /// Duplicates the 15th stack item
    Dup15,
    /// Duplicates the 16th stack item
    Dup16,
    /// Exchange the top two stack items
    Swap1,
    /// Exchange the first and third stack items
    Swap2,
    /// Exchange the first and fourth stack items
    Swap3,
    /// Exchange the first and fifth stack items
    Swap4,
    /// Exchange the first and sixth stack items
    Swap5,
    /// Exchange the first and seventh stack items
    Swap6,
    /// Exchange the first and eighth stack items
    Swap7,
    /// Exchange the first and ninth stack items
    Swap8,
    /// Exchange the first and tenth stack items
    Swap9,
    /// Exchange the first and eleventh stack items
    Swap10,
    /// Exchange the first and twelfth stack items
    Swap11,
    /// Exchange the first and thirteenth stack items
    Swap12,
    /// Exchange the first and fourteenth stack items
    Swap13,
    /// Exchange the first and fifteenth stack items
    Swap14,
    /// Exchange the first and sixteenth stack items
    Swap15,
    /// Exchange the first and seventeenth stack items
    Swap16,
    /// Append Log Record with no Topics
    Log0,
    /// Append Log Record with 1 Topic
    Log1,
    /// Append Log Record with 2 Topics
    Log2,
    /// Append Log Record with 3 Topics
    Log3,
    /// Append Log Record with 4 Topics
    Log4,
    /// Create a new account with associated code
    Create,
    /// Message-call into an account
    Call,
    /// Message-call into this account with an alternative accounts code
    Callcode,
    /// Halt execution, returning output data
    Return,
    /// Message-call into this account with an alternative accounts code, persisting the sender and
    /// value
    Delegatecall,
    /// Create a new account with associated code
    Create2,
    /// Static Message-call into an account
    Staticcall,
    /// Halt execution, reverting state changes, but returning data and remaining gas
    Revert,
    /// Invalid Instruction
    Invalid,
    /// Halt Execution and Register Account for later deletion
    Selfdestruct,
    /// Get hash of an account’s code
    Extcodehash,
}

impl Opcode {
    /// Translates a raw byte into an Opcode
    pub fn from_byte(b: u8) -> Self {
        let opcode = match b {
            0x00 => Opcode::Stop,
            0x01 => Opcode::Add,
            0x02 => Opcode::Mul,
            0x03 => Opcode::Sub,
            0x04 => Opcode::Div,
            0x05 => Opcode::Sdiv,
            0x06 => Opcode::Mod,
            0x07 => Opcode::Smod,
            0x08 => Opcode::Addmod,
            0x09 => Opcode::Mulmod,
            0x0a => Opcode::Exp,
            0x0b => Opcode::Signextend,
            0x10 => Opcode::Lt,
            0x11 => Opcode::Gt,
            0x12 => Opcode::Slt,
            0x13 => Opcode::Sgt,
            0x14 => Opcode::Eq,
            0x15 => Opcode::Iszero,
            0x16 => Opcode::And,
            0x17 => Opcode::Or,
            0x18 => Opcode::Xor,
            0x19 => Opcode::Not,
            0x1a => Opcode::Byte,
            0x1b => Opcode::Shl,
            0x1c => Opcode::Shr,
            0x1d => Opcode::Sar,
            0x20 => Opcode::Sha3,
            0x30 => Opcode::Address,
            0x31 => Opcode::Balance,
            0x32 => Opcode::Origin,
            0x33 => Opcode::Caller,
            0x34 => Opcode::Callvalue,
            0x35 => Opcode::Calldataload,
            0x36 => Opcode::Calldatasize,
            0x37 => Opcode::Calldatacopy,
            0x38 => Opcode::Codesize,
            0x39 => Opcode::Codecopy,
            0x3a => Opcode::Gasprice,
            0x3b => Opcode::Extcodesize,
            0x3c => Opcode::Extcodecopy,
            0x3d => Opcode::Returndatasize,
            0x3e => Opcode::Returndatacopy,
            0x3f => Opcode::Extcodehash,
            0x40 => Opcode::Blockhash,
            0x41 => Opcode::Coinbase,
            0x42 => Opcode::Timestamp,
            0x43 => Opcode::Number,
            0x44 => Opcode::Prevrandao,
            0x45 => Opcode::Gaslimit,
            0x46 => Opcode::Chainid,
            0x47 => Opcode::Selfbalance,
            0x48 => Opcode::Basefee,
            0x50 => Opcode::Pop,
            0x51 => Opcode::Mload,
            0x52 => Opcode::Mstore,
            0x53 => Opcode::Mstore8,
            0x54 => Opcode::Sload,
            0x55 => Opcode::Sstore,
            0x56 => Opcode::Jump,
            0x57 => Opcode::Jumpi,
            0x58 => Opcode::Pc,
            0x59 => Opcode::Msize,
            0x5a => Opcode::Gas,
            0x5b => Opcode::Jumpdest,
            0x60 => Opcode::Push1,
            0x61 => Opcode::Push2,
            0x62 => Opcode::Push3,
            0x63 => Opcode::Push4,
            0x64 => Opcode::Push5,
            0x65 => Opcode::Push6,
            0x66 => Opcode::Push7,
            0x67 => Opcode::Push8,
            0x68 => Opcode::Push9,
            0x69 => Opcode::Push10,
            0x6a => Opcode::Push11,
            0x6b => Opcode::Push12,
            0x6c => Opcode::Push13,
            0x6d => Opcode::Push14,
            0x6e => Opcode::Push15,
            0x6f => Opcode::Push16,
            0x70 => Opcode::Push17,
            0x71 => Opcode::Push18,
            0x72 => Opcode::Push19,
            0x73 => Opcode::Push20,
            0x74 => Opcode::Push21,
            0x75 => Opcode::Push22,
            0x76 => Opcode::Push23,
            0x77 => Opcode::Push24,
            0x78 => Opcode::Push25,
            0x79 => Opcode::Push26,
            0x7a => Opcode::Push27,
            0x7b => Opcode::Push28,
            0x7c => Opcode::Push29,
            0x7d => Opcode::Push30,
            0x7e => Opcode::Push31,
            0x7f => Opcode::Push32,
            0x80 => Opcode::Dup1,
            0x81 => Opcode::Dup2,
            0x82 => Opcode::Dup3,
            0x83 => Opcode::Dup4,
            0x84 => Opcode::Dup5,
            0x85 => Opcode::Dup6,
            0x86 => Opcode::Dup7,
            0x87 => Opcode::Dup8,
            0x88 => Opcode::Dup9,
            0x89 => Opcode::Dup10,
            0x8a => Opcode::Dup11,
            0x8b => Opcode::Dup12,
            0x8c => Opcode::Dup13,
            0x8d => Opcode::Dup14,
            0x8e => Opcode::Dup15,
            0x8f => Opcode::Dup16,
            0x90 => Opcode::Swap1,
            0x91 => Opcode::Swap2,
            0x92 => Opcode::Swap3,
            0x93 => Opcode::Swap4,
            0x94 => Opcode::Swap5,
            0x95 => Opcode::Swap6,
            0x96 => Opcode::Swap7,
            0x97 => Opcode::Swap8,
            0x98 => Opcode::Swap9,
            0x99 => Opcode::Swap10,
            0x9a => Opcode::Swap11,
            0x9b => Opcode::Swap12,
            0x9c => Opcode::Swap13,
            0x9d => Opcode::Swap14,
            0x9e => Opcode::Swap15,
            0x9f => Opcode::Swap16,
            0xa0 => Opcode::Log0,
            0xa1 => Opcode::Log1,
            0xa2 => Opcode::Log2,
            0xa3 => Opcode::Log3,
            0xa4 => Opcode::Log4,
            0xf0 => Opcode::Create,
            0xf1 => Opcode::Call,
            0xf2 => Opcode::Callcode,
            0xf3 => Opcode::Return,
            0xf4 => Opcode::Delegatecall,
            0xf5 => Opcode::Create2,
            0xfa => Opcode::Staticcall,
            0xfd => Opcode::Revert,
            0xfe => Opcode::Invalid,
            0xff => Opcode::Selfdestruct,
            _ => Opcode::Invalid,
        };
        opcode
    }

    /// Check if this is push with value (Push0 will be ignored)
    pub fn is_value_push(&self) -> bool {
        match self {
            Opcode::Push1
            | Opcode::Push2
            | Opcode::Push3
            | Opcode::Push4
            | Opcode::Push5
            | Opcode::Push6
            | Opcode::Push7
            | Opcode::Push8
            | Opcode::Push9
            | Opcode::Push10
            | Opcode::Push11
            | Opcode::Push12
            | Opcode::Push13
            | Opcode::Push14
            | Opcode::Push15
            | Opcode::Push16
            | Opcode::Push17
            | Opcode::Push18
            | Opcode::Push19
            | Opcode::Push20
            | Opcode::Push21
            | Opcode::Push22
            | Opcode::Push23
            | Opcode::Push24
            | Opcode::Push25
            | Opcode::Push26
            | Opcode::Push27
            | Opcode::Push28
            | Opcode::Push29
            | Opcode::Push30
            | Opcode::Push31
            | Opcode::Push32 => true,
            _ => false,
        }
    }

    /// Returns push value size
    pub fn push_value_size(&self) -> usize {
        match self {
            Opcode::Push1 => 1,
            Opcode::Push2 => 2,
            Opcode::Push3 => 3,
            Opcode::Push4 => 4,
            Opcode::Push5 => 5,
            Opcode::Push6 => 6,
            Opcode::Push7 => 7,
            Opcode::Push8 => 8,
            Opcode::Push9 => 9,
            Opcode::Push10 => 10,
            Opcode::Push11 => 11,
            Opcode::Push12 => 12,
            Opcode::Push13 => 13,
            Opcode::Push14 => 14,
            Opcode::Push15 => 15,
            Opcode::Push16 => 16,
            Opcode::Push17 => 17,
            Opcode::Push18 => 18,
            Opcode::Push19 => 19,
            Opcode::Push20 => 20,
            Opcode::Push21 => 21,
            Opcode::Push22 => 22,
            Opcode::Push23 => 23,
            Opcode::Push24 => 24,
            Opcode::Push25 => 25,
            Opcode::Push26 => 26,
            Opcode::Push27 => 27,
            Opcode::Push28 => 28,
            Opcode::Push29 => 29,
            Opcode::Push30 => 30,
            Opcode::Push31 => 31,
            Opcode::Push32 => 32,
            _ => 0,
        }
    }

    /// Translates an Opcode into a byte
    pub fn byte(&self) -> u8 {
        match self {
            Opcode::Stop => 0x00,
            Opcode::Add => 0x01,
            Opcode::Mul => 0x02,
            Opcode::Sub => 0x03,
            Opcode::Div => 0x04,
            Opcode::Sdiv => 0x05,
            Opcode::Mod => 0x06,
            Opcode::Smod => 0x07,
            Opcode::Addmod => 0x08,
            Opcode::Mulmod => 0x09,
            Opcode::Exp => 0x0a,
            Opcode::Signextend => 0x0b,
            Opcode::Lt => 0x10,
            Opcode::Gt => 0x11,
            Opcode::Slt => 0x12,
            Opcode::Sgt => 0x13,
            Opcode::Eq => 0x14,
            Opcode::Iszero => 0x15,
            Opcode::And => 0x16,
            Opcode::Or => 0x17,
            Opcode::Xor => 0x18,
            Opcode::Not => 0x19,
            Opcode::Byte => 0x1a,
            Opcode::Shl => 0x1b,
            Opcode::Shr => 0x1c,
            Opcode::Sar => 0x1d,
            Opcode::Sha3 => 0x20,
            Opcode::Address => 0x30,
            Opcode::Balance => 0x31,
            Opcode::Origin => 0x32,
            Opcode::Caller => 0x33,
            Opcode::Callvalue => 0x34,
            Opcode::Calldataload => 0x35,
            Opcode::Calldatasize => 0x36,
            Opcode::Calldatacopy => 0x37,
            Opcode::Codesize => 0x38,
            Opcode::Codecopy => 0x39,
            Opcode::Gasprice => 0x3a,
            Opcode::Extcodesize => 0x3b,
            Opcode::Extcodecopy => 0x3c,
            Opcode::Returndatasize => 0x3d,
            Opcode::Returndatacopy => 0x3e,
            Opcode::Extcodehash => 0x3f,
            Opcode::Blockhash => 0x40,
            Opcode::Coinbase => 0x41,
            Opcode::Timestamp => 0x42,
            Opcode::Number => 0x43,
            Opcode::Difficulty => 0x44,
            Opcode::Prevrandao => 0x44,
            Opcode::Gaslimit => 0x45,
            Opcode::Chainid => 0x46,
            Opcode::Selfbalance => 0x47,
            Opcode::Basefee => 0x48,
            Opcode::Pop => 0x50,
            Opcode::Mload => 0x51,
            Opcode::Mstore => 0x52,
            Opcode::Mstore8 => 0x53,
            Opcode::Sload => 0x54,
            Opcode::Sstore => 0x55,
            Opcode::Jump => 0x56,
            Opcode::Jumpi => 0x57,
            Opcode::Pc => 0x58,
            Opcode::Msize => 0x59,
            Opcode::Gas => 0x5a,
            Opcode::Jumpdest => 0x5b,
            Opcode::Push0 => 0x5f,
            Opcode::Push1 => 0x60,
            Opcode::Push2 => 0x61,
            Opcode::Push3 => 0x62,
            Opcode::Push4 => 0x63,
            Opcode::Push5 => 0x64,
            Opcode::Push6 => 0x65,
            Opcode::Push7 => 0x66,
            Opcode::Push8 => 0x67,
            Opcode::Push9 => 0x68,
            Opcode::Push10 => 0x69,
            Opcode::Push11 => 0x6a,
            Opcode::Push12 => 0x6b,
            Opcode::Push13 => 0x6c,
            Opcode::Push14 => 0x6d,
            Opcode::Push15 => 0x6e,
            Opcode::Push16 => 0x6f,
            Opcode::Push17 => 0x70,
            Opcode::Push18 => 0x71,
            Opcode::Push19 => 0x72,
            Opcode::Push20 => 0x73,
            Opcode::Push21 => 0x74,
            Opcode::Push22 => 0x75,
            Opcode::Push23 => 0x76,
            Opcode::Push24 => 0x77,
            Opcode::Push25 => 0x78,
            Opcode::Push26 => 0x79,
            Opcode::Push27 => 0x7a,
            Opcode::Push28 => 0x7b,
            Opcode::Push29 => 0x7c,
            Opcode::Push30 => 0x7d,
            Opcode::Push31 => 0x7e,
            Opcode::Push32 => 0x7f,
            Opcode::Dup1 => 0x80,
            Opcode::Dup2 => 0x81,
            Opcode::Dup3 => 0x82,
            Opcode::Dup4 => 0x83,
            Opcode::Dup5 => 0x84,
            Opcode::Dup6 => 0x85,
            Opcode::Dup7 => 0x86,
            Opcode::Dup8 => 0x87,
            Opcode::Dup9 => 0x88,
            Opcode::Dup10 => 0x89,
            Opcode::Dup11 => 0x8a,
            Opcode::Dup12 => 0x8b,
            Opcode::Dup13 => 0x8c,
            Opcode::Dup14 => 0x8d,
            Opcode::Dup15 => 0x8e,
            Opcode::Dup16 => 0x8f,
            Opcode::Swap1 => 0x90,
            Opcode::Swap2 => 0x91,
            Opcode::Swap3 => 0x92,
            Opcode::Swap4 => 0x93,
            Opcode::Swap5 => 0x94,
            Opcode::Swap6 => 0x95,
            Opcode::Swap7 => 0x96,
            Opcode::Swap8 => 0x97,
            Opcode::Swap9 => 0x98,
            Opcode::Swap10 => 0x99,
            Opcode::Swap11 => 0x9a,
            Opcode::Swap12 => 0x9b,
            Opcode::Swap13 => 0x9c,
            Opcode::Swap14 => 0x9d,
            Opcode::Swap15 => 0x9e,
            Opcode::Swap16 => 0x9f,
            Opcode::Log0 => 0xa0,
            Opcode::Log1 => 0xa1,
            Opcode::Log2 => 0xa2,
            Opcode::Log3 => 0xa3,
            Opcode::Log4 => 0xa4,
            Opcode::Create => 0xf0,
            Opcode::Call => 0xf1,
            Opcode::Callcode => 0xf2,
            Opcode::Return => 0xf3,
            Opcode::Delegatecall => 0xf4,
            Opcode::Create2 => 0xf5,
            Opcode::Staticcall => 0xfa,
            Opcode::Revert => 0xfd,
            Opcode::Invalid => 0xfe,
            Opcode::Selfdestruct => 0xff,
        }
    }

    /// Translates an Opcode into opcode string
    pub fn op_string(&self) -> String {
        let opcode_str = match self {
            Opcode::Stop => "Stop",
            Opcode::Add => "Add",
            Opcode::Mul => "Mul",
            Opcode::Sub => "Sub",
            Opcode::Div => "Div",
            Opcode::Sdiv => "Sdiv",
            Opcode::Mod => "Mod",
            Opcode::Smod => "Smod",
            Opcode::Addmod => "Addmod",
            Opcode::Mulmod => "Mulmod",
            Opcode::Exp => "Exp",
            Opcode::Signextend => "Signextend",
            Opcode::Lt => "Lt",
            Opcode::Gt => "Gt",
            Opcode::Slt => "Slt",
            Opcode::Sgt => "Sgt",
            Opcode::Eq => "Eq",
            Opcode::Iszero => "Iszero",
            Opcode::And => "And",
            Opcode::Or => "Or",
            Opcode::Xor => "Xor",
            Opcode::Not => "Not",
            Opcode::Byte => "Byte",
            Opcode::Shl => "Shl",
            Opcode::Shr => "Shr",
            Opcode::Sar => "Sar",
            Opcode::Sha3 => "Sha3",
            Opcode::Address => "Address",
            Opcode::Balance => "Balance",
            Opcode::Origin => "Origin",
            Opcode::Caller => "Caller",
            Opcode::Callvalue => "Callvalue",
            Opcode::Calldataload => "Calldataload",
            Opcode::Calldatasize => "Calldatasize",
            Opcode::Calldatacopy => "Calldatacopy",
            Opcode::Codesize => "Codesize",
            Opcode::Codecopy => "Codecopy",
            Opcode::Gasprice => "Gasprice",
            Opcode::Extcodesize => "Extcodesize",
            Opcode::Extcodecopy => "Extcodecopy",
            Opcode::Returndatasize => "Returndatasize",
            Opcode::Returndatacopy => "Returndatacopy",
            Opcode::Extcodehash => "Extcodehash",
            Opcode::Blockhash => "Blockhash",
            Opcode::Coinbase => "Coinbase",
            Opcode::Timestamp => "Timestamp",
            Opcode::Number => "Number",
            Opcode::Difficulty => "Difficulty",
            Opcode::Prevrandao => "Prevrandao",
            Opcode::Gaslimit => "Gaslimit",
            Opcode::Chainid => "Chainid",
            Opcode::Selfbalance => "Selfbalance",
            Opcode::Basefee => "Basefee",
            Opcode::Pop => "Pop",
            Opcode::Mload => "Mload",
            Opcode::Mstore => "Mstore",
            Opcode::Mstore8 => "Mstore8",
            Opcode::Sload => "Sload",
            Opcode::Sstore => "Sstore",
            Opcode::Jump => "Jump",
            Opcode::Jumpi => "Jumpi",
            Opcode::Pc => "Pc",
            Opcode::Msize => "Msize",
            Opcode::Gas => "Gas",
            Opcode::Jumpdest => "Jumpdest",
            Opcode::Push0 => "Push0",
            Opcode::Push1 => "Push1",
            Opcode::Push2 => "Push2",
            Opcode::Push3 => "Push3",
            Opcode::Push4 => "Push4",
            Opcode::Push5 => "Push5",
            Opcode::Push6 => "Push6",
            Opcode::Push7 => "Push7",
            Opcode::Push8 => "Push8",
            Opcode::Push9 => "Push9",
            Opcode::Push10 => "Push10",
            Opcode::Push11 => "Push11",
            Opcode::Push12 => "Push12",
            Opcode::Push13 => "Push13",
            Opcode::Push14 => "Push14",
            Opcode::Push15 => "Push15",
            Opcode::Push16 => "Push16",
            Opcode::Push17 => "Push17",
            Opcode::Push18 => "Push18",
            Opcode::Push19 => "Push19",
            Opcode::Push20 => "Push20",
            Opcode::Push21 => "Push21",
            Opcode::Push22 => "Push22",
            Opcode::Push23 => "Push23",
            Opcode::Push24 => "Push24",
            Opcode::Push25 => "Push25",
            Opcode::Push26 => "Push26",
            Opcode::Push27 => "Push27",
            Opcode::Push28 => "Push28",
            Opcode::Push29 => "Push29",
            Opcode::Push30 => "Push30",
            Opcode::Push31 => "Push31",
            Opcode::Push32 => "Push32",
            Opcode::Dup1 => "Dup1",
            Opcode::Dup2 => "Dup2",
            Opcode::Dup3 => "Dup3",
            Opcode::Dup4 => "Dup4",
            Opcode::Dup5 => "Dup5",
            Opcode::Dup6 => "Dup6",
            Opcode::Dup7 => "Dup7",
            Opcode::Dup8 => "Dup8",
            Opcode::Dup9 => "Dup9",
            Opcode::Dup10 => "Dup10",
            Opcode::Dup11 => "Dup11",
            Opcode::Dup12 => "Dup12",
            Opcode::Dup13 => "Dup13",
            Opcode::Dup14 => "Dup14",
            Opcode::Dup15 => "Dup15",
            Opcode::Dup16 => "Dup16",
            Opcode::Swap1 => "Swap1",
            Opcode::Swap2 => "Swap2",
            Opcode::Swap3 => "Swap3",
            Opcode::Swap4 => "Swap4",
            Opcode::Swap5 => "Swap5",
            Opcode::Swap6 => "Swap6",
            Opcode::Swap7 => "Swap7",
            Opcode::Swap8 => "Swap8",
            Opcode::Swap9 => "Swap9",
            Opcode::Swap10 => "Swap10",
            Opcode::Swap11 => "Swap11",
            Opcode::Swap12 => "Swap12",
            Opcode::Swap13 => "Swap13",
            Opcode::Swap14 => "Swap14",
            Opcode::Swap15 => "Swap15",
            Opcode::Swap16 => "Swap16",
            Opcode::Log0 => "Log0",
            Opcode::Log1 => "Log1",
            Opcode::Log2 => "Log2",
            Opcode::Log3 => "Log3",
            Opcode::Log4 => "Log4",
            Opcode::Create => "Create",
            Opcode::Call => "Call",
            Opcode::Callcode => "Callcode",
            Opcode::Return => "Return",
            Opcode::Delegatecall => "Delegatecall",
            Opcode::Create2 => "Create2",
            Opcode::Staticcall => "Staticcall",
            Opcode::Revert => "Revert",
            Opcode::Invalid => "Invalid",
            Opcode::Selfdestruct => "Selfdestruct",
        };
        opcode_str.to_string()
    }
}
/// Bytecode Instruction
#[derive(Debug, Clone)]
pub struct Instruction<'a> {
    /// EVM Opcode
    pub opcode: Opcode,
    /// Instruction offset in the current code
    pub offset: usize,
    /// Value for Push Opcodes (except for Push0)
    pub push_value: Option<&'a [u8]>,
}

/// Display Instruction Opcode and Push value (if has one)
impl fmt::Display for Instruction<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let opcode_str = self.opcode.op_string();

        if let Some(push_value) = self.push_value {
            let hex_push_value = utils::bytes_to_hex(push_value);

            write!(f, "{} {}", opcode_str, hex_push_value)
        } else {
            write!(f, "{}", opcode_str)
        }
    }
}

pub type Bytecode<'a> = Vec<Instruction<'a>>;
