use std::collections::HashMap;

use crate::ast::*;
use crate::types::{ClassType, FunctionType, TraitBound, Type, TypeContext, TypeError};

/// Type checker for Nexora
pub struct TypeChecker {
    context: TypeContext,
    errors: Vec<TypeError>,
    in_loop: bool,
    current_function: Option<String>,
}

impl TypeChecker {
    pub fn new() -> Self {
        let mut context = TypeContext::new();

        // Register built-in types
        context.add_type("Int".to_string(), Type::Int);
        context.add_type("Float".to_string(), Type::Float);
        context.add_type("String".to_string(), Type::String);
        context.add_type("Bool".to_string(), Type::Bool);
        context.add_type("Null".to_string(), Type::Null);
        context.add_type("Void".to_string(), Type::Void);

        // Register built-in functions
        let print_type = Type::Function {
            params: vec![Type::Generic {
                name: "T".to_string(),
                bounds: vec![TraitBound {
                    name: "Display".to_string(),
                    args: vec![],
                }],
            }],
            return_type: Box::new(Type::Void),
            is_async: false,
        };
        context.add_variable("print".to_string(), print_type);

        let len_type = Type::Function {
            params: vec![Type::Generic {
                name: "T".to_string(),
                bounds: vec![TraitBound {
                    name: "Iterable".to_string(),
                    args: vec![],
                }],
            }],
            return_type: Box::new(Type::Int),
            is_async: false,
        };
        context.add_variable("len".to_string(), len_type);

        TypeChecker {
            context,
            errors: Vec::new(),
            in_loop: false,
            current_function: None,
        }
    }

    /// Type check a program
    pub fn check_program(&mut self, program: &Program) -> Result<(), Vec<TypeError>> {
        for stmt in &program.stmts {
            self.check_stmt(stmt);
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    /// Type check a statement
    fn check_stmt(&mut self, stmt: &Stmt) -> Type {
        match stmt {
            Stmt::VarDecl {
                name,
                type_annotation,
                value,
                is_const,
            } => {
                let value_type = self.check_expr(value);

                if let Some(annotation) = type_annotation {
                    let declared_type = self.resolve_type_name(annotation);
                    if !value_type.is_assignable_to(&declared_type) {
                        self.errors
                            .push(TypeError::Mismatch(declared_type, value_type.clone()));
                    }
                    self.context
                        .add_variable(name.clone(), declared_type);
                } else {
                    self.context.add_variable(name.clone(), value_type);
                }

                Type::Void
            }

            Stmt::FuncDecl {
                name,
                params,
                return_type,
                body,
                is_async,
            } => {
                let param_types: Vec<Type> = params
                    .iter()
                    .map(|p| {
                        if let Some(type_ann) = &p.type_annotation {
                            self.resolve_type_name(type_ann)
                        } else {
                            self.context.new_type_var(Some(p.name.clone()))
                        }
                    })
                    .collect();

                let return = if let Some(ret_type) = return_type {
                    self.resolve_type_name(ret_type)
                } else {
                    self.context.new_type_var(Some(format!("{}_return", name)))
                };

                let func_type = Type::Function {
                    params: param_types.clone(),
                    return_type: Box::new(return.clone()),
                    is_async: *is_async,
                };

                self.context.add_variable(name.clone(), func_type);

                // Check function body
                let old_function = self.current_function.replace(name.clone());
                let old_in_loop = self.in_loop;
                self.in_loop = false;

                let mut param_names = Vec::new();
                for (param, param_type) in params.iter().zip(param_types.iter()) {
                    self.context
                        .add_variable(param.name.clone(), param_type.clone());
                    param_names.push(param.name.clone());
                }

                let body_type = self.check_block(body);

                // Verify return type
                if !body_type.is_assignable_to(&return) && return != Type::Void {
                    self.errors
                        .push(TypeError::Mismatch(return, body_type));
                }

                self.current_function = old_function;
                self.in_loop = old_in_loop;

                Type::Void
            }

            Stmt::Return(expr) => {
                if let Some(value) = expr {
                    let value_type = self.check_expr(value);
                    if let Some(func_name) = &self.current_function {
                        if let Some(func_type) = self.context.get_variable(func_name).cloned() {
                            if let Type::Function { return_type, .. } = func_type {
                                if !value_type.is_assignable_to(&return_type) {
                                    self.errors.push(TypeError::Mismatch(
                                        *return_type,
                                        value_type,
                                    ));
                                }
                            }
                        }
                    }
                    value_type
                } else {
                    Type::Void
                }
            }

            Stmt::If {
                condition,
                then_body,
                elif_clauses,
                else_body,
            } => {
                let cond_type = self.check_expr(condition);
                if cond_type != Type::Bool && cond_type != Type::Unknown {
                    self.errors.push(TypeError::Mismatch(Type::Bool, cond_type));
                }

                self.check_block(then_body);

                for (cond, body) in elif_clauses {
                    let cond_type = self.check_expr(cond);
                    if cond_type != Type::Bool && cond_type != Type::Unknown {
                        self.errors
                            .push(TypeError::Mismatch(Type::Bool, cond_type));
                    }
                    self.check_block(body);
                }

                if let Some(else_b) = else_body {
                    self.check_block(else_b);
                }

                Type::Void
            }

            Stmt::While { condition, body } => {
                let cond_type = self.check_expr(condition);
                if cond_type != Type::Bool && cond_type != Type::Unknown {
                    self.errors.push(TypeError::Mismatch(Type::Bool, cond_type));
                }

                let old_in_loop = self.in_loop;
                self.in_loop = true;
                self.check_block(body);
                self.in_loop = old_in_loop;

                Type::Void
            }

            Stmt::For {
                variable,
                iterable,
                body,
            } => {
                let iter_type = self.check_expr(iterable);

                let elem_type = match &iter_type {
                    Type::Array(inner) => (**inner).clone(),
                    Type::String => Type::Char,
                    Type::Generic { .. } => self.context.new_type_var(None),
                    _ => {
                        self.errors.push(TypeError::NotIterable(iter_type));
                        Type::Unknown
                    }
                };

                self.context
                    .add_variable(variable.clone(), elem_type);

                let old_in_loop = self.in_loop;
                self.in_loop = true;
                self.check_block(body);
                self.in_loop = old_in_loop;

                Type::Void
            }

            Stmt::Break => {
                if !self.in_loop {
                    self.errors
                        .push(TypeError::ConstraintError("break outside loop".to_string()));
                }
                Type::Never
            }

            Stmt::Continue => {
                if !self.in_loop {
                    self.errors.push(TypeError::ConstraintError(
                        "continue outside loop".to_string(),
                    ));
                }
                Type::Never
            }

            Stmt::Block(block) => {
                self.check_block(block);
                Type::Void
            }

            Stmt::ClassDecl {
                name,
                superclass,
                body,
            } => {
                let mut class_type = ClassType {
                    name: name.clone(),
                    type_params: Vec::new(),
                    superclass: None,
                    implements: Vec::new(),
                    fields: HashMap::new(),
                    methods: HashMap::new(),
                };

                // Check superclass
                if let Some(super_name) = superclass {
                    if let Some(super_type) = self.context.get_type(super_name).cloned() {
                        class_type.superclass = Some(Box::new(super_type));
                    } else {
                        self.errors
                            .push(TypeError::UndefinedType(super_name.clone()));
                    }
                }

                // Check class body
                for stmt in &body.methods {
                    if let Stmt::FuncDecl {
                        name: method_name,
                        params,
                        return_type,
                        body: method_body,
                        ..
                    } = stmt
                    {
                        let param_types: Vec<(String, Type)> = params
                            .iter()
                            .map(|p| {
                                let ty = if let Some(type_ann) = &p.type_annotation {
                                    self.resolve_type_name(type_ann)
                                } else {
                                    self.context.new_type_var(Some(p.name.clone()))
                                };
                                (p.name.clone(), ty)
                            })
                            .collect();

                        let return = if let Some(ret_type) = return_type {
                            self.resolve_type_name(ret_type)
                        } else {
                            Type::Void
                        };

                        class_type.methods.insert(
                            method_name.clone(),
                            FunctionType {
                                params: param_types,
                                return_type: return,
                                is_async: false,
                                is_method: true,
                                is_static: false,
                            },
                        );
                    }
                }

                self.context
                    .add_type(name.clone(), Type::Class(class_type));

                Type::Void
            }

            Stmt::TryCatch {
                try_body,
                catch_var,
                catch_body,
                finally_body,
            } => {
                self.check_block(try_body);

                if let Some(var_name) = catch_var {
                    self.context
                        .add_variable(var_name.clone(), Type::String);
                }

                if let Some(catch_b) = catch_body {
                    self.check_block(catch_b);
                }

                if let Some(finally_b) = finally_body {
                    self.check_block(finally_b);
                }

                Type::Void
            }

            Stmt::Throw(expr) => {
                self.check_expr(expr);
                Type::Never
            }

            Stmt::Import { .. } => Type::Void,
            Stmt::ImportFrom { .. } => Type::Void,
            Stmt::Module { .. } => Type::Void,
            Stmt::Export(stmt) => self.check_stmt(stmt),
            Stmt::TypeDecl { .. } => Type::Void,
            Stmt::InterfaceDecl { .. } => Type::Void,
            Stmt::EnumDecl { .. } => Type::Void,

            Stmt::Expr(expr) => self.check_expr(expr),
        }
    }

    /// Type check a block
    fn check_block(&mut self, block: &Block) -> Type {
        let mut last_type = Type::Void;

        for stmt in &block.stmts {
            last_type = self.check_stmt(stmt);
        }

        last_type
    }

    /// Type check an expression
    fn check_expr(&mut self, expr: &Expr) -> Type {
        match expr {
            Expr::Integer(_) => Type::Int,
            Expr::Float(_) => Type::Float,
            Expr::String(_) => Type::String,
            Expr::Boolean(_) => Type::Bool,
            Expr::Null => Type::Null,

            Expr::Array(elements) => {
                if elements.is_empty() {
                    Type::Array(Box::new(self.context.new_type_var(None)))
                } else {
                    let first_type = self.check_expr(&elements[0]);
                    for elem in &elements[1..] {
                        let elem_type = self.check_expr(elem);
                        if !elem_type.is_assignable_to(&first_type) {
                            self.errors.push(TypeError::Mismatch(
                                first_type.clone(),
                                elem_type,
                            ));
                        }
                    }
                    Type::Array(Box::new(first_type))
                }
            }

            Expr::Object(pairs) => {
                let mut fields = HashMap::new();
                for (key, value) in pairs {
                    let value_type = self.check_expr(value);
                    fields.insert(key.clone(), value_type);
                }
                // Object types are handled as maps or class instances
                Type::Map(
                    Box::new(Type::String),
                    Box::new(self.context.new_type_var(None)),
                )
            }

            Expr::Identifier(name) => self
                .context
                .get_variable(name)
                .cloned()
                .unwrap_or_else(|| {
                    self.errors
                        .push(TypeError::UndefinedVariable(name.clone()));
                    Type::Unknown
                }),

            Expr::Binary { op, left, right } => {
                let left_type = self.check_expr(left);
                let right_type = self.check_expr(right);

                match op {
                    BinaryOp::Add | BinaryOp::Subtract | BinaryOp::Multiply | BinaryOp::Divide => {
                        if left_type.is_numeric() && right_type.is_numeric() {
                            if left_type == Type::Float || right_type == Type::Float {
                                Type::Float
                            } else {
                                Type::Int
                            }
                        } else if *op == BinaryOp::Add
                            && (left_type == Type::String || right_type == Type::String)
                        {
                            Type::String
                        } else {
                            self.errors.push(TypeError::Mismatch(left_type, right_type));
                            Type::Unknown
                        }
                    }
                    BinaryOp::Modulo | BinaryOp::Power => {
                        if left_type.is_numeric() && right_type.is_numeric() {
                            left_type
                        } else {
                            self.errors.push(TypeError::NotNumeric(left_type));
                            Type::Unknown
                        }
                    }
                    BinaryOp::Equal
                    | BinaryOp::NotEqual
                    | BinaryOp::Less
                    | BinaryOp::Greater
                    | BinaryOp::LessEqual
                    | BinaryOp::GreaterEqual => {
                        if left_type.is_comparable() && right_type.is_comparable() {
                            Type::Bool
                        } else {
                            self.errors
                                .push(TypeError::NotComparable(left_type));
                            Type::Unknown
                        }
                    }
                    BinaryOp::And | BinaryOp::Or => {
                        if left_type == Type::Bool && right_type == Type::Bool {
                            Type::Bool
                        } else {
                            self.errors.push(TypeError::Mismatch(Type::Bool, left_type));
                            Type::Unknown
                        }
                    }
                    _ => Type::Unknown,
                }
            }

            Expr::Unary { op, operand } => {
                let operand_type = self.check_expr(operand);

                match op {
                    UnaryOp::Negate => {
                        if operand_type.is_numeric() {
                            operand_type
                        } else {
                            self.errors
                                .push(TypeError::NotNumeric(operand_type));
                            Type::Unknown
                        }
                    }
                    UnaryOp::Not => {
                        if operand_type == Type::Bool {
                            Type::Bool
                        } else {
                            self.errors
                                .push(TypeError::Mismatch(Type::Bool, operand_type));
                            Type::Unknown
                        }
                    }
                    UnaryOp::Increment | UnaryOp::Decrement => {
                        if operand_type.is_numeric() {
                            operand_type
                        } else {
                            self.errors
                                .push(TypeError::NotNumeric(operand_type));
                            Type::Unknown
                        }
                    }
                }
            }

            Expr::Call { callee, args } => {
                let callee_type = self.check_expr(callee);

                match &callee_type {
                    Type::Function {
                        params,
                        return_type,
                        ..
                    } => {
                        if params.len() != args.len() {
                            self.errors
                                .push(TypeError::ArityMismatch(params.len(), args.len()));
                        }

                        for (param_type, arg) in params.iter().zip(args.iter()) {
                            let arg_type = self.check_expr(arg);
                            if !arg_type.is_assignable_to(param_type) {
                                self.errors.push(TypeError::Mismatch(
                                    param_type.clone(),
                                    arg_type,
                                ));
                            }
                        }

                        (**return_type).clone()
                    }
                    _ => {
                        self.errors.push(TypeError::NotFunction(callee_type));
                        Type::Unknown
                    }
                }
            }

            Expr::MethodCall {
                object,
                method,
                args,
            } => {
                let object_type = self.check_expr(object);

                match &object_type {
                    Type::Class(class) => {
                        if let Some(method_type) = class.methods.get(method) {
                            if method_type.params.len() != args.len() {
                                self.errors.push(TypeError::ArityMismatch(
                                    method_type.params.len(),
                                    args.len(),
                                ));
                            }
                            method_type.return_type.clone()
                        } else {
                            self.errors
                                .push(TypeError::MissingField(method.clone(), object_type));
                            Type::Unknown
                        }
                    }
                    Type::Array(_) => {
                        // Built-in array methods
                        match method.as_str() {
                            "length" => Type::Int,
                            "push" | "pop" | "map" | "filter" | "reduce" => {
                                // Simplified - would need full method signatures
                                self.context.new_type_var(None)
                            }
                            _ => {
                                self.errors
                                    .push(TypeError::MissingField(method.clone(), object_type));
                                Type::Unknown
                            }
                        }
                    }
                    Type::String => match method.as_str() {
                        "length" => Type::Int,
                        "toLowerCase" | "toUpperCase" | "trim" => Type::String,
                        "includes" | "startsWith" | "endsWith" => Type::Bool,
                        "indexOf" => Type::Int,
                        _ => {
                            self.errors
                                .push(TypeError::MissingField(method.clone(), object_type));
                            Type::Unknown
                        }
                    },
                    _ => {
                        self.errors.push(TypeError::NotCallable(object_type));
                        Type::Unknown
                    }
                }
            }

            Expr::PropertyAccess { object, property } => {
                let object_type = self.check_expr(object);

                match &object_type {
                    Type::Class(class) => {
                        if let Some(field_type) = class.fields.get(property) {
                            field_type.clone()
                        } else if let Some(method_type) = class.methods.get(property) {
                            Type::Function {
                                params: method_type
                                    .params
                                    .iter()
                                    .map(|(_, t)| t.clone())
                                    .collect(),
                                return_type: Box::new(method_type.return_type.clone()),
                                is_async: false,
                            }
                        } else {
                            self.errors.push(TypeError::MissingField(
                                property.clone(),
                                object_type,
                            ));
                            Type::Unknown
                        }
                    }
                    Type::Map(_, val_type) => (**val_type).clone(),
                    _ => {
                        self.errors.push(TypeError::MissingField(
                            property.clone(),
                            object_type,
                        ));
                        Type::Unknown
                    }
                }
            }

            Expr::IndexAccess { object, index } => {
                let object_type = self.check_expr(object);
                let index_type = self.check_expr(index);

                match &object_type {
                    Type::Array(inner) => {
                        if index_type != Type::Int && index_type != Type::Unknown {
                            self.errors.push(TypeError::Mismatch(Type::Int, index_type));
                        }
                        (**inner).clone()
                    }
                    Type::Map(key_type, val_type) => {
                        if !index_type.is_assignable_to(&key_type) {
                            self.errors
                                .push(TypeError::Mismatch((**key_type).clone(), index_type));
                        }
                        (**val_type).clone()
                    }
                    _ => {
                        self.errors.push(TypeError::NotIndexable(object_type));
                        Type::Unknown
                    }
                }
            }

            Expr::Lambda { params, body } => {
                let param_types: Vec<Type> = params
                    .iter()
                    .map(|p| self.context.new_type_var(Some(p.clone())))
                    .collect();

                for (name, param_type) in params.iter().zip(param_types.iter()) {
                    self.context
                        .add_variable(name.clone(), param_type.clone());
                }

                let return_type = self.check_expr(body);

                Type::Function {
                    params: param_types,
                    return_type: Box::new(return_type),
                    is_async: false,
                }
            }

            Expr::Ternary {
                condition,
                then_expr,
                else_expr,
            } => {
                let cond_type = self.check_expr(condition);
                if cond_type != Type::Bool && cond_type != Type::Unknown {
                    self.errors.push(TypeError::Mismatch(Type::Bool, cond_type));
                }

                let then_type = self.check_expr(then_expr);
                let else_type = self.check_expr(else_expr);

                if then_type.is_assignable_to(&else_type) {
                    else_type
                } else if else_type.is_assignable_to(&then_type) {
                    then_type
                } else {
                    self.errors
                        .push(TypeError::Mismatch(then_type, else_type));
                    Type::Unknown
                }
            }

            Expr::Assign { target, value } => {
                let target_type = self.check_expr(target);
                let value_type = self.check_expr(value);

                if !value_type.is_assignable_to(&target_type) {
                    self.errors
                        .push(TypeError::Mismatch(target_type.clone(), value_type));
                }

                target_type
            }

            Expr::CompoundAssign { op, target, value } => {
                let target_type = self.check_expr(target);
                let value_type = self.check_expr(value);

                if !target_type.is_numeric() {
                    self.errors
                        .push(TypeError::NotNumeric(target_type.clone()));
                }

                if !value_type.is_assignable_to(&target_type) {
                    self.errors
                        .push(TypeError::Mismatch(target_type.clone(), value_type));
                }

                target_type
            }

            Expr::Await { expr } => {
                let expr_type = self.check_expr(expr);
                match &expr_type {
                    Type::Function { return_type, .. } => (**return_type).clone(),
                    _ => {
                        // Could be a generic awaitable
                        self.context.new_type_var(None)
                    }
                }
            }

            Expr::New { class, args } => {
                let class_type = self.check_expr(class);

                match &class_type {
                    Type::Class(class) => {
                        // Check constructor arguments
                        if let Some(init) = class.methods.get("init") {
                            if init.params.len() != args.len() {
                                self.errors.push(TypeError::ArityMismatch(
                                    init.params.len(),
                                    args.len(),
                                ));
                            }
                        }
                        class_type
                    }
                    _ => {
                        self.errors.push(TypeError::NotCallable(class_type));
                        Type::Unknown
                    }
                }
            }

            Expr::This => self
                .context
                .get_variable("this")
                .cloned()
                .unwrap_or(Type::Unknown),

            Expr::Self_ => self
                .context
                .get_variable("self")
                .cloned()
                .unwrap_or(Type::Unknown),

            Expr::Match { expr, arms } => {
                let expr_type = self.check_expr(expr);
                let mut result_type = Type::Unknown;

                for arm in arms {
                    self.check_pattern(&arm.pattern, &expr_type);

                    if let Some(guard) = &arm.guard {
                        let guard_type = self.check_expr(guard);
                        if guard_type != Type::Bool && guard_type != Type::Unknown {
                            self.errors
                                .push(TypeError::Mismatch(Type::Bool, guard_type));
                        }
                    }

                    let arm_type = self.check_expr(&arm.body);
                    if result_type == Type::Unknown {
                        result_type = arm_type;
                    } else if !arm_type.is_assignable_to(&result_type) {
                        self.errors
                            .push(TypeError::Mismatch(result_type.clone(), arm_type));
                    }
                }

                result_type
            }

            Expr::AiGenerate { .. } => Type::String,
        }
    }

    /// Check a pattern
    fn check_pattern(&mut self, pattern: &Pattern, expected_type: &Type) {
        match pattern {
            Pattern::Literal(lit) => {
                let lit_type = self.check_expr(lit);
                if !lit_type.is_assignable_to(expected_type) {
                    self.errors
                        .push(TypeError::Mismatch(expected_type.clone(), lit_type));
                }
            }
            Pattern::Identifier(name) => {
                self.context
                    .add_variable(name.clone(), expected_type.clone());
            }
            Pattern::Array(patterns) => {
                if let Type::Array(inner) = expected_type {
                    for p in patterns {
                        self.check_pattern(p, inner);
                    }
                }
            }
            Pattern::Object(pairs) => {
                // Object patterns would need field type lookup
                for (_, p) in pairs {
                    self.check_pattern(p, &Type::Unknown);
                }
            }
            Pattern::Wildcard => {}
        }
    }

    /// Resolve a type name to a Type
    fn resolve_type_name(&self, name: &str) -> Type {
        match name {
            "Int" | "int" => Type::Int,
            "Float" | "float" => Type::Float,
            "String" | "string" => Type::String,
            "Bool" | "bool" => Type::Bool,
            "Null" | "null" => Type::Null,
            "Void" | "void" => Type::Void,
            "Never" | "never" => Type::Never,
            _ => {
                if let Some(ty) = self.context.get_type(name) {
                    ty.clone()
                } else if name.starts_with('[') && name.ends_with(']') {
                    let inner = &name[1..name.len() - 1];
                    Type::Array(Box::new(self.resolve_type_name(inner)))
                } else if name.starts_with('{') && name.ends_with('}') {
                    let inner = &name[1..name.len() - 1];
                    if let Some((key, val)) = inner.split_once(", ") {
                        Type::Map(
                            Box::new(self.resolve_type_name(key)),
                            Box::new(self.resolve_type_name(val)),
                        )
                    } else {
                        Type::Unknown
                    }
                } else {
                    Type::Generic {
                        name: name.to_string(),
                        bounds: Vec::new(),
                    }
                }
            }
        }
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn check_source(source: &str) -> Result<(), Vec<TypeError>> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program().unwrap();
        let mut checker = TypeChecker::new();
        checker.check_program(&program)
    }

    #[test]
    fn test_var_decl() {
        assert!(check_source("let x = 42").is_ok());
    }

    #[test]
    fn test_type_mismatch() {
        let result = check_source(r#"
let x: String = 42
        "#);
        assert!(result.is_err());
    }

    #[test]
    fn test_function_call() {
        let result = check_source(r#"
func add(a: Int, b: Int) -> Int {
    return a + b
}
add(1, 2)
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_arity_mismatch() {
        let result = check_source(r#"
func add(a: Int, b: Int) -> Int {
    return a + b
}
add(1)
        "#);
        assert!(result.is_err());
    }

    #[test]
    fn test_if_condition() {
        let result = check_source(r#"
if true {
    let x = 1
}
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_if_non_bool() {
        let result = check_source(r#"
if 42 {
    let x = 1
}
        "#);
        assert!(result.is_err());
    }

    #[test]
    fn test_array() {
        let result = check_source("let arr = [1, 2, 3]");
        assert!(result.is_ok());
    }

    #[test]
    fn test_string_concat() {
        let result = check_source(r#"
let s = "hello" + " world"
        "#);
        assert!(result.is_ok());
    }
}
