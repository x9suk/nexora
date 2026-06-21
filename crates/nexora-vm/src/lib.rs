pub mod bytecode;
pub mod compiler;
pub mod opcodes;
pub mod vm;

pub use bytecode::{CompiledFunction, CompiledModule, Constant};
pub use compiler::{CompileError, Compiler};
pub use opcodes::Opcode;
pub use vm::{RuntimeError, Value, VM};

/// Run source code directly
pub fn run(source: &str) -> Result<Value, RuntimeError> {
    let mut lexer = nexora_compiler::Lexer::new(source);
    let tokens = lexer
        .tokenize()
        .map_err(|e| RuntimeError::Error(e.to_string()))?;

    let mut parser = nexora_compiler::Parser::new(tokens);
    let program = parser
        .parse_program()
        .map_err(|e| RuntimeError::Error(e.to_string()))?;

    let mut compiler = Compiler::new();
    let module = compiler
        .compile(&program)
        .map_err(|e| RuntimeError::Error(e.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ")))?;

    let mut vm = VM::new();
    vm.run(&module)
}

/// Compile source code to bytecode
pub fn compile(source: &str) -> Result<CompiledModule, Vec<CompileError>> {
    let mut lexer = nexora_compiler::Lexer::new(source);
    let tokens = lexer
        .tokenize()
        .map_err(|e| vec![CompileError {
            message: e.to_string(),
            line: 0,
            column: 0,
        }])?;

    let mut parser = nexora_compiler::Parser::new(tokens);
    let program = parser
        .parse_program()
        .map_err(|e| vec![CompileError {
            message: e.to_string(),
            line: 0,
            column: 0,
        }])?;

    let mut compiler = Compiler::new();
    compiler.compile(&program)
}

/// Disassemble bytecode for debugging
pub fn disassemble(module: &CompiledModule) -> String {
    let mut output = String::new();

    output.push_str(&format!("Module: {}\n", module.name));
    output.push_str(&format!("Functions: {}\n", module.functions.len()));
    output.push_str("\n");

    // Disassemble main function
    output.push_str("=== Main Function ===\n");
    disassemble_function(&module.main, &mut output);

    // Disassemble other functions
    for (i, func) in module.functions.iter().enumerate() {
        output.push_str(&format!("\n=== Function {} ===\n", i));
        disassemble_function(func, &mut output);
    }

    output
}

fn disassemble_function(func: &CompiledFunction, output: &mut String) {
    output.push_str(&format!(
        "Function: {} (arity: {}, locals: {})\n",
        func.name, func.arity, func.locals
    ));
    output.push_str(&format!("Constants: {}\n", func.constants.len()));
    output.push_str(&format!("Bytecode: {} instructions\n", func.bytecode.len()));
    output.push_str("\n");

    for (i, instruction) in func.bytecode.iter().enumerate() {
        output.push_str(&format!(
            "{:04}: {}",
            i,
            instruction.opcode
        ));

        if !instruction.operands.is_empty() {
            let operands: Vec<String> = instruction.operands.iter().map(|o| o.to_string()).collect();
            output.push_str(&format!(" {}", operands.join(" ")));
        }

        output.push('\n');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_integer() {
        let result = run("42").unwrap();
        assert!(matches!(result, Value::Integer(42)));
    }

    #[test]
    fn test_run_string() {
        let result = run(r#""hello""#).unwrap();
        assert!(matches!(result, Value::String(s) if s == "hello"));
    }

    #[test]
    fn test_run_arithmetic() {
        let result = run("2 + 3 * 4").unwrap();
        assert!(matches!(result, Value::Integer(14)));
    }

    #[test]
    fn test_compile() {
        let result = compile("let x = 42");
        assert!(result.is_ok());
    }

    #[test]
    fn test_disassemble() {
        let module = compile("42").unwrap();
        let output = disassemble(&module);
        assert!(output.contains("Module:"));
    }
}
