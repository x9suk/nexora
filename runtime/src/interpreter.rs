use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use nexora_compiler::ast::*;
use nexora_compiler::Expr;

use crate::value::{Closure, Environment, Function, RuntimeError, Value};

pub struct Interpreter {
    env: Arc<Mutex<Environment>>,
    call_depth: usize,
    max_call_depth: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut env = Environment::new();
        Self::setup_builtins(&mut env);

        Interpreter {
            env: Arc::new(Mutex::new(env)),
            call_depth: 0,
            max_call_depth: 1000,
        }
    }

    fn setup_builtins(env: &mut Environment) {
        // Print function
        env.define(
            "print".to_string(),
            Value::NativeFunction(crate::value::NativeFunction {
                name: "print".to_string(),
                func: Arc::new(|args| {
                    let output: Vec<String> = args.iter().map(|a| a.to_string_value()).collect();
                    println!("{}", output.join(" "));
                    Ok(Value::Null)
                }),
            }),
        );

        // Typeof function
        env.define(
            "typeof".to_string(),
            Value::NativeFunction(crate::value::NativeFunction {
                name: "typeof".to_string(),
                func: Arc::new(|args| {
                    if args.len() != 1 {
                        return Err(RuntimeError::Error("typeof requires 1 argument".into()));
                    }
                    Ok(Value::String(args[0].type_name().to_string()))
                }),
            }),
        );

        // Length function
        env.define(
            "len".to_string(),
            Value::NativeFunction(crate::value::NativeFunction {
                name: "len".to_string(),
                func: Arc::new(|args| {
                    if args.len() != 1 {
                        return Err(RuntimeError::Error("len requires 1 argument".into()));
                    }
                    match &args[0] {
                        Value::String(s) => Ok(Value::Integer(s.len() as i64)),
                        Value::Array(arr) => Ok(Value::Integer(arr.len() as i64)),
                        Value::Object(obj) => Ok(Value::Integer(obj.len() as i64)),
                        _ => Err(RuntimeError::TypeError(format!(
                            "Cannot get length of {}",
                            args[0].type_name()
                        ))),
                    }
                }),
            }),
        );

        // String conversion
        env.define(
            "str".to_string(),
            Value::NativeFunction(crate::value::NativeFunction {
                name: "str".to_string(),
                func: Arc::new(|args| {
                    if args.len() != 1 {
                        return Err(RuntimeError::Error("str requires 1 argument".into()));
                    }
                    Ok(Value::String(args[0].to_string_value()))
                }),
            }),
        );

        // Number conversion
        env.define(
            "num".to_string(),
            Value::NativeFunction(crate::value::NativeFunction {
                name: "num".to_string(),
                func: Arc::new(|args| {
                    if args.len() != 1 {
                        return Err(RuntimeError::Error("num requires 1 argument".into()));
                    }
                    match &args[0] {
                        Value::String(s) => {
                            if let Ok(n) = s.parse::<i64>() {
                                Ok(Value::Integer(n))
                            } else if let Ok(n) = s.parse::<f64>() {
                                Ok(Value::Float(n))
                            } else {
                                Err(RuntimeError::TypeError("Cannot convert string to number".into()))
                            }
                        }
                        Value::Integer(n) => Ok(Value::Integer(*n)),
                        Value::Float(n) => Ok(Value::Float(*n)),
                        _ => Err(RuntimeError::TypeError(format!(
                            "Cannot convert {} to number",
                            args[0].type_name()
                        ))),
                    }
                }),
            }),
        );

        // ParseInt
        env.define(
            "parseInt".to_string(),
            Value::NativeFunction(crate::value::NativeFunction {
                name: "parseInt".to_string(),
                func: Arc::new(|args| {
                    if args.len() != 1 {
                        return Err(RuntimeError::Error("parseInt requires 1 argument".into()));
                    }
                    match &args[0] {
                        Value::String(s) => s
                            .parse::<i64>()
                            .map(Value::Integer)
                            .map_err(|_| RuntimeError::TypeError("Invalid integer".into())),
                        Value::Integer(n) => Ok(Value::Integer(*n)),
                        _ => Err(RuntimeError::TypeError(format!(
                            "Cannot parse {} as integer",
                            args[0].type_name()
                        ))),
                    }
                }),
            }),
        );

        // ParseFloat
        env.define(
            "parseFloat".to_string(),
            Value::NativeFunction(crate::value::NativeFunction {
                name: "parseFloat".to_string(),
                func: Arc::new(|args| {
                    if args.len() != 1 {
                        return Err(RuntimeError::Error("parseFloat requires 1 argument".into()));
                    }
                    match &args[0] {
                        Value::String(s) => s
                            .parse::<f64>()
                            .map(Value::Float)
                            .map_err(|_| RuntimeError::TypeError("Invalid float".into())),
                        Value::Float(n) => Ok(Value::Float(*n)),
                        Value::Integer(n) => Ok(Value::Float(*n as f64)),
                        _ => Err(RuntimeError::TypeError(format!(
                            "Cannot parse {} as float",
                            args[0].type_name()
                        ))),
                    }
                }),
            }),
        );

        // Array push
        env.define(
            "push".to_string(),
            Value::NativeFunction(crate::value::NativeFunction {
                name: "push".to_string(),
                func: Arc::new(|args| {
                    if args.len() < 2 {
                        return Err(RuntimeError::Error("push requires at least 2 arguments".into()));
                    }
                    match &args[0] {
                        Value::Array(arr) => {
                            let mut new_arr = arr.clone();
                            new_arr.extend_from_slice(&args[1..]);
                            Ok(Value::Array(new_arr))
                        }
                        _ => Err(RuntimeError::TypeError(format!(
                            "Cannot push to {}",
                            args[0].type_name()
                        ))),
                    }
                }),
            }),
        );

        // Array pop
        env.define(
            "pop".to_string(),
            Value::NativeFunction(crate::value::NativeFunction {
                name: "pop".to_string(),
                func: Arc::new(|args| {
                    if args.len() != 1 {
                        return Err(RuntimeError::Error("pop requires 1 argument".into()));
                    }
                    match &args[0] {
                        Value::Array(arr) => {
                            if arr.is_empty() {
                                Ok(Value::Null)
                            } else {
                                let mut new_arr = arr.clone();
                                Ok(new_arr.pop().unwrap())
                            }
                        }
                        _ => Err(RuntimeError::TypeError(format!(
                            "Cannot pop from {}",
                            args[0].type_name()
                        ))),
                    }
                }),
            }),
        );

        // String split
        env.define(
            "split".to_string(),
            Value::NativeFunction(crate::value::NativeFunction {
                name: "split".to_string(),
                func: Arc::new(|args| {
                    if args.len() < 2 {
                        return Err(RuntimeError::Error("split requires 2 arguments".into()));
                    }
                    match (&args[0], &args[1]) {
                        (Value::String(s), Value::String(delimiter)) => {
                            let parts: Vec<Value> = s
                                .split(delimiter)
                                .map(|p| Value::String(p.to_string()))
                                .collect();
                            Ok(Value::Array(parts))
                        }
                        _ => Err(RuntimeError::TypeError("split requires two strings".into())),
                    }
                }),
            }),
        );

        // String join
        env.define(
            "join".to_string(),
            Value::NativeFunction(crate::value::NativeFunction {
                name: "join".to_string(),
                func: Arc::new(|args| {
                    if args.len() < 2 {
                        return Err(RuntimeError::Error("join requires 2 arguments".into()));
                    }
                    match (&args[0], &args[1]) {
                        (Value::Array(arr), Value::String(sep)) => {
                            let parts: Vec<String> = arr.iter().map(|v| v.to_string_value()).collect();
                            Ok(Value::String(parts.join(sep)))
                        }
                        _ => Err(RuntimeError::TypeError("join requires array and string".into())),
                    }
                }),
            }),
        );

        // Math functions
        env.define(
            "Math".to_string(),
            Value::Object({
                let mut math = HashMap::new();
                math.insert(
                    "PI".to_string(),
                    Value::Float(std::f64::consts::PI),
                );
                math.insert(
                    "E".to_string(),
                    Value::Float(std::f64::consts::E),
                );
                math.insert(
                    "abs".to_string(),
                    Value::NativeFunction(crate::value::NativeFunction {
                        name: "abs".to_string(),
                        func: Arc::new(|args| {
                            if args.len() != 1 {
                                return Err(RuntimeError::Error("abs requires 1 argument".into()));
                            }
                            match &args[0] {
                                Value::Integer(n) => Ok(Value::Integer(n.abs())),
                                Value::Float(n) => Ok(Value::Float(n.abs())),
                                _ => Err(RuntimeError::TypeError("abs requires a number".into())),
                            }
                        }),
                    }),
                );
                math.insert(
                    "max".to_string(),
                    Value::NativeFunction(crate::value::NativeFunction {
                        name: "max".to_string(),
                        func: Arc::new(|args| {
                            if args.len() < 2 {
                                return Err(RuntimeError::Error("max requires at least 2 arguments".into()));
                            }
                            let mut max = args[0].clone();
                            for arg in &args[1..] {
                                if arg.to_number()? > max.to_number()? {
                                    max = arg.clone();
                                }
                            }
                            Ok(max)
                        }),
                    }),
                );
                math.insert(
                    "min".to_string(),
                    Value::NativeFunction(crate::value::NativeFunction {
                        name: "min".to_string(),
                        func: Arc::new(|args| {
                            if args.len() < 2 {
                                return Err(RuntimeError::Error("min requires at least 2 arguments".into()));
                            }
                            let mut min = args[0].clone();
                            for arg in &args[1..] {
                                if arg.to_number()? < min.to_number()? {
                                    min = arg.clone();
                                }
                            }
                            Ok(min)
                        }),
                    }),
                );
                math.insert(
                    "sqrt".to_string(),
                    Value::NativeFunction(crate::value::NativeFunction {
                        name: "sqrt".to_string(),
                        func: Arc::new(|args| {
                            if args.len() != 1 {
                                return Err(RuntimeError::Error("sqrt requires 1 argument".into()));
                            }
                            let n = args[0].to_number()?;
                            Ok(Value::Float(n.sqrt()))
                        }),
                    }),
                );
                math.insert(
                    "pow".to_string(),
                    Value::NativeFunction(crate::value::NativeFunction {
                        name: "pow".to_string(),
                        func: Arc::new(|args| {
                            if args.len() != 2 {
                                return Err(RuntimeError::Error("pow requires 2 arguments".into()));
                            }
                            let base = args[0].to_number()?;
                            let exp = args[1].to_number()?;
                            Ok(Value::Float(base.powf(exp)))
                        }),
                    }),
                );
                math
            }),
        );
    }

    pub fn interpret(&mut self, program: &Program) -> Result<Value, RuntimeError> {
        let mut result = Value::Null;
        for stmt in &program.stmts {
            result = self.exec_stmt(stmt)?;
        }
        Ok(result)
    }

    fn exec_stmt(&mut self, stmt: &Stmt) -> Result<Value, RuntimeError> {
        match stmt {
            Stmt::Expr(expr) => self.eval_expr(expr),
            Stmt::VarDecl {
                name,
                type_annotation: _,
                value,
                is_const,
            } => {
                let val = self.eval_expr(value)?;
                self.env.lock().unwrap().define(name.clone(), val);
                Ok(Value::Null)
            }
            Stmt::FuncDecl {
                name,
                params,
                return_type: _,
                body,
                is_async,
            } => {
                let func = Value::Function(Function {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                    is_async: *is_async,
                });
                self.env.lock().unwrap().define(name.clone(), func);
                Ok(Value::Null)
            }
            Stmt::Return(expr) => {
                let value = if let Some(e) = expr {
                    self.eval_expr(e)?
                } else {
                    Value::Null
                };
                Err(RuntimeError::Return(value))
            }
            Stmt::If {
                condition,
                then_body,
                elif_clauses,
                else_body,
            } => {
                if self.eval_expr(condition)?.is_truthy() {
                    self.exec_block(then_body)?;
                } else {
                    let mut executed = false;
                    for (cond, body) in elif_clauses {
                        if self.eval_expr(cond)?.is_truthy() {
                            self.exec_block(body)?;
                            executed = true;
                            break;
                        }
                    }
                    if !executed {
                        if let Some(else_b) = else_body {
                            self.exec_block(else_b)?;
                        }
                    }
                }
                Ok(Value::Null)
            }
            Stmt::While { condition, body } => {
                while self.eval_expr(condition)?.is_truthy() {
                    match self.exec_block(body) {
                        Ok(_) => {}
                        Err(RuntimeError::Break) => break,
                        Err(RuntimeError::Continue) => continue,
                        Err(e) => return Err(e),
                    }
                }
                Ok(Value::Null)
            }
            Stmt::For {
                variable,
                iterable,
                body,
            } => {
                let iter_value = self.eval_expr(iterable)?;
                match iter_value {
                    Value::Array(arr) => {
                        let mut result = Value::Null;
                        for item in arr {
                            self.env.lock().unwrap().define(variable.clone(), item);
                            match self.exec_block(body) {
                                Ok(v) => result = v,
                                Err(RuntimeError::Break) => break,
                                Err(RuntimeError::Continue) => continue,
                                Err(RuntimeError::Return(v)) => return Err(RuntimeError::Return(v)),
                                Err(e) => return Err(e),
                            }
                        }
                        Ok(result)
                    }
                    Value::String(s) => {
                        let mut result = Value::Null;
                        for ch in s.chars() {
                            self.env
                                .lock()
                                .unwrap()
                                .define(variable.clone(), Value::String(ch.to_string()));
                            match self.exec_block(body) {
                                Ok(v) => result = v,
                                Err(RuntimeError::Break) => break,
                                Err(RuntimeError::Continue) => continue,
                                Err(RuntimeError::Return(v)) => return Err(RuntimeError::Return(v)),
                                Err(e) => return Err(e),
                            }
                        }
                        Ok(result)
                    }
                    _ => Err(RuntimeError::TypeError(format!(
                        "Cannot iterate over {}",
                        iter_value.type_name()
                    ))),
                }
            }
            Stmt::Break => Err(RuntimeError::Break),
            Stmt::Continue => Err(RuntimeError::Continue),
            Stmt::Block(block) => {
                self.exec_block(block)?;
                Ok(Value::Null)
            }
            Stmt::Import { module, alias } => {
                // Import handling - for now just define null
                let name = alias.as_deref().unwrap_or(module);
                self.env
                    .lock()
                    .unwrap()
                    .define(name.to_string(), Value::Null);
                Ok(Value::Null)
            }
            Stmt::ImportFrom { module, names } => {
                // Import from handling - for now just define nulls
                for name in names {
                    self.env
                        .lock()
                        .unwrap()
                        .define(name.clone(), Value::Null);
                }
                Ok(Value::Null)
            }
            Stmt::ClassDecl {
                name,
                superclass,
                body,
            } => {
                let mut methods = HashMap::new();
                let mut properties = HashMap::new();
                let mut constructor = None;

                for stmt in &body.methods {
                    if let Stmt::FuncDecl {
                        name: mname,
                        params,
                        body: mbody,
                        is_async,
                        ..
                    } = stmt
                    {
                        let func = Value::Function(Function {
                            name: mname.clone(),
                            params: params.clone(),
                            body: mbody.clone(),
                            is_async: *is_async,
                        });
                        methods.insert(mname.clone(), func);
                    }
                }

                for stmt in &body.properties {
                    if let Stmt::VarDecl { name: pname, value, .. } = stmt {
                        let val = self.eval_expr(value)?;
                        properties.insert(pname.clone(), val);
                    }
                }

                if let Some(stmt) = &body.constructor {
                    if let Stmt::FuncDecl {
                        params, body: cbody, ..
                    } = stmt
                    {
                        constructor = Some(Function {
                            name: "init".to_string(),
                            params: params.clone(),
                            body: cbody.clone(),
                            is_async: false,
                        });
                    }
                }

                let class = Value::Class(Class {
                    name: name.clone(),
                    superclass: superclass.as_ref().map(|_| {
                        // In a real implementation, look up the superclass
                        Box::new(Class {
                            name: String::new(),
                            superclass: None,
                            methods: HashMap::new(),
                            properties: HashMap::new(),
                            constructor: None,
                        })
                    }),
                    methods,
                    properties,
                    constructor,
                });

                self.env.lock().unwrap().define(name.clone(), class);
                Ok(Value::Null)
            }
            Stmt::TryCatch {
                try_body,
                catch_var,
                catch_body,
                finally_body,
            } => {
                match self.exec_block(try_body) {
                    Ok(_) => {}
                    Err(e) => {
                        if let Some(catch_b) = catch_body {
                            if let Some(var_name) = catch_var {
                                let error_msg = Value::String(e.to_string());
                                self.env
                                    .lock()
                                    .unwrap()
                                    .define(var_name.clone(), error_msg);
                            }
                            self.exec_block(catch_b)?;
                        }
                    }
                }
                if let Some(finally_b) = finally_body {
                    self.exec_block(finally_b)?;
                }
                Ok(Value::Null)
            }
            Stmt::Throw(expr) => {
                let value = self.eval_expr(expr)?;
                Err(RuntimeError::Error(value.to_string_value()))
            }
            Stmt::Module { name, body } => {
                self.exec_block(body)?;
                Ok(Value::Null)
            }
            Stmt::Export(stmt) => self.exec_stmt(stmt),
            Stmt::TypeDecl { .. } => Ok(Value::Null),
            Stmt::InterfaceDecl { .. } => Ok(Value::Null),
            Stmt::EnumDecl { .. } => Ok(Value::Null),
        }
    }

    fn exec_block(&mut self, block: &Block) -> Result<Value, RuntimeError> {
        let parent_env = self.env.clone();
        let new_env = Arc::new(Mutex::new(Environment::with_parent(parent_env)));
        let old_env = std::mem::replace(&mut self.env, new_env);

        let mut result = Value::Null;
        for stmt in &block.stmts {
            result = self.exec_stmt(stmt)?;
        }

        self.env = old_env;
        Ok(result)
    }

    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        self.call_depth += 1;
        if self.call_depth > self.max_call_depth {
            return Err(RuntimeError::StackOverflow);
        }

        let result = self.eval_expr_inner(expr);
        self.call_depth -= 1;
        result
    }

    fn eval_expr_inner(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Integer(n) => Ok(Value::Integer(*n)),
            Expr::Float(n) => Ok(Value::Float(*n)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::Boolean(b) => Ok(Value::Boolean(*b)),
            Expr::Null => Ok(Value::Null),
            Expr::Array(elements) => {
                let mut values = Vec::new();
                for elem in elements {
                    values.push(self.eval_expr(elem)?);
                }
                Ok(Value::Array(values))
            }
            Expr::Object(pairs) => {
                let mut map = HashMap::new();
                for (key, value) in pairs {
                    map.insert(key.clone(), self.eval_expr(value)?);
                }
                Ok(Value::Object(map))
            }
            Expr::Identifier(name) => self.env.lock().unwrap().get(name).ok_or_else(|| {
                RuntimeError::UndefinedVariable(name.clone())
            }),
            Expr::Binary { op, left, right } => {
                let left_val = self.eval_expr(left)?;
                let right_val = self.eval_expr(right)?;

                match op {
                    BinaryOp::Add => left_val.add(&right_val),
                    BinaryOp::Subtract => left_val.subtract(&right_val),
                    BinaryOp::Multiply => left_val.multiply(&right_val),
                    BinaryOp::Divide => left_val.divide(&right_val),
                    BinaryOp::Modulo => left_val.modulo(&right_val),
                    BinaryOp::Power => left_val.power(&right_val),
                    BinaryOp::Equal => Ok(Value::Boolean(left_val.equal(&right_val))),
                    BinaryOp::NotEqual => Ok(Value::Boolean(!left_val.equal(&right_val))),
                    BinaryOp::Less => Ok(Value::Boolean(left_val.less_than(&right_val)?)),
                    BinaryOp::Greater => Ok(Value::Boolean(left_val.greater_than(&right_val)?)),
                    BinaryOp::LessEqual => {
                        Ok(Value::Boolean(left_val.less_than(&right_val)? || left_val.equal(&right_val)))
                    }
                    BinaryOp::GreaterEqual => {
                        Ok(Value::Boolean(left_val.greater_than(&right_val)? || left_val.equal(&right_val)))
                    }
                    BinaryOp::And => Ok(Value::Boolean(
                        left_val.is_truthy() && right_val.is_truthy(),
                    )),
                    BinaryOp::Or => Ok(Value::Boolean(
                        left_val.is_truthy() || right_val.is_truthy(),
                    )),
                    BinaryOp::BitwiseAnd => {
                        let a = left_val.to_number()? as i64;
                        let b = right_val.to_number()? as i64;
                        Ok(Value::Integer(a & b))
                    }
                    BinaryOp::BitwiseOr => {
                        let a = left_val.to_number()? as i64;
                        let b = right_val.to_number()? as i64;
                        Ok(Value::Integer(a | b))
                    }
                    BinaryOp::BitwiseXor => {
                        let a = left_val.to_number()? as i64;
                        let b = right_val.to_number()? as i64;
                        Ok(Value::Integer(a ^ b))
                    }
                    BinaryOp::ShiftLeft => {
                        let a = left_val.to_number()? as i64;
                        let b = right_val.to_number()? as u32;
                        Ok(Value::Integer(a << b))
                    }
                    BinaryOp::ShiftRight => {
                        let a = left_val.to_number()? as i64;
                        let b = right_val.to_number()? as u32;
                        Ok(Value::Integer(a >> b))
                    }
                }
            }
            Expr::Unary { op, operand } => {
                let val = self.eval_expr(operand)?;
                match op {
                    UnaryOp::Negate => val.negate(),
                    UnaryOp::Not => Ok(Value::Boolean(!val.is_truthy())),
                    UnaryOp::Increment => {
                        let new_val = Value::Integer(val.to_number()? as i64 + 1);
                        Ok(new_val)
                    }
                    UnaryOp::Decrement => {
                        let new_val = Value::Integer(val.to_number()? as i64 - 1);
                        Ok(new_val)
                    }
                }
            }
            Expr::Call { callee, args } => {
                let func = self.eval_expr(callee)?;
                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.eval_expr(arg)?);
                }
                self.call_function(&func, &arg_values)
            }
            Expr::MethodCall {
                object,
                method,
                args,
            } => {
                let obj = self.eval_expr(object)?;
                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.eval_expr(arg)?);
                }
                self.call_method(&obj, method, &arg_values)
            }
            Expr::PropertyAccess { object, property } => {
                let obj = self.eval_expr(object)?;
                match &obj {
                    Value::Object(map) => map
                        .get(property)
                        .cloned()
                        .ok_or_else(|| RuntimeError::UndefinedProperty(property.clone())),
                    Value::Instance(inst) => inst
                        .properties
                        .get(property)
                        .cloned()
                        .or_else(|| inst.class.methods.get(property).cloned())
                        .ok_or_else(|| RuntimeError::UndefinedProperty(property.clone())),
                    Value::Array(arr) => {
                        if property == "length" {
                            Ok(Value::Integer(arr.len() as i64))
                        } else {
                            Err(RuntimeError::UndefinedProperty(property.clone()))
                        }
                    }
                    Value::String(s) => {
                        if property == "length" {
                            Ok(Value::Integer(s.len() as i64))
                        } else {
                            Err(RuntimeError::UndefinedProperty(property.clone()))
                        }
                    }
                    _ => Err(RuntimeError::TypeError(format!(
                        "Cannot access property on {}",
                        obj.type_name()
                    ))),
                }
            }
            Expr::IndexAccess { object, index } => {
                let obj = self.eval_expr(object)?;
                let idx = self.eval_expr(index)?;

                match (&obj, &idx) {
                    (Value::Array(arr), Value::Integer(i)) => {
                        let idx = *i as usize;
                        if idx < arr.len() {
                            Ok(arr[idx].clone())
                        } else {
                            Err(RuntimeError::IndexOutOfBounds(format!(
                                "Index {} out of bounds for array of length {}",
                                idx,
                                arr.len()
                            )))
                        }
                    }
                    (Value::Object(map), Value::String(key)) => map
                        .get(key)
                        .cloned()
                        .ok_or_else(|| RuntimeError::UndefinedProperty(key.clone())),
                    (Value::String(s), Value::Integer(i)) => {
                        let idx = *i as usize;
                        if let Some(ch) = s.chars().nth(idx) {
                            Ok(Value::String(ch.to_string()))
                        } else {
                            Err(RuntimeError::IndexOutOfBounds(format!(
                                "Index {} out of bounds for string of length {}",
                                idx,
                                s.len()
                            )))
                        }
                    }
                    _ => Err(RuntimeError::TypeError(format!(
                        "Cannot index into {} with {}",
                        obj.type_name(),
                        idx.type_name()
                    ))),
                }
            }
            Expr::Lambda { params, body } => {
                let env = self.env.lock().unwrap().clone();
                Ok(Value::Closure(Closure {
                    params: params
                        .iter()
                        .map(|p| Param {
                            name: p.clone(),
                            type_annotation: None,
                            default: None,
                        })
                        .collect(),
                    body: Block {
                        stmts: vec![Stmt::Expr(*body.clone())],
                    },
                    env,
                }))
            }
            Expr::Ternary {
                condition,
                then_expr,
                else_expr,
            } => {
                if self.eval_expr(condition)?.is_truthy() {
                    self.eval_expr(then_expr)
                } else {
                    self.eval_expr(else_expr)
                }
            }
            Expr::Assign { target, value } => {
                let val = self.eval_expr(value)?;
                if let Expr::Identifier(name) = target.as_ref() {
                    if self.env.lock().unwrap().set(name, val.clone()) {
                        Ok(val)
                    } else {
                        Err(RuntimeError::UndefinedVariable(name.clone()))
                    }
                } else {
                    Err(RuntimeError::Error("Invalid assignment target".into()))
                }
            }
            Expr::CompoundAssign { op, target, value } => {
                let current = self.eval_expr(target)?;
                let right = self.eval_expr(value)?;
                let new_val = match op {
                    BinaryOp::Add => current.add(&right)?,
                    BinaryOp::Subtract => current.subtract(&right)?,
                    BinaryOp::Multiply => current.multiply(&right)?,
                    BinaryOp::Divide => current.divide(&right)?,
                    BinaryOp::Modulo => current.modulo(&right)?,
                    BinaryOp::Power => current.power(&right)?,
                    _ => return Err(RuntimeError::Error("Invalid compound assignment operator".into())),
                };
                if let Expr::Identifier(name) = target.as_ref() {
                    self.env.lock().unwrap().set(name, new_val.clone());
                    Ok(new_val)
                } else {
                    Err(RuntimeError::Error("Invalid assignment target".into()))
                }
            }
            Expr::Await { expr } => {
                // For now, just evaluate the expression
                self.eval_expr(expr)
            }
            Expr::New { class, args } => {
                let class_val = self.eval_expr(class)?;
                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.eval_expr(arg)?);
                }

                match &class_val {
                    Value::Class(class) => {
                        let mut properties = HashMap::new();
                        for (key, value) in &class.properties {
                            properties.insert(key.clone(), value.clone());
                        }

                        let mut instance = Value::Instance(Instance {
                            class: class.clone(),
                            properties,
                        });

                        if let Some(constructor) = &class.constructor {
                            let constructor_val = Value::Function(constructor.clone());
                            self.call_function(&constructor_val, &arg_values)?;
                        }

                        Ok(instance)
                    }
                    _ => Err(RuntimeError::TypeError(format!(
                        "Cannot instantiate {}",
                        class_val.type_name()
                    ))),
                }
            }
            Expr::This => {
                // Look up 'this' in environment
                self.env
                    .lock()
                    .unwrap()
                    .get("this")
                    .ok_or_else(|| RuntimeError::Error("'this' used outside of class".into()))
            }
            Expr::Self_ => {
                self.env
                    .lock()
                    .unwrap()
                    .get("self")
                    .ok_or_else(|| RuntimeError::Error("'self' used outside of method".into()))
            }
            Expr::Match { expr, arms } => {
                let val = self.eval_expr(expr)?;
                for arm in arms {
                    if self.match_pattern(&arm.pattern, &val)? {
                        if let Some(guard) = &arm.guard {
                            if !self.eval_expr(guard)?.is_truthy() {
                                continue;
                            }
                        }
                        return self.eval_expr(&arm.body);
                    }
                }
                Err(RuntimeError::Error("No matching pattern in match expression".into()))
            }
            Expr::AiGenerate { prompt } => {
                // Placeholder for AI generation
                Ok(Value::String(format!("[AI Generated: {}]", prompt)))
            }
        }
    }

    fn match_pattern(&mut self, pattern: &Pattern, value: &Value) -> Result<bool, RuntimeError> {
        match pattern {
            Pattern::Literal(lit) => {
                let lit_val = self.eval_expr(lit)?;
                Ok(value.equal(&lit_val))
            }
            Pattern::Identifier(name) => {
                self.env.lock().unwrap().define(name.clone(), value.clone());
                Ok(true)
            }
            Pattern::Wildcard => Ok(true),
            Pattern::Array(patterns) => {
                if let Value::Array(arr) = value {
                    if arr.len() == patterns.len() {
                        for (p, v) in patterns.iter().zip(arr.iter()) {
                            if !self.match_pattern(p, v)? {
                                return Ok(false);
                            }
                        }
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                } else {
                    Ok(false)
                }
            }
            Pattern::Object(pairs) => {
                if let Value::Object(map) = value {
                    for (key, pat) in pairs {
                        if let Some(v) = map.get(key) {
                            if !self.match_pattern(pat, v)? {
                                return Ok(false);
                            }
                        } else {
                            return Ok(false);
                        }
                    }
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    }

    fn call_function(
        &mut self,
        func: &Value,
        args: &[Value],
    ) -> Result<Value, RuntimeError> {
        match func {
            Value::Function(f) => {
                self.call_depth += 1;
                if self.call_depth > self.max_call_depth {
                    return Err(RuntimeError::StackOverflow);
                }

                let parent_env = self.env.clone();
                let mut func_env = Environment::with_parent(parent_env);

                for (param, arg) in f.params.iter().zip(args.iter()) {
                    func_env.define(param.name.clone(), arg.clone());
                }

                let old_env = std::mem::replace(
                    &mut self.env,
                    Arc::new(Mutex::new(func_env)),
                );

                let result = self.exec_block(&f.body);
                self.env = old_env;
                self.call_depth -= 1;

                match result {
                    Ok(val) => Ok(val),
                    Err(RuntimeError::Return(val)) => Ok(val),
                    Err(e) => Err(e),
                }
            }
            Value::NativeFunction(f) => (f.func)(args),
            Value::Closure(c) => {
                self.call_depth += 1;
                if self.call_depth > self.max_call_depth {
                    return Err(RuntimeError::StackOverflow);
                }

                let parent_env = Arc::new(Mutex::new(c.env.clone()));
                let mut func_env = Environment::with_parent(parent_env);

                for (param, arg) in c.params.iter().zip(args.iter()) {
                    func_env.define(param.name.clone(), arg.clone());
                }

                let old_env = std::mem::replace(
                    &mut self.env,
                    Arc::new(Mutex::new(func_env)),
                );

                let result = self.exec_block(&c.body);
                self.env = old_env;
                self.call_depth -= 1;

                match result {
                    Ok(val) => Ok(val),
                    Err(RuntimeError::Return(val)) => Ok(val),
                    Err(e) => Err(e),
                }
            }
            _ => Err(RuntimeError::TypeError(format!(
                "Cannot call {}",
                func.type_name()
            ))),
        }
    }

    fn call_method(
        &mut self,
        obj: &Value,
        method: &str,
        args: &[Value],
    ) -> Result<Value, RuntimeError> {
        match obj {
            Value::Instance(inst) => {
                if let Some(func) = inst.class.methods.get(method) {
                    let mut new_args = vec![obj.clone()];
                    new_args.extend_from_slice(args);
                    self.call_function(func, &new_args)
                } else {
                    Err(RuntimeError::UndefinedProperty(method.to_string()))
                }
            }
            Value::Array(arr) => {
                match method {
                    "push" => {
                        let mut new_arr = arr.clone();
                        new_arr.extend_from_slice(args);
                        Ok(Value::Array(new_arr))
                    }
                    "pop" => {
                        let mut new_arr = arr.clone();
                        Ok(new_arr.pop().unwrap_or(Value::Null))
                    }
                    "map" => {
                        if args.len() != 1 {
                            return Err(RuntimeError::Error("map requires 1 argument".into()));
                        }
                        let mut result = Vec::new();
                        for item in arr {
                            let mapped = self.call_function(&args[0], &[item.clone()])?;
                            result.push(mapped);
                        }
                        Ok(Value::Array(result))
                    }
                    "filter" => {
                        if args.len() != 1 {
                            return Err(RuntimeError::Error("filter requires 1 argument".into()));
                        }
                        let mut result = Vec::new();
                        for item in arr {
                            if self.call_function(&args[0], &[item.clone()])?.is_truthy() {
                                result.push(item.clone());
                            }
                        }
                        Ok(Value::Array(result))
                    }
                    "reduce" => {
                        if args.len() != 2 {
                            return Err(RuntimeError::Error("reduce requires 2 arguments".into()));
                        }
                        let mut acc = args[0].clone();
                        for item in arr {
                            acc = self.call_function(&args[1], &[acc, item.clone()])?;
                        }
                        Ok(acc)
                    }
                    "find" => {
                        if args.len() != 1 {
                            return Err(RuntimeError::Error("find requires 1 argument".into()));
                        }
                        for item in arr {
                            if self.call_function(&args[0], &[item.clone()])?.is_truthy() {
                                return Ok(item.clone());
                            }
                        }
                        Ok(Value::Null)
                    }
                    "indexOf" => {
                        if args.len() != 1 {
                            return Err(RuntimeError::Error("indexOf requires 1 argument".into()));
                        }
                        for (i, item) in arr.iter().enumerate() {
                            if item.equal(&args[0]) {
                                return Ok(Value::Integer(i as i64));
                            }
                        }
                        Ok(Value::Integer(-1))
                    }
                    "slice" => {
                        let start = if args.len() > 0 {
                            args[0].to_number()? as usize
                        } else {
                            0
                        };
                        let end = if args.len() > 1 {
                            args[1].to_number()? as usize
                        } else {
                            arr.len()
                        };
                        if start <= end && end <= arr.len() {
                            Ok(Value::Array(arr[start..end].to_vec()))
                        } else {
                            Err(RuntimeError::Error("Invalid slice range".into()))
                        }
                    }
                    _ => Err(RuntimeError::UndefinedProperty(method.to_string())),
                }
            }
            Value::String(s) => {
                match method {
                    "length" => Ok(Value::Integer(s.len() as i64)),
                    "charAt" => {
                        if args.len() != 1 {
                            return Err(RuntimeError::Error("charAt requires 1 argument".into()));
                        }
                        let idx = args[0].to_number()? as usize;
                        if let Some(ch) = s.chars().nth(idx) {
                            Ok(Value::String(ch.to_string()))
                        } else {
                            Ok(Value::Null)
                        }
                    }
                    "substring" => {
                        let start = if args.len() > 0 {
                            args[0].to_number()? as usize
                        } else {
                            0
                        };
                        let end = if args.len() > 1 {
                            args[1].to_number()? as usize
                        } else {
                            s.len()
                        };
                        if start <= end && end <= s.len() {
                            Ok(Value::String(s[start..end].to_string()))
                        } else {
                            Err(RuntimeError::Error("Invalid substring range".into()))
                        }
                    }
                    "toLowerCase" => Ok(Value::String(s.to_lowercase())),
                    "toUpperCase" => Ok(Value::String(s.to_uppercase())),
                    "trim" => Ok(Value::String(s.trim().to_string())),
                    "includes" => {
                        if args.len() != 1 {
                            return Err(RuntimeError::Error("includes requires 1 argument".into()));
                        }
                        let search = args[0].to_string_value();
                        Ok(Value::Boolean(s.contains(&search)))
                    }
                    "startsWith" => {
                        if args.len() != 1 {
                            return Err(RuntimeError::Error("startsWith requires 1 argument".into()));
                        }
                        let prefix = args[0].to_string_value();
                        Ok(Value::Boolean(s.starts_with(&prefix)))
                    }
                    "endsWith" => {
                        if args.len() != 1 {
                            return Err(RuntimeError::Error("endsWith requires 1 argument".into()));
                        }
                        let suffix = args[0].to_string_value();
                        Ok(Value::Boolean(s.ends_with(&suffix)))
                    }
                    "indexOf" => {
                        if args.len() != 1 {
                            return Err(RuntimeError::Error("indexOf requires 1 argument".into()));
                        }
                        let search = args[0].to_string_value();
                        match s.find(&search) {
                            Some(idx) => Ok(Value::Integer(idx as i64)),
                            None => Ok(Value::Integer(-1)),
                        }
                    }
                    _ => Err(RuntimeError::UndefinedProperty(method.to_string())),
                }
            }
            Value::Object(map) => {
                if let Some(value) = map.get(method) {
                    if let Value::Function(_) = value {
                        let mut new_args = vec![obj.clone()];
                        new_args.extend_from_slice(args);
                        self.call_function(value, &new_args)
                    } else {
                        Ok(value.clone())
                    }
                } else {
                    Err(RuntimeError::UndefinedProperty(method.to_string()))
                }
            }
            _ => Err(RuntimeError::TypeError(format!(
                "Cannot call method on {}",
                obj.type_name()
            ))),
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}
