use crate::opcodes::Opcode;

/// A single bytecode instruction
#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operands: Vec<u32>,
    pub line: usize,
    pub column: usize,
}

impl Instruction {
    pub fn new(opcode: Opcode, operands: Vec<u32>, line: usize, column: usize) -> Self {
        Instruction {
            opcode,
            operands,
            line,
            column,
        }
    }

    /// Get the size of this instruction in bytes
    pub fn size(&self) -> usize {
        1 + self.operands.len() * 4
    }
}

/// A compiled function
#[derive(Debug, Clone)]
pub struct CompiledFunction {
    pub name: String,
    pub arity: usize,
    pub upvalue_count: usize,
    pub bytecode: Vec<Instruction>,
    pub constants: Vec<Constant>,
    pub locals: usize,
    pub is_async: bool,
}

impl CompiledFunction {
    pub fn new(name: String, arity: usize) -> Self {
        CompiledFunction {
            name,
            arity,
            upvalue_count: 0,
            bytecode: Vec::new(),
            constants: Vec::new(),
            locals: 0,
            is_async: false,
        }
    }

    /// Add a constant to the constant pool
    pub fn add_constant(&mut self, constant: Constant) -> usize {
        let index = self.constants.len();
        self.constants.push(constant);
        index
    }

    /// Emit an instruction
    pub fn emit(&mut self, instruction: Instruction) {
        self.bytecode.push(instruction);
    }

    /// Emit a simple opcode with no operands
    pub fn emit_simple(&mut self, opcode: Opcode, line: usize, column: usize) {
        self.emit(Instruction::new(opcode, vec![], line, column));
    }

    /// Emit an opcode with one operand
    pub fn emit_with_operand(&mut self, opcode: Opcode, operand: u32, line: usize, column: usize) {
        self.emit(Instruction::new(opcode, vec![operand], line, column));
    }

    /// Get the current bytecode offset
    pub fn current_offset(&self) -> usize {
        self.bytecode.len()
    }

    /// Patch a jump instruction
    pub fn patch_jump(&mut self, offset: usize) {
        let jump = self.current_offset() - offset;
        self.bytecode[offset].operands[0] = jump as u32;
    }
}

/// Constants in the constant pool
#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
    Function(CompiledFunction),
}

impl Constant {
    pub fn type_name(&self) -> &str {
        match self {
            Constant::Integer(_) => "Integer",
            Constant::Float(_) => "Float",
            Constant::String(_) => "String",
            Constant::Boolean(_) => "Boolean",
            Constant::Null => "Null",
            Constant::Function(_) => "Function",
        }
    }
}

impl std::fmt::Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Constant::Integer(n) => write!(f, "{}", n),
            Constant::Float(n) => write!(f, "{}", n),
            Constant::String(s) => write!(f, "\"{}\"", s),
            Constant::Boolean(b) => write!(f, "{}", b),
            Constant::Null => write!(f, "null"),
            Constant::Function(func) => write!(f, "<fn {}>", func.name),
        }
    }
}

/// Upvalue reference
#[derive(Debug, Clone)]
pub struct Upvalue {
    pub index: u32,
    pub is_local: bool,
}

impl Upvalue {
    pub fn new(index: u32, is_local: bool) -> Self {
        Upvalue { index, is_local }
    }
}

/// A compiled module
#[derive(Debug, Clone)]
pub struct CompiledModule {
    pub name: String,
    pub functions: Vec<CompiledFunction>,
    pub main: CompiledFunction,
    pub exports: Vec<String>,
    pub imports: Vec<String>,
}

impl CompiledModule {
    pub fn new(name: String) -> Self {
        CompiledModule {
            name,
            functions: Vec::new(),
            main: CompiledFunction::new("<main>".to_string(), 0),
            exports: Vec::new(),
            imports: Vec::new(),
        }
    }
}

/// Bytecode serializer
pub struct BytecodeSerializer {
    buffer: Vec<u8>,
}

impl BytecodeSerializer {
    pub fn new() -> Self {
        BytecodeSerializer {
            buffer: Vec::new(),
        }
    }

    /// Serialize a compiled module to bytes
    pub fn serialize(&mut self, module: &CompiledModule) -> Vec<u8> {
        self.buffer.clear();

        // Magic number
        self.write_u32(0x4E45584F); // "NEXO"

        // Version
        self.write_u32(1);

        // Module name
        self.write_string(&module.name);

        // Exports
        self.write_u32(module.exports.len() as u32);
        for export in &module.exports {
            self.write_string(export);
        }

        // Imports
        self.write_u32(module.imports.len() as u32);
        for import in &module.imports {
            self.write_string(import);
        }

        // Functions
        self.write_u32(module.functions.len() as u32);
        for func in &module.functions {
            self.serialize_function(func);
        }

        // Main function
        self.serialize_function(&module.main);

        self.buffer.clone()
    }

    fn serialize_function(&mut self, func: &CompiledFunction) {
        self.write_string(&func.name);
        self.write_u32(func.arity as u32);
        self.write_u32(func.upvalue_count as u32);
        self.write_u32(func.locals as u32);
        self.write_bool(func.is_async);

        // Constants
        self.write_u32(func.constants.len() as u32);
        for constant in &func.constants {
            self.serialize_constant(constant);
        }

        // Bytecode
        self.write_u32(func.bytecode.len() as u32);
        for instruction in &func.bytecode {
            self.write_u8(instruction.opcode as u8);
            for operand in &instruction.operands {
                self.write_u32(*operand);
            }
        }
    }

    fn serialize_constant(&mut self, constant: &Constant) {
        match constant {
            Constant::Integer(n) => {
                self.write_u8(0);
                self.write_i64(*n);
            }
            Constant::Float(n) => {
                self.write_u8(1);
                self.write_f64(*n);
            }
            Constant::String(s) => {
                self.write_u8(2);
                self.write_string(s);
            }
            Constant::Boolean(b) => {
                self.write_u8(3);
                self.write_bool(*b);
            }
            Constant::Null => {
                self.write_u8(4);
            }
            Constant::Function(func) => {
                self.write_u8(5);
                self.serialize_function(func);
            }
        }
    }

    fn write_u8(&mut self, value: u8) {
        self.buffer.push(value);
    }

    fn write_u32(&mut self, value: u32) {
        self.buffer.extend_from_slice(&value.to_le_bytes());
    }

    fn write_i64(&mut self, value: i64) {
        self.buffer.extend_from_slice(&value.to_le_bytes());
    }

    fn write_f64(&mut self, value: f64) {
        self.buffer.extend_from_slice(&value.to_le_bytes());
    }

    fn write_bool(&mut self, value: bool) {
        self.buffer.push(if value { 1 } else { 0 });
    }

    fn write_string(&mut self, value: &str) {
        self.write_u32(value.len() as u32);
        self.buffer.extend_from_slice(value.as_bytes());
    }
}

/// Bytecode deserializer
pub struct BytecodeDeserializer {
    buffer: Vec<u8>,
    position: usize,
}

impl BytecodeDeserializer {
    pub fn new(buffer: Vec<u8>) -> Self {
        BytecodeDeserializer {
            buffer,
            position: 0,
        }
    }

    /// Deserialize a compiled module from bytes
    pub fn deserialize(&mut self) -> Result<CompiledModule, String> {
        // Magic number
        let magic = self.read_u32()?;
        if magic != 0x4E45584F {
            return Err("Invalid magic number".to_string());
        }

        // Version
        let version = self.read_u32()?;
        if version != 1 {
            return Err(format!("Unsupported version: {}", version));
        }

        // Module name
        let name = self.read_string()?;

        // Exports
        let export_count = self.read_u32()?;
        let mut exports = Vec::new();
        for _ in 0..export_count {
            exports.push(self.read_string()?);
        }

        // Imports
        let import_count = self.read_u32()?;
        let mut imports = Vec::new();
        for _ in 0..import_count {
            imports.push(self.read_string()?);
        }

        // Functions
        let func_count = self.read_u32()?;
        let mut functions = Vec::new();
        for _ in 0..func_count {
            functions.push(self.deserialize_function()?);
        }

        // Main function
        let main = self.deserialize_function()?;

        Ok(CompiledModule {
            name,
            functions,
            main,
            exports,
            imports,
        })
    }

    fn deserialize_function(&mut self) -> Result<CompiledFunction, String> {
        let name = self.read_string()?;
        let arity = self.read_u32()? as usize;
        let upvalue_count = self.read_u32()? as usize;
        let locals = self.read_u32()? as usize;
        let is_async = self.read_bool()?;

        // Constants
        let const_count = self.read_u32()?;
        let mut constants = Vec::new();
        for _ in 0..const_count {
            constants.push(self.deserialize_constant()?);
        }

        // Bytecode
        let bytecode_len = self.read_u32()? as usize;
        let mut bytecode = Vec::new();
        for _ in 0..bytecode_len {
            let opcode: Opcode = self.read_u8()?.into();
            let operand_count = opcode.operand_count();
            let mut operands = Vec::new();
            for _ in 0..operand_count {
                operands.push(self.read_u32()?);
            }
            bytecode.push(Instruction {
                opcode,
                operands,
                line: 0,
                column: 0,
            });
        }

        Ok(CompiledFunction {
            name,
            arity,
            upvalue_count,
            bytecode,
            constants,
            locals,
            is_async,
        })
    }

    fn deserialize_constant(&mut self) -> Result<Constant, String> {
        let tag = self.read_u8()?;
        match tag {
            0 => Ok(Constant::Integer(self.read_i64()?)),
            1 => Ok(Constant::Float(self.read_f64()?)),
            2 => Ok(Constant::String(self.read_string()?)),
            3 => Ok(Constant::Boolean(self.read_bool()?)),
            4 => Ok(Constant::Null),
            5 => Ok(Constant::Function(self.deserialize_function()?)),
            _ => Err(format!("Unknown constant tag: {}", tag)),
        }
    }

    fn read_u8(&mut self) -> Result<u8, String> {
        if self.position >= self.buffer.len() {
            return Err("Unexpected end of buffer".to_string());
        }
        let value = self.buffer[self.position];
        self.position += 1;
        Ok(value)
    }

    fn read_u32(&mut self) -> Result<u32, String> {
        if self.position + 4 > self.buffer.len() {
            return Err("Unexpected end of buffer".to_string());
        }
        let value = u32::from_le_bytes([
            self.buffer[self.position],
            self.buffer[self.position + 1],
            self.buffer[self.position + 2],
            self.buffer[self.position + 3],
        ]);
        self.position += 4;
        Ok(value)
    }

    fn read_i64(&mut self) -> Result<i64, String> {
        if self.position + 8 > self.buffer.len() {
            return Err("Unexpected end of buffer".to_string());
        }
        let value = i64::from_le_bytes([
            self.buffer[self.position],
            self.buffer[self.position + 1],
            self.buffer[self.position + 2],
            self.buffer[self.position + 3],
            self.buffer[self.position + 4],
            self.buffer[self.position + 5],
            self.buffer[self.position + 6],
            self.buffer[self.position + 7],
        ]);
        self.position += 8;
        Ok(value)
    }

    fn read_f64(&mut self) -> Result<f64, String> {
        if self.position + 8 > self.buffer.len() {
            return Err("Unexpected end of buffer".to_string());
        }
        let value = f64::from_le_bytes([
            self.buffer[self.position],
            self.buffer[self.position + 1],
            self.buffer[self.position + 2],
            self.buffer[self.position + 3],
            self.buffer[self.position + 4],
            self.buffer[self.position + 5],
            self.buffer[self.position + 6],
            self.buffer[self.position + 7],
        ]);
        self.position += 8;
        Ok(value)
    }

    fn read_bool(&mut self) -> Result<bool, String> {
        Ok(self.read_u8()? != 0)
    }

    fn read_string(&mut self) -> Result<String, String> {
        let len = self.read_u32()? as usize;
        if self.position + len > self.buffer.len() {
            return Err("Unexpected end of buffer".to_string());
        }
        let value = String::from_utf8(self.buffer[self.position..self.position + len].to_vec())
            .map_err(|e| e.to_string())?;
        self.position += len;
        Ok(value)
    }
}

impl Default for BytecodeSerializer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_display() {
        assert_eq!(Constant::Integer(42).to_string(), "42");
        assert_eq!(Constant::Float(3.14).to_string(), "3.14");
        assert_eq!(Constant::String("hello".to_string()).to_string(), "\"hello\"");
        assert_eq!(Constant::Boolean(true).to_string(), "true");
        assert_eq!(Constant::Null.to_string(), "null");
    }

    #[test]
    fn test_compiled_function() {
        let mut func = CompiledFunction::new("test".to_string(), 2);
        assert_eq!(func.name, "test");
        assert_eq!(func.arity, 2);
        assert!(func.bytecode.is_empty());
    }

    #[test]
    fn test_serialization_roundtrip() {
        let mut module = CompiledModule::new("test".to_string());
        module.main.emit_simple(Opcode::NOP, 1, 0);
        module.main.emit_simple(Opcode::RETURN, 1, 0);

        let mut serializer = BytecodeSerializer::new();
        let bytes = serializer.serialize(&module);

        let mut deserializer = BytecodeDeserializer::new(bytes);
        let restored = deserializer.deserialize().unwrap();

        assert_eq!(restored.name, "test");
        assert_eq!(restored.main.bytecode.len(), 2);
    }
}
