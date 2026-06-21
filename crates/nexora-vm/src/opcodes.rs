use std::fmt;

use serde::{Deserialize, Serialize};

/// Opcodes for the Nexora Virtual Machine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Opcode {
    // Stack operations
    NOP = 0x00,
    POP = 0x01,
    DUP = 0x02,
    SWAP = 0x03,

    // Constants
    CONST = 0x10,
    CONST_LONG = 0x11,

    // Local variables
    LOCAL = 0x20,
    SET_LOCAL = 0x21,
    LOCAL_0 = 0x22,
    LOCAL_1 = 0x23,
    LOCAL_2 = 0x24,
    LOCAL_3 = 0x25,
    SET_LOCAL_0 = 0x26,
    SET_LOCAL_1 = 0x27,
    SET_LOCAL_2 = 0x28,
    SET_LOCAL_3 = 0x29,

    // Global variables
    GLOBAL = 0x30,
    SET_GLOBAL = 0x31,

    // Upvalues (closures)
    GET_UPVALUE = 0x40,
    SET_UPVALUE = 0x41,
    CLOSE_UPVALUE = 0x42,

    // Arithmetic
    ADD = 0x50,
    SUB = 0x51,
    MUL = 0x52,
    DIV = 0x53,
    MOD = 0x54,
    POW = 0x55,
    NEG = 0x56,

    // Bitwise
    AND = 0x60,
    OR = 0x61,
    XOR = 0x62,
    NOT = 0x63,
    SHL = 0x64,
    SHR = 0x65,

    // Comparison
    EQ = 0x70,
    NEQ = 0x71,
    LT = 0x72,
    GT = 0x73,
    LTE = 0x74,
    GTE = 0x75,

    // Logical
    LAND = 0x80,
    LOR = 0x81,
    LNOT = 0x82,

    // Control flow
    JUMP = 0x90,
    JUMP_IF = 0x91,
    JUMP_IF_NOT = 0x92,
    LOOP = 0x93,

    // Functions
    CALL = 0xA0,
    CALL_METHOD = 0xA1,
    INVOKE = 0xA2,
    SUPER_INVOKE = 0xA3,
    RETURN = 0xA4,
    CLOSURE = 0xA5,

    // Objects
    NEW_OBJECT = 0xB0,
    GET_PROPERTY = 0xB1,
    SET_PROPERTY = 0xB2,
    GET_INDEX = 0xB3,
    SET_INDEX = 0xB4,
    NEW_ARRAY = 0xB5,
    ARRAY_APPEND = 0xB6,

    // Classes
    CLASS = 0xC0,
    INHERIT = 0xC1,
    METHOD = 0xC2,

    // Async
    ASYNC = 0xD0,
    AWAIT = 0xD1,
    YIELD = 0xD2,

    // Imports/Exports
    IMPORT = 0xE0,
    EXPORT = 0xE1,

    // Debug
    TRACE = 0xF0,
    BREAKPOINT = 0xF1,
}

impl Opcode {
    /// Get the name of the opcode
    pub fn name(&self) -> &str {
        match self {
            Opcode::NOP => "NOP",
            Opcode::POP => "POP",
            Opcode::DUP => "DUP",
            Opcode::SWAP => "SWAP",
            Opcode::CONST => "CONST",
            Opcode::CONST_LONG => "CONST_LONG",
            Opcode::LOCAL => "LOCAL",
            Opcode::SET_LOCAL => "SET_LOCAL",
            Opcode::LOCAL_0 => "LOCAL_0",
            Opcode::LOCAL_1 => "LOCAL_1",
            Opcode::LOCAL_2 => "LOCAL_2",
            Opcode::LOCAL_3 => "LOCAL_3",
            Opcode::SET_LOCAL_0 => "SET_LOCAL_0",
            Opcode::SET_LOCAL_1 => "SET_LOCAL_1",
            Opcode::SET_LOCAL_2 => "SET_LOCAL_2",
            Opcode::SET_LOCAL_3 => "SET_LOCAL_3",
            Opcode::GLOBAL => "GLOBAL",
            Opcode::SET_GLOBAL => "SET_GLOBAL",
            Opcode::GET_UPVALUE => "GET_UPVALUE",
            Opcode::SET_UPVALUE => "SET_UPVALUE",
            Opcode::CLOSE_UPVALUE => "CLOSE_UPVALUE",
            Opcode::ADD => "ADD",
            Opcode::SUB => "SUB",
            Opcode::MUL => "MUL",
            Opcode::DIV => "DIV",
            Opcode::MOD => "MOD",
            Opcode::POW => "POW",
            Opcode::NEG => "NEG",
            Opcode::AND => "AND",
            Opcode::OR => "OR",
            Opcode::XOR => "XOR",
            Opcode::NOT => "NOT",
            Opcode::SHL => "SHL",
            Opcode::SHR => "SHR",
            Opcode::EQ => "EQ",
            Opcode::NEQ => "NEQ",
            Opcode::LT => "LT",
            Opcode::GT => "GT",
            Opcode::LTE => "LTE",
            Opcode::GTE => "GTE",
            Opcode::LAND => "LAND",
            Opcode::LOR => "LOR",
            Opcode::LNOT => "LNOT",
            Opcode::JUMP => "JUMP",
            Opcode::JUMP_IF => "JUMP_IF",
            Opcode::JUMP_IF_NOT => "JUMP_IF_NOT",
            Opcode::LOOP => "LOOP",
            Opcode::CALL => "CALL",
            Opcode::CALL_METHOD => "CALL_METHOD",
            Opcode::INVOKE => "INVOKE",
            Opcode::SUPER_INVOKE => "SUPER_INVOKE",
            Opcode::RETURN => "RETURN",
            Opcode::CLOSURE => "CLOSURE",
            Opcode::NEW_OBJECT => "NEW_OBJECT",
            Opcode::GET_PROPERTY => "GET_PROPERTY",
            Opcode::SET_PROPERTY => "SET_PROPERTY",
            Opcode::GET_INDEX => "GET_INDEX",
            Opcode::SET_INDEX => "SET_INDEX",
            Opcode::NEW_ARRAY => "NEW_ARRAY",
            Opcode::ARRAY_APPEND => "ARRAY_APPEND",
            Opcode::CLASS => "CLASS",
            Opcode::INHERIT => "INHERIT",
            Opcode::METHOD => "METHOD",
            Opcode::ASYNC => "ASYNC",
            Opcode::AWAIT => "AWAIT",
            Opcode::YIELD => "YIELD",
            Opcode::IMPORT => "IMPORT",
            Opcode::EXPORT => "EXPORT",
            Opcode::TRACE => "TRACE",
            Opcode::BREAKPOINT => "BREAKPOINT",
        }
    }

    /// Get the number of operands for this opcode
    pub fn operand_count(&self) -> usize {
        match self {
            Opcode::NOP
            | Opcode::POP
            | Opcode::DUP
            | Opcode::SWAP
            | Opcode::ADD
            | Opcode::SUB
            | Opcode::MUL
            | Opcode::DIV
            | Opcode::MOD
            | Opcode::POW
            | Opcode::NEG
            | Opcode::AND
            | Opcode::OR
            | Opcode::XOR
            | Opcode::NOT
            | Opcode::SHL
            | Opcode::SHR
            | Opcode::EQ
            | Opcode::NEQ
            | Opcode::LT
            | Opcode::GT
            | Opcode::LTE
            | Opcode::GTE
            | Opcode::LAND
            | Opcode::LOR
            | Opcode::LNOT
            | Opcode::RETURN
            | Opcode::CLOSE_UPVALUE
            | Opcode::TRACE
            | Opcode::BREAKPOINT => 0,

            Opcode::LOCAL_0
            | Opcode::LOCAL_1
            | Opcode::LOCAL_2
            | Opcode::LOCAL_3
            | Opcode::SET_LOCAL_0
            | Opcode::SET_LOCAL_1
            | Opcode::SET_LOCAL_2
            | Opcode::SET_LOCAL_3 => 0,

            Opcode::CONST
            | Opcode::LOCAL
            | Opcode::SET_LOCAL
            | Opcode::GLOBAL
            | Opcode::SET_GLOBAL
            | Opcode::GET_UPVALUE
            | Opcode::SET_UPVALUE
            | Opcode::JUMP
            | Opcode::JUMP_IF
            | Opcode::JUMP_IF_NOT
            | Opcode::LOOP
            | Opcode::CALL
            | Opcode::CALL_METHOD
            | Opcode::INVOKE
            | Opcode::SUPER_INVOKE
            | Opcode::CLOSURE
            | Opcode::NEW_OBJECT
            | Opcode::GET_PROPERTY
            | Opcode::SET_PROPERTY
            | Opcode::GET_INDEX
            | Opcode::SET_INDEX
            | Opcode::NEW_ARRAY
            | Opcode::ARRAY_APPEND
            | Opcode::CLASS
            | Opcode::INHERIT
            | Opcode::METHOD
            | Opcode::ASYNC
            | Opcode::AWAIT
            | Opcode::YIELD
            | Opcode::IMPORT
            | Opcode::EXPORT => 1,

            Opcode::CONST_LONG => 2,
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_names() {
        assert_eq!(Opcode::NOP.name(), "NOP");
        assert_eq!(Opcode::ADD.name(), "ADD");
        assert_eq!(Opcode::RETURN.name(), "RETURN");
    }

    #[test]
    fn test_operand_count() {
        assert_eq!(Opcode::NOP.operand_count(), 0);
        assert_eq!(Opcode::ADD.operand_count(), 0);
        assert_eq!(Opcode::CONST.operand_count(), 1);
        assert_eq!(Opcode::CONST_LONG.operand_count(), 2);
    }

    #[test]
    fn test_opcode_conversion() {
        let opcode: Opcode = 0x00.into();
        assert_eq!(opcode, Opcode::NOP);

        let opcode: Opcode = 0x50.into();
        assert_eq!(opcode, Opcode::ADD);
    }
}
