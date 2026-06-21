use std::collections::HashMap;

use nexora_compiler::ast::*;
use nexora_compiler::Expr;

use crate::bytecode::{CompiledFunction, CompiledModule, Constant};
use crate::opcodes::Opcode;

/// Compiler error
#[derive(Debug, Clone)]
pub struct CompileError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Compile error at {}:{}: {}", self.line, self.column, self.message)
    }
}

impl std::error::Error for CompileError {}

/// Local variable in scope
#[derive(Debug, Clone)]
struct Local {
    name: String,
    depth: usize,
    is_captured: bool,
}

/// Upvalue being closed over
#[derive(Debug, Clone)]
struct Upvalue {
    index: u32,
    is_local: bool,
}

/// Bytecode compiler
pub struct Compiler {
    functions: Vec<CompiledFunction>,
    current_function: CompiledFunction,
    locals: Vec<Local>,
    upvalues: Vec<Upvalue>,
    scope_depth: usize,
    loop_stack: Vec<LoopContext>,
}

#[derive(Debug, Clone)]
struct LoopContext {
    start_offset: usize,
    break_jumps: Vec<usize>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            functions: Vec::new(),
            current_function: CompiledFunction::new("<main>".to_string(), 0),
            locals: Vec::new(),
            upvalues: Vec::new(),
            scope_depth: 0,
            loop_stack: Vec::new(),
        }
    }

    /// Compile a program to bytecode
    pub fn compile(&mut self, program: &Program) -> Result<CompiledModule, Vec<CompileError>> {
        let mut errors = Vec::new();

        for stmt in &program.stmts {
            if let Err(e) = self.compile_stmt(stmt) {
                errors.push(e);
            }
        }

        // Emit return
        self.current_function.emit_simple(Opcode::RETURN, 0, 0);

        if !errors.is_empty() {
            return Err(errors);
        }

        let main = std::mem::replace(
            &mut self.current_function,
            CompiledFunction::new("<main>".to_string(), 0),
        );

        Ok(CompiledModule {
            name: "<module>".to_string(),
            functions: self.functions.clone(),
            main,
            exports: Vec::new(),
            imports: Vec::new(),
        })
    }

    fn compile_stmt(&mut self, stmt: &Stmt) -> Result<(), CompileError> {
        match stmt {
            Stmt::VarDecl {
                name,
                type_annotation: _,
                value,
                is_const: _,
            } => {
                // Create a new local variable
                self.add_local(name.clone(), value.line);

                // Compile the value
                self.compile_expr(value)?;

                // Set the local
                let slot = self.resolve_local(name)?;
                self.current_function
                    .emit_with_operand(Opcode::SET_LOCAL, slot as u32, 0, 0);

                // Pop the value (we only need the assignment)
                self.current_function.emit_simple(Opcode::POP, 0, 0);

                Ok(())
            }

            Stmt::FuncDecl {
                name,
                params,
                return_type: _,
                body,
                is_async,
            } => {
                // Create nested function
                let mut func_compiler = Compiler::new();
                func_compiler.current_function.name = name.clone();
                func_compiler.current_function.arity = params.len();
                func_compiler.current_function.is_async = *is_async;

                // Add parameters as locals
                for param in params {
                    func_compiler.add_local(param.name.clone(), 0);
                }

                // Compile function body
                for stmt in &body.stmts {
                    func_compiler.compile_stmt(stmt)?;
                }

                // Emit implicit return
                func_compiler
                    .current_function
                    .emit_simple(Opcode::CONST, 0, 0);
                func_compiler.emit_constant(Constant::Null, 0, 0);
                func_compiler
                    .current_function
                    .emit_simple(Opcode::RETURN, 0, 0);

                // Store compiled function
                let func_index = self.functions.len();
                self.functions.push(func_compiler.current_function);

                // Create closure
                self.current_function
                    .emit_simple(Opcode::CLOSURE, 0, 0);
                self.current_function
                    .emit_with_operand(Opcode::CONST, func_index as u32, 0, 0);

                // Store as local
                self.add_local(name.clone(), 0);
                let slot = self.resolve_local(name)?;
                self.current_function
                    .emit_with_operand(Opcode::SET_LOCAL, slot as u32, 0, 0);
                self.current_function.emit_simple(Opcode::POP, 0, 0);

                Ok(())
            }

            Stmt::Return(expr) => {
                if let Some(value) = expr {
                    self.compile_expr(value)?;
                } else {
                    self.current_function
                        .emit_simple(Opcode::CONST, 0, 0);
                    self.emit_constant(Constant::Null, 0, 0);
                }
                self.current_function
                    .emit_simple(Opcode::RETURN, 0, 0);
                Ok(())
            }

            Stmt::If {
                condition,
                then_body,
                elif_clauses,
                else_body,
            } => {
                self.compile_expr(condition)?;

                let then_jump = self.current_function.current_offset();
                self.current_function
                    .emit_with_operand(Opcode::JUMP_IF_NOT, 0, 0, 0);

                // Then body
                for stmt in &then_body.stmts {
                    self.compile_stmt(stmt)?;
                }

                let mut end_jumps = Vec::new();

                // Elif clauses
                for (elif_cond, elif_body) in elif_clauses {
                    end_jumps.push(self.current_function.current_offset());
                    self.current_function
                        .emit_with_operand(Opcode::JUMP, 0, 0, 0);

                    self.current_function.patch_jump(then_jump);

                    self.compile_expr(elif_cond)?;
                    let elif_jump = self.current_function.current_offset();
                    self.current_function
                        .emit_with_operand(Opcode::JUMP_IF_NOT, 0, 0, 0);

                    for stmt in &elif_body.stmts {
                        self.compile_stmt(stmt)?;
                    }
                }

                // Else body
                if let Some(else_b) = else_body {
                    end_jumps.push(self.current_function.current_offset());
                    self.current_function
                        .emit_with_operand(Opcode::JUMP, 0, 0, 0);

                    self.current_function.patch_jump(then_jump);

                    for stmt in &else_b.stmts {
                        self.compile_stmt(stmt)?;
                    }
                } else {
                    self.current_function.patch_jump(then_jump);
                }

                // Patch end jumps
                for jump in end_jumps {
                    self.current_function.patch_jump(jump);
                }

                Ok(())
            }

            Stmt::While { condition, body } => {
                let loop_start = self.current_function.current_offset();

                self.compile_expr(condition)?;

                let exit_jump = self.current_function.current_offset();
                self.current_function
                    .emit_with_operand(Opcode::JUMP_IF_NOT, 0, 0, 0);

                self.loop_stack.push(LoopContext {
                    start_offset: loop_start,
                    break_jumps: Vec::new(),
                });

                for stmt in &body.stmts {
                    self.compile_stmt(stmt)?;
                }

                // Loop back
                let loop_offset = self.current_function.current_offset() - loop_start;
                self.current_function
                    .emit_with_operand(Opcode::LOOP, loop_offset as u32, 0, 0);

                // Patch exit jump
                self.current_function.patch_jump(exit_jump);

                // Patch breaks
                if let Some(loop_ctx) = self.loop_stack.pop() {
                    for break_jump in loop_ctx.break_jumps {
                        self.current_function.patch_jump(break_jump);
                    }
                }

                Ok(())
            }

            Stmt::For {
                variable,
                iterable,
                body,
            } => {
                // Compile iterable
                self.compile_expr(iterable)?;

                // Create index variable
                self.add_local("__index".to_string(), 0);
                let index_slot = self.resolve_local("__index")?;
                self.current_function
                    .emit_simple(Opcode::CONST, 0, 0);
                self.emit_constant(Constant::Integer(0), 0, 0);
                self.current_function
                    .emit_with_operand(Opcode::SET_LOCAL, index_slot as u32, 0, 0);
                self.current_function.emit_simple(Opcode::POP, 0, 0);

                // Create loop variable
                self.add_local(variable.clone(), 0);
                let var_slot = self.resolve_local(variable)?;

                let loop_start = self.current_function.current_offset();

                // Get index
                self.current_function
                    .emit_with_operand(Opcode::LOCAL, index_slot as u32, 0, 0);

                // Get array length
                // TODO: Implement array length check

                // Check if index < length
                self.current_function
                    .emit_with_operand(Opcode::LOCAL, index_slot as u32, 0, 0);

                // Push array and index for comparison
                self.current_function
                    .emit_with_operand(Opcode::LOCAL, var_slot as u32 - 1, 0, 0);

                // Get element at index
                self.current_function
                    .emit_simple(Opcode::GET_INDEX, 0, 0);
                self.current_function
                    .emit_with_operand(Opcode::SET_LOCAL, var_slot as u32, 0, 0);
                self.current_function.emit_simple(Opcode::POP, 0, 0);

                // Compile body
                self.loop_stack.push(LoopContext {
                    start_offset: loop_start,
                    break_jumps: Vec::new(),
                });

                for stmt in &body.stmts {
                    self.compile_stmt(stmt)?;
                }

                // Increment index
                self.current_function
                    .emit_with_operand(Opcode::LOCAL, index_slot as u32, 0, 0);
                self.current_function
                    .emit_simple(Opcode::CONST, 0, 0);
                self.emit_constant(Constant::Integer(1), 0, 0);
                self.current_function
                    .emit_simple(Opcode::ADD, 0, 0);
                self.current_function
                    .emit_with_operand(Opcode::SET_LOCAL, index_slot as u32, 0, 0);
                self.current_function.emit_simple(Opcode::POP, 0, 0);

                // Loop back
                let loop_offset = self.current_function.current_offset() - loop_start;
                self.current_function
                    .emit_with_operand(Opcode::LOOP, loop_offset as u32, 0, 0);

                // Patch breaks
                if let Some(loop_ctx) = self.loop_stack.pop() {
                    for break_jump in loop_ctx.break_jumps {
                        self.current_function.patch_jump(break_jump);
                    }
                }

                Ok(())
            }

            Stmt::Break => {
                if let Some(loop_ctx) = self.loop_stack.last_mut() {
                    let jump = self.current_function.current_offset();
                    self.current_function
                        .emit_with_operand(Opcode::JUMP, 0, 0, 0);
                    loop_ctx.break_jumps.push(jump);
                } else {
                    return Err(CompileError {
                        message: "break outside loop".to_string(),
                        line: 0,
                        column: 0,
                    });
                }
                Ok(())
            }

            Stmt::Continue => {
                if let Some(loop_ctx) = self.loop_stack.last() {
                    let loop_offset =
                        self.current_function.current_offset() - loop_ctx.start_offset + 1;
                    self.current_function
                        .emit_with_operand(Opcode::LOOP, loop_offset as u32, 0, 0);
                } else {
                    return Err(CompileError {
                        message: "continue outside loop".to_string(),
                        line: 0,
                        column: 0,
                    });
                }
                Ok(())
            }

            Stmt::Block(block) => {
                self.enter_scope();
                for stmt in &block.stmts {
                    self.compile_stmt(stmt)?;
                }
                self.exit_scope();
                Ok(())
            }

            Stmt::Expr(expr) => {
                self.compile_expr(expr)?;
                self.current_function.emit_simple(Opcode::POP, 0, 0);
                Ok(())
            }

            Stmt::ClassDecl {
                name,
                superclass,
                body,
            } => {
                // Create class
                self.current_function
                    .emit_simple(Opcode::CLASS, 0, 0);

                if let Some(super_name) = superclass {
                    self.current_function
                        .emit_with_operand(Opcode::GLOBAL, 0, 0);
                    // TODO: Emit super name constant
                }

                // Store class
                self.add_local(name.clone(), 0);
                let slot = self.resolve_local(name)?;
                self.current_function
                    .emit_with_operand(Opcode::SET_LOCAL, slot as u32, 0, 0);

                Ok(())
            }

            Stmt::TryCatch {
                try_body,
                catch_var: _,
                catch_body,
                finally_body,
            } => {
                // Simple try-catch implementation
                for stmt in &try_body.stmts {
                    self.compile_stmt(stmt)?;
                }

                if let Some(catch_b) = catch_body {
                    for stmt in &catch_b.stmts {
                        self.compile_stmt(stmt)?;
                    }
                }

                if let Some(finally_b) = finally_body {
                    for stmt in &finally_b.stmts {
                        self.compile_stmt(stmt)?;
                    }
                }

                Ok(())
            }

            Stmt::Throw(expr) => {
                self.compile_expr(expr)?;
                // TODO: Implement throw opcode
                Ok(())
            }

            Stmt::Import { .. } => Ok(()),
            Stmt::ImportFrom { .. } => Ok(()),
            Stmt::Module { .. } => Ok(()),
            Stmt::Export(stmt) => self.compile_stmt(stmt),
            Stmt::TypeDecl { .. } => Ok(()),
            Stmt::InterfaceDecl { .. } => Ok(()),
            Stmt::EnumDecl { .. } => Ok(()),
        }
    }

    fn compile_expr(&mut self, expr: &Expr) -> Result<(), CompileError> {
        match expr {
            Expr::Integer(n) => {
                self.current_function
                    .emit_simple(Opcode::CONST, 0, 0);
                self.emit_constant(Constant::Integer(*n), 0, 0);
                Ok(())
            }

            Expr::Float(n) => {
                self.current_function
                    .emit_simple(Opcode::CONST, 0, 0);
                self.emit_constant(Constant::Float(*n), 0, 0);
                Ok(())
            }

            Expr::String(s) => {
                self.current_function
                    .emit_simple(Opcode::CONST, 0, 0);
                self.emit_constant(Constant::String(s.clone()), 0, 0);
                Ok(())
            }

            Expr::Boolean(b) => {
                self.current_function
                    .emit_simple(Opcode::CONST, 0, 0);
                self.emit_constant(Constant::Boolean(*b), 0, 0);
                Ok(())
            }

            Expr::Null => {
                self.current_function
                    .emit_simple(Opcode::CONST, 0, 0);
                self.emit_constant(Constant::Null, 0, 0);
                Ok(())
            }

            Expr::Array(elements) => {
                for elem in elements {
                    self.compile_expr(elem)?;
                }
                self.current_function
                    .emit_with_operand(Opcode::NEW_ARRAY, elements.len() as u32, 0, 0);
                Ok(())
            }

            Expr::Object(pairs) => {
                // Create object
                self.current_function
                    .emit_simple(Opcode::NEW_OBJECT, 0, 0);

                for (key, value) in pairs {
                    // Key
                    self.current_function
                        .emit_simple(Opcode::CONST, 0, 0);
                    self.emit_constant(Constant::String(key.clone()), 0, 0);

                    // Value
                    self.compile_expr(value)?;

                    // Set property
                    self.current_function
                        .emit_simple(Opcode::SET_PROPERTY, 0, 0);
                }

                Ok(())
            }

            Expr::Identifier(name) => {
                if let Some(slot) = self.resolve_local(name) {
                    self.current_function
                        .emit_with_operand(Opcode::LOCAL, slot as u32, 0, 0);
                } else if let Some(upvalue) = self.resolve_upvalue(name) {
                    self.current_function
                        .emit_with_operand(Opcode::GET_UPVALUE, upvalue as u32, 0, 0);
                } else {
                    self.current_function
                        .emit_with_operand(Opcode::GLOBAL, 0, 0);
                    // TODO: Emit name constant for global lookup
                }
                Ok(())
            }

            Expr::Binary { op, left, right } => {
                // Short-circuit for logical operators
                match op {
                    BinaryOp::And => {
                        self.compile_expr(left)?;
                        let end_jump = self.current_function.current_offset();
                        self.current_function
                            .emit_with_operand(Opcode::JUMP_IF_NOT, 0, 0, 0);
                        self.current_function.emit_simple(Opcode::POP, 0, 0);
                        self.compile_expr(right)?;
                        self.current_function.patch_jump(end_jump);
                        return Ok(());
                    }
                    BinaryOp::Or => {
                        self.compile_expr(left)?;
                        let else_jump = self.current_function.current_offset();
                        self.current_function
                            .emit_with_operand(Opcode::JUMP_IF, 0, 0, 0);
                        self.current_function.emit_simple(Opcode::POP, 0, 0);
                        self.compile_expr(right)?;
                        self.current_function.patch_jump(else_jump);
                        return Ok(());
                    }
                    _ => {}
                }

                self.compile_expr(left)?;
                self.compile_expr(right)?;

                let opcode = match op {
                    BinaryOp::Add => Opcode::ADD,
                    BinaryOp::Subtract => Opcode::SUB,
                    BinaryOp::Multiply => Opcode::MUL,
                    BinaryOp::Divide => Opcode::DIV,
                    BinaryOp::Modulo => Opcode::MOD,
                    BinaryOp::Power => Opcode::POW,
                    BinaryOp::Equal => Opcode::EQ,
                    BinaryOp::NotEqual => Opcode::NEQ,
                    BinaryOp::Less => Opcode::LT,
                    BinaryOp::Greater => Opcode::GT,
                    BinaryOp::LessEqual => Opcode::LTE,
                    BinaryOp::GreaterEqual => Opcode::GTE,
                    BinaryOp::BitwiseAnd => Opcode::AND,
                    BinaryOp::BitwiseOr => Opcode::OR,
                    BinaryOp::BitwiseXor => Opcode::XOR,
                    BinaryOp::ShiftLeft => Opcode::SHL,
                    BinaryOp::ShiftRight => Opcode::SHR,
                    _ => return Err(CompileError {
                        message: format!("Unsupported binary operator: {:?}", op),
                        line: 0,
                        column: 0,
                    }),
                };

                self.current_function.emit_simple(opcode, 0, 0);
                Ok(())
            }

            Expr::Unary { op, operand } => {
                self.compile_expr(operand)?;

                let opcode = match op {
                    UnaryOp::Negate => Opcode::NEG,
                    UnaryOp::Not => Opcode::LNOT,
                    _ => return Err(CompileError {
                        message: format!("Unsupported unary operator: {:?}", op),
                        line: 0,
                        column: 0,
                    }),
                };

                self.current_function.emit_simple(opcode, 0, 0);
                Ok(())
            }

            Expr::Call { callee, args } => {
                // Compile arguments
                for arg in args {
                    self.compile_expr(arg)?;
                }

                // Compile callee
                self.compile_expr(callee)?;

                // Call
                self.current_function
                    .emit_with_operand(Opcode::CALL, args.len() as u32, 0, 0);

                Ok(())
            }

            Expr::MethodCall {
                object,
                method,
                args,
            } => {
                // Compile object
                self.compile_expr(object)?;

                // Compile arguments
                for arg in args {
                    self.compile_expr(arg)?;
                }

                // Method name
                self.current_function
                    .emit_simple(Opcode::CONST, 0, 0);
                self.emit_constant(Constant::String(method.clone()), 0, 0);

                // Call method
                self.current_function
                    .emit_with_operand(Opcode::CALL_METHOD, args.len() as u32, 0, 0);

                Ok(())
            }

            Expr::PropertyAccess { object, property } => {
                self.compile_expr(object)?;

                self.current_function
                    .emit_simple(Opcode::CONST, 0, 0);
                self.emit_constant(Constant::String(property.clone()), 0, 0);

                self.current_function
                    .emit_simple(Opcode::GET_PROPERTY, 0, 0);

                Ok(())
            }

            Expr::IndexAccess { object, index } => {
                self.compile_expr(object)?;
                self.compile_expr(index)?;

                self.current_function
                    .emit_simple(Opcode::GET_INDEX, 0, 0);

                Ok(())
            }

            Expr::Assign { target, value } => {
                self.compile_expr(value)?;

                match target.as_ref() {
                    Expr::Identifier(name) => {
                        if let Some(slot) = self.resolve_local(name) {
                            self.current_function
                                .emit_with_operand(Opcode::SET_LOCAL, slot as u32, 0, 0);
                        } else if let Some(upvalue) = self.resolve_upvalue(name) {
                            self.current_function
                                .emit_with_operand(Opcode::SET_UPVALUE, upvalue as u32, 0, 0);
                        } else {
                            self.current_function
                                .emit_with_operand(Opcode::SET_GLOBAL, 0, 0);
                        }
                    }
                    Expr::PropertyAccess { object, property } => {
                        self.compile_expr(object)?;
                        self.current_function
                            .emit_simple(Opcode::CONST, 0, 0);
                        self.emit_constant(Constant::String(property.clone()), 0, 0);
                        self.current_function
                            .emit_simple(Opcode::SET_PROPERTY, 0, 0);
                    }
                    Expr::IndexAccess { object, index } => {
                        self.compile_expr(object)?;
                        self.compile_expr(index)?;
                        self.current_function
                            .emit_simple(Opcode::SET_INDEX, 0, 0);
                    }
                    _ => {
                        return Err(CompileError {
                            message: "Invalid assignment target".to_string(),
                            line: 0,
                            column: 0,
                        });
                    }
                }

                Ok(())
            }

            Expr::Lambda { params, body } => {
                let mut func_compiler = Compiler::new();
                func_compiler.current_function.name = "<lambda>".to_string();
                func_compiler.current_function.arity = params.len();

                for param in params {
                    func_compiler.add_local(param.clone(), 0);
                }

                func_compiler.compile_expr(body)?;
                func_compiler
                    .current_function
                    .emit_simple(Opcode::RETURN, 0, 0);

                let func_index = self.functions.len();
                self.functions.push(func_compiler.current_function);

                self.current_function
                    .emit_simple(Opcode::CLOSURE, 0, 0);
                self.current_function
                    .emit_with_operand(Opcode::CONST, func_index as u32, 0, 0);

                Ok(())
            }

            Expr::Ternary {
                condition,
                then_expr,
                else_expr,
            } => {
                self.compile_expr(condition)?;

                let then_jump = self.current_function.current_offset();
                self.current_function
                    .emit_with_operand(Opcode::JUMP_IF_NOT, 0, 0, 0);

                self.compile_expr(then_expr)?;

                let else_jump = self.current_function.current_offset();
                self.current_function
                    .emit_with_operand(Opcode::JUMP, 0, 0, 0);

                self.current_function.patch_jump(then_jump);

                self.compile_expr(else_expr)?;

                self.current_function.patch_jump(else_jump);

                Ok(())
            }

            Expr::Await { expr } => {
                self.compile_expr(expr)?;
                self.current_function.emit_simple(Opcode::AWAIT, 0, 0);
                Ok(())
            }

            Expr::New { class, args } => {
                self.compile_expr(class)?;

                for arg in args {
                    self.compile_expr(arg)?;
                }

                self.current_function
                    .emit_with_operand(Opcode::CALL, args.len() as u32, 0, 0);

                Ok(())
            }

            Expr::This => {
                if let Some(slot) = self.resolve_local("this") {
                    self.current_function
                        .emit_with_operand(Opcode::LOCAL, slot as u32, 0, 0);
                }
                Ok(())
            }

            Expr::Self_ => {
                if let Some(slot) = self.resolve_local("self") {
                    self.current_function
                        .emit_with_operand(Opcode::LOCAL, slot as u32, 0, 0);
                }
                Ok(())
            }

            Expr::Match { expr, arms } => {
                self.compile_expr(expr)?;

                // Simple match implementation - just compile first arm
                if let Some(arm) = arms.first() {
                    self.compile_expr(&arm.body)?;
                }

                Ok(())
            }

            Expr::AiGenerate { prompt } => {
                // Emit as string for now
                self.current_function
                    .emit_simple(Opcode::CONST, 0, 0);
                self.emit_constant(
                    Constant::String(format!("[AI Generated: {}]", prompt)),
                    0,
                    0,
                );
                Ok(())
            }

            Expr::CompoundAssign { op, target, value } => {
                // Compile target
                self.compile_expr(target.clone())?;

                // Compile value
                self.compile_expr(value)?;

                // Apply operation
                let opcode = match op {
                    BinaryOp::Add => Opcode::ADD,
                    BinaryOp::Subtract => Opcode::SUB,
                    BinaryOp::Multiply => Opcode::MUL,
                    BinaryOp::Divide => Opcode::DIV,
                    _ => Opcode::ADD,
                };

                self.current_function.emit_simple(opcode, 0, 0);

                // Store back
                match target.as_ref() {
                    Expr::Identifier(name) => {
                        if let Some(slot) = self.resolve_local(name) {
                            self.current_function
                                .emit_with_operand(Opcode::SET_LOCAL, slot as u32, 0, 0);
                        }
                    }
                    _ => {}
                }

                Ok(())
            }
        }
    }

    fn add_local(&mut self, name: String, line: usize) {
        self.locals.push(Local {
            name,
            depth: self.scope_depth,
            is_captured: false,
        });
        self.current_function.locals = self.locals.len();
    }

    fn resolve_local(&self, name: &str) -> Option<u32> {
        for (i, local) in self.locals.iter().enumerate().rev() {
            if local.name == name && local.depth <= self.scope_depth {
                return Some(i as u32);
            }
        }
        None
    }

    fn resolve_upvalue(&mut self, name: &str) -> Option<u32> {
        // Check if we have an enclosing function
        // For now, return None
        None
    }

    fn enter_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn exit_scope(&mut self) {
        self.scope_depth -= 1;

        // Remove locals that are out of scope
        while let Some(local) = self.locals.last() {
            if local.depth > self.scope_depth {
                self.locals.pop();
                self.current_function.emit_simple(Opcode::POP, 0, 0);
            } else {
                break;
            }
        }
    }

    fn emit_constant(&mut self, constant: Constant, line: usize, column: usize) {
        let index = self.current_function.add_constant(constant);
        self.current_function
            .emit_with_operand(Opcode::CONST, index as u32, line, column);
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexora_compiler::Lexer;
    use nexora_compiler::Parser;

    fn compile_source(source: &str) -> Result<CompiledModule, Vec<CompileError>> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program().unwrap();
        let mut compiler = Compiler::new();
        compiler.compile(&program)
    }

    #[test]
    fn test_compile_integer() {
        let result = compile_source("42");
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_string() {
        let result = compile_source(r#""hello""#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_binary() {
        let result = compile_source("1 + 2");
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_variable() {
        let result = compile_source("let x = 10");
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_if() {
        let result = compile_source("if true { 1 } else { 2 }");
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_while() {
        let result = compile_source("while true { break }");
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_function() {
        let result = compile_source("func add(a, b) { return a + b }");
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_array() {
        let result = compile_source("[1, 2, 3]");
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_object() {
        let result = compile_source(r#"{ name: "test", value: 42 }"#);
        assert!(result.is_ok());
    }
}
