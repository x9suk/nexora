use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::ast::*;
use crate::error::NexoraError;
use crate::value::Value;

pub struct Interpreter {
    globals: HashMap<String, Value>,
    locals: Vec<HashMap<String, Value>>,
    base_path: PathBuf,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = HashMap::new();

        Interpreter {
            globals,
            locals: Vec::new(),
            base_path: PathBuf::new(),
        }
    }

    pub fn set_base_path(&mut self, path: &str) {
        self.base_path = PathBuf::from(path).parent().unwrap_or(std::path::Path::new(".")).to_path_buf();
    }

    pub fn get_globals(&self) -> &HashMap<String, Value> {
        &self.globals
    }

    pub fn run(&mut self, stmts: &[Stmt]) -> Result<(), NexoraError> {
        for stmt in stmts {
            self.exec_stmt(stmt)?;
        }
        Ok(())
    }

    fn get_var(&self, name: &str) -> Result<Value, NexoraError> {
        for scope in self.locals.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Ok(val.clone());
            }
        }
        self.globals
            .get(name)
            .cloned()
            .ok_or_else(|| NexoraError::UndefinedVariable(name.to_string()))
    }

    fn set_var(&mut self, name: &str, value: Value) {
        if let Some(scope) = self.locals.last_mut() {
            scope.insert(name.to_string(), value);
        } else {
            self.globals.insert(name.to_string(), value);
        }
    }

    fn push_scope(&mut self) {
        self.locals.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.locals.pop();
    }

    fn exec_stmt(&mut self, stmt: &Stmt) -> Result<Option<Value>, NexoraError> {
        match stmt {
            Stmt::Expr(expr) => {
                self.eval_expr(expr)?;
                Ok(None)
            }
            Stmt::Var { name, value } => {
                let val = self.eval_expr(value)?;
                self.set_var(name, val);
                Ok(None)
            }
            Stmt::Assign { name, value } => {
                let val = self.eval_expr(value)?;
                if self.locals.iter().rev().any(|scope| scope.contains_key(name)) {
                    for scope in self.locals.iter_mut().rev() {
                        if scope.contains_key(name) {
                            scope.insert(name.to_string(), val);
                            break;
                        }
                    }
                } else if self.globals.contains_key(name) {
                    self.globals.insert(name.to_string(), val);
                } else {
                    return Err(NexoraError::UndefinedVariable(name.clone()));
                }
                Ok(None)
            }
            Stmt::PropertyAssign { object, prop, value } => {
                let val = self.eval_expr(value)?;
                let obj = self.eval_expr(object)?;
                match obj {
                    Value::ObjectInstance { class_name, mut fields } => {
                        fields.insert(prop.clone(), val);
                        let instance = Value::ObjectInstance { class_name, fields };
                        match object {
                            Expr::Ident(name) => { self.set_var(name, instance); }
                            Expr::This => { self.set_var("this", instance); }
                            _ => {}
                        }
                        Ok(None)
                    }
                    Value::Object(mut map) => {
                        map.insert(prop.clone(), val);
                        match object {
                            Expr::Ident(name) => { self.set_var(name, Value::Object(map)); }
                            Expr::This => { self.set_var("this", Value::Object(map)); }
                            _ => {}
                        }
                        Ok(None)
                    }
                    _ => Err(NexoraError::TypeError(format!(
                        "Cannot set property '{}' on {}",
                        prop,
                        obj.type_name()
                    ))),
                }
            }
            Stmt::Func { name, params, body } => {
                let func = Value::Func {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                };
                self.set_var(name, func);
                Ok(None)
            }
            Stmt::AsyncFunc { name, params, body } => {
                // For now, async functions run synchronously (no real async support yet)
                let func = Value::Func {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                };
                self.set_var(name, func);
                Ok(None)
            }
            Stmt::Return(expr) => {
                let val = if let Some(e) = expr {
                    self.eval_expr(e)?
                } else {
                    Value::Null
                };
                Err(NexoraError::ReturnValue(val))
            }
            Stmt::If {
                condition,
                then_body,
                else_body,
            } => {
                let cond = self.eval_expr(condition)?;
                if cond.is_truthy() {
                    self.exec_block(then_body)?;
                } else if let Some(else_b) = else_body {
                    self.exec_block(else_b)?;
                }
                Ok(None)
            }
            Stmt::While { condition, body } => {
                while self.eval_expr(condition)?.is_truthy() {
                    match self.exec_block(body) {
                        Ok(_) => {}
                        Err(NexoraError::BreakSignal) => break,
                        Err(NexoraError::ContinueSignal) => continue,
                        Err(e) => return Err(e),
                    }
                }
                Ok(None)
            }
            Stmt::For {
                var,
                iterable,
                body,
            } => {
                let iter_val = self.eval_expr(iterable)?;
                match iter_val {
                    Value::Array(arr) => {
                        for item in arr {
                            self.push_scope();
                            self.set_var(var, item);
                            match self.exec_block(body) {
                                Ok(_) => {}
                                Err(NexoraError::BreakSignal) => {
                                    self.pop_scope();
                                    break;
                                }
                                Err(NexoraError::ContinueSignal) => {
                                    self.pop_scope();
                                    continue;
                                }
                                Err(e) => {
                                    self.pop_scope();
                                    return Err(e);
                                }
                            }
                            self.pop_scope();
                        }
                    }
                    Value::String(s) => {
                        for ch in s.chars() {
                            self.push_scope();
                            self.set_var(var, Value::String(ch.to_string()));
                            match self.exec_block(body) {
                                Ok(_) => {}
                                Err(NexoraError::BreakSignal) => {
                                    self.pop_scope();
                                    break;
                                }
                                Err(NexoraError::ContinueSignal) => {
                                    self.pop_scope();
                                    continue;
                                }
                                Err(e) => {
                                    self.pop_scope();
                                    return Err(e);
                                }
                            }
                            self.pop_scope();
                        }
                    }
                    _ => {
                        return Err(NexoraError::TypeError(format!(
                            "Cannot iterate over {}",
                            iter_val.type_name()
                        )));
                    }
                }
                Ok(None)
            }
            Stmt::Break => Err(NexoraError::BreakSignal),
            Stmt::Continue => Err(NexoraError::ContinueSignal),
            Stmt::Print(args) => {
                let mut output = Vec::new();
                for arg in args {
                    let val = self.eval_expr(arg)?;
                    output.push(val.to_string());
                }
                println!("{}", output.join(" "));
                Ok(None)
            }
            Stmt::Import { path, names, alias } => {
                // Try to find the file: first in base_path, then in lib directory
                let full_path = if self.base_path.exists() && self.base_path.join(path).exists() {
                    self.base_path.join(path)
                } else {
                    // Check lib directory
                    let lib_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("lib").join(path);
                    if lib_path.exists() {
                        lib_path
                    } else if std::path::PathBuf::from(path).exists() {
                        std::path::PathBuf::from(path)
                    } else {
                        return Err(NexoraError::TypeError(format!("Cannot import '{}': file not found", path)));
                    }
                };
                let content = fs::read_to_string(&full_path)
                    .map_err(|e| NexoraError::TypeError(format!("Cannot import '{}': {}", path, e)))?;
                let mut lexer = crate::lexer::Lexer::new(&content);
                let tokens = lexer.tokenize();
                let mut parser = crate::parser::Parser::new(tokens);
                let stmts = parser.parse();
                let old_base = self.base_path.clone();
                self.set_base_path(&full_path.to_string_lossy());
                
                // Save current globals to detect new definitions
                let old_globals: HashMap<String, Value> = self.globals.clone();
                self.run(&stmts)?;
                
                // Handle named imports: import { sqrt, pow } from "math"
                if let Some(names) = names {
                    for name in names {
                        if let Some(val) = self.globals.get(name) {
                            self.set_var(name, val.clone());
                        } else {
                            return Err(NexoraError::TypeError(format!("'{}' not found in '{}'", name, path)));
                        }
                    }
                } else if let Some(alias_name) = alias {
                    // Handle namespace import: import math from "math"
                    let mut module_obj = HashMap::new();
                    for (name, val) in self.globals.iter() {
                        if !old_globals.contains_key(name) {
                            module_obj.insert(name.clone(), val.clone());
                        }
                    }
                    self.set_var(alias_name, Value::Object(module_obj));
                } else {
                    // import "math" - import everything into global scope
                    for (name, val) in self.globals.clone().iter() {
                        if !old_globals.contains_key(name) {
                            self.set_var(name, val.clone());
                        }
                    }
                }
                
                self.base_path = old_base;
                Ok(None)
            }
            Stmt::Class { name, parent, methods } => {
                let class_val = Value::ClassDef {
                    name: name.clone(),
                    parent: parent.clone(),
                    methods: methods.clone(),
                };
                self.set_var(name, class_val);
                Ok(None)
            }
            Stmt::Try {
                body,
                catch_var,
                catch_body,
                finally_body,
            } => {
                let result = self.exec_block(body);
                match result {
                    Ok(_) => {
                        if let Some(fb) = finally_body {
                            self.exec_block(fb)?;
                        }
                        Ok(None)
                    }
                    Err(NexoraError::ExceptionSignal(exc_val)) => {
                        if let (Some(cv), Some(cb)) = (catch_var, catch_body) {
                            self.push_scope();
                            self.set_var(cv, exc_val);
                            let catch_result = self.exec_block(cb);
                            self.pop_scope();
                            match catch_result {
                                Ok(_) => {}
                                Err(e) => {
                                    if let Some(fb) = finally_body {
                                        self.exec_block(fb)?;
                                    }
                                    return Err(e);
                                }
                            }
                        }
                        if let Some(fb) = finally_body {
                            self.exec_block(fb)?;
                        }
                        Ok(None)
                    }
                    Err(e) => {
                        if let (Some(cv), Some(cb)) = (catch_var.as_ref(), catch_body.as_ref()) {
                            self.push_scope();
                            self.set_var(cv, Value::String(e.to_string()));
                            let catch_result = self.exec_block(cb);
                            self.pop_scope();
                            match catch_result {
                                Ok(_) => {}
                                Err(e2) => {
                                    if let Some(fb) = finally_body {
                                        self.exec_block(fb)?;
                                    }
                                    return Err(e2);
                                }
                            }
                        }
                        if let Some(fb) = finally_body {
                            self.exec_block(fb)?;
                        }
                        Ok(None)
                    }
                }
            }
            Stmt::Throw(expr) => {
                let val = self.eval_expr(expr)?;
                Err(NexoraError::ExceptionSignal(val))
            }
            Stmt::Assert { condition, message } => {
                let cond_val = self.eval_expr(condition)?;
                if !cond_val.is_truthy() {
                    let msg = if let Some(m) = message {
                        let m_val = self.eval_expr(m)?;
                        format!("Assert failed: {}", m_val)
                    } else {
                        format!("Assert failed: {}", cond_val)
                    };
                    return Err(NexoraError::TypeError(msg));
                }
                Ok(None)
            }
            Stmt::Test { name, body } => {
                print!("test {} ... ", name);
                match self.exec_block(body) {
                    Ok(_) => {
                        println!("ok");
                        Ok(None)
                    }
                    Err(e) => {
                        println!("FAILED: {}", e);
                        Ok(None)
                    }
                }
            }
            // Type system nodes - ignored at runtime (type-erased generics)
            Stmt::TypeAlias { name, .. } => {
                self.set_var(name, Value::Null);
                Ok(None)
            }
            Stmt::Interface { name, .. } => {
                self.set_var(name, Value::Null);
                Ok(None)
            }
            Stmt::GenericFunc { name, params, body, .. } => {
                let func = Value::Func {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                };
                self.set_var(name, func);
                Ok(None)
            }
            Stmt::GenericClass { name, parent, methods, .. } => {
                let class_val = Value::ClassDef {
                    name: name.clone(),
                    parent: parent.clone(),
                    methods: methods.clone(),
                };
                self.set_var(name, class_val);
                Ok(None)
            }
        }
    }

    fn exec_block(&mut self, stmts: &[Stmt]) -> Result<Option<Value>, NexoraError> {
        self.push_scope();
        let mut result = None;
        for stmt in stmts {
            match self.exec_stmt(stmt) {
                Ok(val) => { result = val; }
                Err(e) => {
                    self.pop_scope();
                    return Err(e);
                }
            }
        }
        self.pop_scope();
        Ok(result)
    }

    fn eval_expr(&mut self, expr: &Expr) -> Result<Value, NexoraError> {
        match expr {
            Expr::Integer(n) => Ok(Value::Integer(*n)),
            Expr::Float(n) => Ok(Value::Float(*n)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::StringInterp(parts) => {
                let mut result = String::new();
                for part in parts {
                    match part {
                        InterpPart::Text(t) => result.push_str(t),
                        InterpPart::Expr(e) => {
                            let val = self.eval_expr(e)?;
                            result.push_str(&val.to_string());
                        }
                    }
                }
                Ok(Value::String(result))
            }
            Expr::Bool(b) => Ok(Value::Bool(*b)),
            Expr::Null => Ok(Value::Null),
            Expr::AnonFunc { params, body } => {
                Ok(Value::Func {
                    name: "<anonymous>".to_string(),
                    params: params.clone(),
                    body: body.clone(),
                })
            }
            Expr::Lambda { params, body } => {
                let mut captures = HashMap::new();
                // Capture outer scope variables referenced in the body
                for (name, val) in self.globals.iter() {
                    captures.insert(name.clone(), val.clone());
                }
                for scope in self.locals.iter() {
                    for (name, val) in scope.iter() {
                        captures.insert(name.clone(), val.clone());
                    }
                }
                Ok(Value::Closure {
                    name: "<lambda>".to_string(),
                    params: params.clone(),
                    body: body.clone(),
                    captures,
                })
            }
            Expr::Match { value, arms } => {
                let val = self.eval_expr(value)?;
                for arm in arms {
                    // Check for wildcard _
                    if let Expr::Ident(ref ident) = arm.pattern {
                        if ident == "_" {
                            return self.eval_expr(&arm.body);
                        }
                    }
                    let pattern_val = self.eval_expr(&arm.pattern)?;
                    if val == pattern_val {
                        return self.eval_expr(&arm.body);
                    }
                }
                Err(NexoraError::TypeError(format!(
                    "No matching arm for {}",
                    val
                )))
            }
            Expr::Generic { base, .. } => self.eval_expr(base),
            Expr::TypeAnnotation { .. } => Ok(Value::Null),
            Expr::Super(method) => {
                let this_val = self.get_var("this")?;
                if let Value::ObjectInstance { class_name, .. } = &this_val {
                    if let Ok(Value::ClassDef { parent: Some(parent_name), .. }) = self.get_var(class_name) {
                        if let Ok(Value::ClassDef { methods, .. }) = self.get_var(&parent_name) {
                            for m in methods {
                                if let Stmt::Func { name: mname, params, body } = m {
                                    if mname == *method {
                                        return Ok(Value::Func {
                                            name: mname.clone(),
                                            params: params.clone(),
                                            body: body.clone(),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
                Err(NexoraError::UndefinedVariable(format!(
                    "Super method '{}' not found",
                    method
                )))
            }
            Expr::This => {
                self.get_var("this")
            }
            Expr::New { class, args } => {
                let class_val = self.eval_expr(class)?;
                match class_val {
                    Value::ClassDef { name, methods, parent, .. } => {
                        let mut fields = HashMap::new();
                        fields.insert("class".to_string(), Value::String(name.clone()));
                        for arg in args {
                            let val = self.eval_expr(arg)?;
                            let idx = args.iter().position(|a| std::mem::discriminant(a) == std::mem::discriminant(arg)).unwrap_or(0);
                            fields.insert(format!("arg{}", idx), val);
                        }
                        let instance = Value::ObjectInstance {
                            class_name: name.clone(),
                            fields,
                        };
                        // Collect all methods including parent's
                        let mut all_methods = methods.clone();
                        if let Some(ref parent_name) = parent {
                            if let Ok(Value::ClassDef { methods: parent_methods, .. }) = self.get_var(parent_name) {
                                for m in parent_methods {
                                    if !all_methods.iter().any(|existing| {
                                        if let Stmt::Func { name: en, .. } = existing {
                                            if let Stmt::Func { name: mn, .. } = &m { en == mn } else { false }
                                        } else { false }
                                    }) {
                                        all_methods.push(m);
                                    }
                                }
                            }
                        }
                        if let Some(init_method) = all_methods.iter().find(|m| {
                            if let Stmt::Func { name, .. } = m {
                                name == "init"
                            } else {
                                false
                            }
                        }) {
                            if let Stmt::Func { params, body, .. } = init_method {
                                let mut new_instance = match instance {
                                    Value::ObjectInstance { class_name, fields } => Value::ObjectInstance { class_name, fields },
                                    _ => unreachable!(),
                                };
                                let mut arg_vals = Vec::new();
                                for arg in args {
                                    arg_vals.push(self.eval_expr(arg)?);
                                }
                                self.push_scope();
                                if let Value::ObjectInstance { .. } = &new_instance {
                                    self.set_var("this", new_instance.clone());
                                }
                                for (param, val) in params.iter().zip(arg_vals) {
                                    self.set_var(param, val);
                                }
                                for stmt in body {
                                    match self.exec_stmt(stmt) {
                                        Ok(_) => {}
                                        Err(NexoraError::ReturnValue(_)) => break,
                                        Err(e) => {
                                            self.pop_scope();
                                            return Err(e);
                                        }
                                    }
                                }
                                new_instance = self.get_var("this")?;
                                self.pop_scope();
                                return Ok(new_instance);
                            }
                        }
                        Ok(instance)
                    }
                    _ => Err(NexoraError::TypeError(format!("{} is not a class", class_val.type_name()))),
                }
            }
            Expr::Ident(name) => self.get_var(name),
            Expr::Array(elements) => {
                let mut vals = Vec::new();
                for elem in elements {
                    vals.push(self.eval_expr(elem)?);
                }
                Ok(Value::Array(vals))
            }
            Expr::Object(pairs) => {
                let mut map = HashMap::new();
                for (key, val) in pairs {
                    map.insert(key.clone(), self.eval_expr(val)?);
                }
                Ok(Value::Object(map))
            }
            Expr::BinaryOp { op, left, right } => {
                let l = self.eval_expr(left)?;
                let r = self.eval_expr(right)?;
                self.eval_binop(op, &l, &r)
            }
            Expr::UnaryOp { op, expr } => {
                let val = self.eval_expr(expr)?;
                match op {
                    UnaryOp::Neg => match val {
                        Value::Integer(n) => Ok(Value::Integer(-n)),
                        Value::Float(n) => Ok(Value::Float(-n)),
                        _ => Err(NexoraError::TypeError(format!(
                            "Cannot negate {}",
                            val.type_name()
                        ))),
                    },
                    UnaryOp::Not => Ok(Value::Bool(!val.is_truthy())),
                }
            }
            Expr::Call { name, args } => {
                if let Expr::Ident(n) = name.as_ref() {
                    match n.as_str() {
                        "print" => {
                            let mut output = Vec::new();
                            for arg in args {
                                let val = self.eval_expr(arg)?;
                                output.push(val.to_string());
                            }
                            println!("{}", output.join(" "));
                            return Ok(Value::Null);
                        }
                        "input" => {
                            if !args.is_empty() {
                                let prompt = self.eval_expr(&args[0])?;
                                print!("{}", prompt);
                            } else {
                                print!("> ");
                            }
                            use std::io::{self, Write};
                            io::stdout().flush().unwrap();
                            let mut line = String::new();
                            io::stdin().read_line(&mut line).unwrap();
                            return Ok(Value::String(line.trim().to_string()));
                        }
                        "type_of" => {
                            if args.len() != 1 {
                                return Err(NexoraError::WrongArity { expected: 1, found: args.len() });
                            }
                            let val = self.eval_expr(&args[0])?;
                            return Ok(Value::String(val.type_name().to_string()));
                        }
                        "str" => {
                            if args.len() != 1 {
                                return Err(NexoraError::WrongArity { expected: 1, found: args.len() });
                            }
                            let val = self.eval_expr(&args[0])?;
                            return Ok(Value::String(val.to_string()));
                        }
                        "int" => {
                            if args.len() != 1 {
                                return Err(NexoraError::WrongArity { expected: 1, found: args.len() });
                            }
                            let val = self.eval_expr(&args[0])?;
                            match val {
                                Value::String(s) => {
                                    let n: i64 = s.parse().map_err(|_| NexoraError::TypeError(format!("Cannot convert '{}' to integer", s)))?;
                                    return Ok(Value::Integer(n));
                                }
                                Value::Float(f) => return Ok(Value::Integer(f as i64)),
                                Value::Integer(n) => return Ok(Value::Integer(n)),
                                Value::Bool(b) => return Ok(Value::Integer(if b { 1 } else { 0 })),
                                _ => return Err(NexoraError::TypeError(format!("Cannot convert {} to integer", val.type_name()))),
                            }
                        }
                        "float" => {
                            if args.len() != 1 {
                                return Err(NexoraError::WrongArity { expected: 1, found: args.len() });
                            }
                            let val = self.eval_expr(&args[0])?;
                            match val {
                                Value::String(s) => {
                                    let n: f64 = s.parse().map_err(|_| NexoraError::TypeError(format!("Cannot convert '{}' to float", s)))?;
                                    return Ok(Value::Float(n));
                                }
                                Value::Integer(n) => return Ok(Value::Float(n as f64)),
                                Value::Float(f) => return Ok(Value::Float(f)),
                                _ => return Err(NexoraError::TypeError(format!("Cannot convert {} to float", val.type_name()))),
                            }
                        }
                        "len" => {
                            if args.len() != 1 {
                                return Err(NexoraError::WrongArity { expected: 1, found: args.len() });
                            }
                            let val = self.eval_expr(&args[0])?;
                            return match val {
                                Value::Array(a) => Ok(Value::Integer(a.len() as i64)),
                                Value::String(s) => Ok(Value::Integer(s.len() as i64)),
                                Value::Object(m) => Ok(Value::Integer(m.len() as i64)),
                                _ => Err(NexoraError::TypeError(format!("Cannot get length of {}", val.type_name()))),
                            };
                        }
                        "push" => {
                            if args.len() != 2 {
                                return Err(NexoraError::WrongArity { expected: 2, found: args.len() });
                            }
                            let elem = self.eval_expr(&args[1])?;
                            if let Expr::Ident(name) = &args[0] {
                                let mut arr_val = self.get_var(name)?;
                                if let Value::Array(ref mut arr) = arr_val {
                                    arr.push(elem);
                                    self.set_var(name, arr_val.clone());
                                    return Ok(arr_val);
                                }
                                return Err(NexoraError::TypeError(format!("Cannot push to {}", arr_val.type_name())));
                            }
                            let mut arr_val = self.eval_expr(&args[0])?;
                            if let Value::Array(ref mut arr) = arr_val {
                                arr.push(elem);
                                return Ok(arr_val);
                            }
                            return Err(NexoraError::TypeError(format!("Cannot push to {}", arr_val.type_name())));
                        }
                        "pop" => {
                            if args.len() != 1 {
                                return Err(NexoraError::WrongArity { expected: 1, found: args.len() });
                            }
                            if let Expr::Ident(name) = &args[0] {
                                let mut arr_val = self.get_var(name)?;
                                if let Value::Array(ref mut arr) = arr_val {
                                    let elem = arr.pop().ok_or_else(|| NexoraError::IndexOutOfBounds("Cannot pop from empty array".to_string()))?;
                                    self.set_var(name, arr_val.clone());
                                    return Ok(elem);
                                }
                                return Err(NexoraError::TypeError(format!("Cannot pop from {}", arr_val.type_name())));
                            }
                            return Err(NexoraError::TypeError("pop requires a variable".to_string()));
                        }
                        "index_of" => {
                            if args.len() != 2 {
                                return Err(NexoraError::WrongArity { expected: 2, found: args.len() });
                            }
                            let haystack = self.eval_expr(&args[0])?;
                            let needle = self.eval_expr(&args[1])?;
                            match (&haystack, &needle) {
                                (Value::String(h), Value::String(n)) => {
                                    return Ok(Value::Integer(h.find(n).map(|i| i as i64).unwrap_or(-1)));
                                }
                                (Value::Array(arr), needle) => {
                                    for (i, item) in arr.iter().enumerate() {
                                        if item == needle {
                                            return Ok(Value::Integer(i as i64));
                                        }
                                    }
                                    return Ok(Value::Integer(-1));
                                }
                                _ => return Err(NexoraError::TypeError(format!("Cannot search in {}", haystack.type_name()))),
                            }
                        }
                        "slice" => {
                            if args.len() < 2 || args.len() > 3 {
                                return Err(NexoraError::WrongArity { expected: 2, found: args.len() });
                            }
                            let obj = self.eval_expr(&args[0])?;
                            let start = self.eval_expr(&args[1])?;
                            let end = if args.len() > 3 {
                                self.eval_expr(&args[3])?
                            } else if args.len() > 2 {
                                self.eval_expr(&args[2])?
                            } else {
                                match &obj {
                                    Value::String(s) => Value::Integer(s.len() as i64),
                                    Value::Array(a) => Value::Integer(a.len() as i64),
                                    _ => return Err(NexoraError::TypeError(format!("Cannot slice {}", obj.type_name()))),
                                }
                            };
                            let s = match start { Value::Integer(i) => i as usize, _ => return Err(NexoraError::TypeError("Slice index must be integer".to_string())) };
                            let e = match end { Value::Integer(i) => i as usize, _ => return Err(NexoraError::TypeError("Slice index must be integer".to_string())) };
                            match obj {
                                Value::String(ref string) => {
                                    let slice = if e <= string.len() { &string[s..e] } else { &string[s..] };
                                    return Ok(Value::String(slice.to_string()));
                                }
                                Value::Array(ref arr) => {
                                    let slice = if e <= arr.len() { arr[s..e].to_vec() } else { arr[s..].to_vec() };
                                    return Ok(Value::Array(slice));
                                }
                                _ => return Err(NexoraError::TypeError(format!("Cannot slice {}", obj.type_name()))),
                            }
                        }
                        "split" => {
                            if args.len() != 2 {
                                return Err(NexoraError::WrongArity { expected: 2, found: args.len() });
                            }
                            let s = self.eval_expr(&args[0])?;
                            let delim = self.eval_expr(&args[1])?;
                            match (&s, &delim) {
                                (Value::String(s), Value::String(d)) => {
                                    let parts: Vec<Value> = s.split(d).map(|p| Value::String(p.to_string())).collect();
                                    return Ok(Value::Array(parts));
                                }
                                _ => return Err(NexoraError::TypeError("split requires strings".to_string())),
                            }
                        }
                        "join" => {
                            if args.len() != 2 {
                                return Err(NexoraError::WrongArity { expected: 2, found: args.len() });
                            }
                            let arr = self.eval_expr(&args[0])?;
                            let delim = self.eval_expr(&args[1])?;
                            match (&arr, &delim) {
                                (Value::Array(a), Value::String(d)) => {
                                    let parts: Vec<String> = a.iter().map(|v| v.to_string()).collect();
                                    return Ok(Value::String(parts.join(d)));
                                }
                                _ => return Err(NexoraError::TypeError("join requires array and string".to_string())),
                            }
                        }
                        "contains" => {
                            if args.len() != 2 {
                                return Err(NexoraError::WrongArity { expected: 2, found: args.len() });
                            }
                            let haystack = self.eval_expr(&args[0])?;
                            let needle = self.eval_expr(&args[1])?;
                            match (&haystack, &needle) {
                                (Value::String(h), Value::String(n)) => {
                                    return Ok(Value::Bool(h.contains(n)));
                                }
                                (Value::Array(a), needle) => {
                                    return Ok(Value::Bool(a.iter().any(|item| item == needle)));
                                }
                                _ => return Err(NexoraError::TypeError(format!("Cannot check contains for {}", haystack.type_name()))),
                            }
                        }
                        "replace" => {
                            if args.len() != 3 {
                                return Err(NexoraError::WrongArity { expected: 3, found: args.len() });
                            }
                            let s = self.eval_expr(&args[0])?;
                            let from = self.eval_expr(&args[1])?;
                            let to = self.eval_expr(&args[2])?;
                            match (&s, &from, &to) {
                                (Value::String(s), Value::String(f), Value::String(t)) => {
                                    return Ok(Value::String(s.replace(f, t)));
                                }
                                _ => return Err(NexoraError::TypeError("replace requires strings".to_string())),
                            }
                        }
                        "upper" => {
                            if args.len() != 1 {
                                return Err(NexoraError::WrongArity { expected: 1, found: args.len() });
                            }
                            let s = self.eval_expr(&args[0])?;
                            match s {
                                Value::String(s) => return Ok(Value::String(s.to_uppercase())),
                                _ => return Err(NexoraError::TypeError("upper requires a string".to_string())),
                            }
                        }
                        "lower" => {
                            if args.len() != 1 {
                                return Err(NexoraError::WrongArity { expected: 1, found: args.len() });
                            }
                            let s = self.eval_expr(&args[0])?;
                            match s {
                                Value::String(s) => return Ok(Value::String(s.to_lowercase())),
                                _ => return Err(NexoraError::TypeError("lower requires a string".to_string())),
                            }
                        }
                        "trim" => {
                            if args.len() != 1 {
                                return Err(NexoraError::WrongArity { expected: 1, found: args.len() });
                            }
                            let s = self.eval_expr(&args[0])?;
                            match s {
                                Value::String(s) => return Ok(Value::String(s.trim().to_string())),
                                _ => return Err(NexoraError::TypeError("trim requires a string".to_string())),
                            }
                        }
                        "starts_with" => {
                            if args.len() != 2 {
                                return Err(NexoraError::WrongArity { expected: 2, found: args.len() });
                            }
                            let s = self.eval_expr(&args[0])?;
                            let prefix = self.eval_expr(&args[1])?;
                            match (&s, &prefix) {
                                (Value::String(s), Value::String(p)) => return Ok(Value::Bool(s.starts_with(p))),
                                _ => return Err(NexoraError::TypeError("starts_with requires strings".to_string())),
                            }
                        }
                        "ends_with" => {
                            if args.len() != 2 {
                                return Err(NexoraError::WrongArity { expected: 2, found: args.len() });
                            }
                            let s = self.eval_expr(&args[0])?;
                            let suffix = self.eval_expr(&args[1])?;
                            match (&s, &suffix) {
                                (Value::String(s), Value::String(p)) => return Ok(Value::Bool(s.ends_with(p))),
                                _ => return Err(NexoraError::TypeError("ends_with requires strings".to_string())),
                            }
                        }
                        "repeat" => {
                            if args.len() != 2 {
                                return Err(NexoraError::WrongArity { expected: 2, found: args.len() });
                            }
                            let s = self.eval_expr(&args[0])?;
                            let n = self.eval_expr(&args[1])?;
                            match (&s, &n) {
                                (Value::String(s), Value::Integer(n)) => return Ok(Value::String(s.repeat(*n as usize))),
                                _ => return Err(NexoraError::TypeError("repeat requires string and integer".to_string())),
                            }
                        }
                        "char_at" => {
                            if args.len() != 2 {
                                return Err(NexoraError::WrongArity { expected: 2, found: args.len() });
                            }
                            let s = self.eval_expr(&args[0])?;
                            let i = self.eval_expr(&args[1])?;
                            match (&s, &i) {
                                (Value::String(s), Value::Integer(i)) => {
                                    let idx = *i as usize;
                                    let ch = s.chars().nth(idx).ok_or_else(|| NexoraError::IndexOutOfBounds(format!("Index {} out of bounds", idx)))?;
                                    return Ok(Value::String(ch.to_string()));
                                }
                                _ => return Err(NexoraError::TypeError("char_at requires string and integer".to_string())),
                            }
                        }
                        "to_chars" => {
                            if args.len() != 1 {
                                return Err(NexoraError::WrongArity { expected: 1, found: args.len() });
                            }
                            let s = self.eval_expr(&args[0])?;
                            match s {
                                Value::String(s) => {
                                    let chars: Vec<Value> = s.chars().map(|c| Value::String(c.to_string())).collect();
                                    return Ok(Value::Array(chars));
                                }
                                _ => return Err(NexoraError::TypeError("to_chars requires a string".to_string())),
                            }
                        }
                        "lines" => {
                            if args.len() != 1 {
                                return Err(NexoraError::WrongArity { expected: 1, found: args.len() });
                            }
                            let s = self.eval_expr(&args[0])?;
                            match s {
                                Value::String(s) => {
                                    let parts: Vec<Value> = s.lines().map(|l| Value::String(l.to_string())).collect();
                                    return Ok(Value::Array(parts));
                                }
                                _ => return Err(NexoraError::TypeError("lines requires a string".to_string())),
                            }
                        }
                        // === MATH FUNCTIONS ===
                        "sqrt" => {
                            if args.len() != 1 { return Err(NexoraError::WrongArity { expected: 1, found: args.len() }); }
                            let val = self.eval_expr(&args[0])?;
                            let num = match val { Value::Integer(n) => n as f64, Value::Float(f) => f, _ => return Err(NexoraError::TypeError("sqrt requires a number".to_string())) };
                            return Ok(Value::Float(num.sqrt()));
                        }
                        "pow" => {
                            if args.len() != 2 { return Err(NexoraError::WrongArity { expected: 2, found: args.len() }); }
                            let base = self.eval_expr(&args[0])?;
                            let exp = self.eval_expr(&args[1])?;
                            let b = match base { Value::Integer(n) => n as f64, Value::Float(f) => f, _ => return Err(NexoraError::TypeError("pow requires numbers".to_string())) };
                            let e = match exp { Value::Integer(n) => n as f64, Value::Float(f) => f, _ => return Err(NexoraError::TypeError("pow requires numbers".to_string())) };
                            return Ok(Value::Float(b.powf(e)));
                        }
                        "abs" => {
                            if args.len() != 1 { return Err(NexoraError::WrongArity { expected: 1, found: args.len() }); }
                            let val = self.eval_expr(&args[0])?;
                            match val { Value::Integer(n) => return Ok(Value::Integer(n.abs())), Value::Float(f) => return Ok(Value::Float(f.abs())), _ => return Err(NexoraError::TypeError("abs requires a number".to_string())) };
                        }
                        "floor" => {
                            if args.len() != 1 { return Err(NexoraError::WrongArity { expected: 1, found: args.len() }); }
                            let val = self.eval_expr(&args[0])?;
                            match val { Value::Float(f) => return Ok(Value::Integer(f.floor() as i64)), Value::Integer(n) => return Ok(Value::Integer(n)), _ => return Err(NexoraError::TypeError("floor requires a number".to_string())) };
                        }
                        "ceil" => {
                            if args.len() != 1 { return Err(NexoraError::WrongArity { expected: 1, found: args.len() }); }
                            let val = self.eval_expr(&args[0])?;
                            match val { Value::Float(f) => return Ok(Value::Integer(f.ceil() as i64)), Value::Integer(n) => return Ok(Value::Integer(n)), _ => return Err(NexoraError::TypeError("ceil requires a number".to_string())) };
                        }
                        "min" => {
                            if args.len() != 2 { return Err(NexoraError::WrongArity { expected: 2, found: args.len() }); }
                            let a = self.eval_expr(&args[0])?;
                            let b = self.eval_expr(&args[1])?;
                            match (&a, &b) {
                                (Value::Integer(x), Value::Integer(y)) => return Ok(Value::Integer(if x < y { *x } else { *y })),
                                (Value::Float(x), Value::Float(y)) => return Ok(Value::Float(if x < y { *x } else { *y })),
                                (Value::Integer(x), Value::Float(y)) => return Ok(Value::Float(if (*x as f64) < *y { *x as f64 } else { *y })),
                                (Value::Float(x), Value::Integer(y)) => return Ok(Value::Float(if *x < (*y as f64) { *x } else { *y as f64 })),
                                _ => return Err(NexoraError::TypeError("min requires numbers".to_string())),
                            }
                        }
                        "max" => {
                            if args.len() != 2 { return Err(NexoraError::WrongArity { expected: 2, found: args.len() }); }
                            let a = self.eval_expr(&args[0])?;
                            let b = self.eval_expr(&args[1])?;
                            match (&a, &b) {
                                (Value::Integer(x), Value::Integer(y)) => return Ok(Value::Integer(if x > y { *x } else { *y })),
                                (Value::Float(x), Value::Float(y)) => return Ok(Value::Float(if x > y { *x } else { *y })),
                                (Value::Integer(x), Value::Float(y)) => return Ok(Value::Float(if (*x as f64) > *y { *x as f64 } else { *y })),
                                (Value::Float(x), Value::Integer(y)) => return Ok(Value::Float(if *x > (*y as f64) { *x } else { *y as f64 })),
                                _ => return Err(NexoraError::TypeError("max requires numbers".to_string())),
                            }
                        }
                        "random" => {
                            use rand::Rng;
                            let mut rng = rand::thread_rng();
                            if args.is_empty() {
                                return Ok(Value::Float(rng.gen_range(0.0..1.0)));
                            } else if args.len() == 1 {
                                let max = self.eval_expr(&args[0])?;
                                match max { Value::Integer(m) => return Ok(Value::Integer(rng.gen_range(0..m))), _ => return Err(NexoraError::TypeError("random max must be integer".to_string())) };
                            } else {
                                let lo = self.eval_expr(&args[0])?;
                                let hi = self.eval_expr(&args[1])?;
                                match (&lo, &hi) {
                                    (Value::Integer(a), Value::Integer(b)) => return Ok(Value::Integer(rng.gen_range(*a..*b))),
                                    _ => return Err(NexoraError::TypeError("random range requires integers".to_string())),
                                }
                            }
                        }
                        "round" => {
                            if args.len() != 1 { return Err(NexoraError::WrongArity { expected: 1, found: args.len() }); }
                            let val = self.eval_expr(&args[0])?;
                            match val { Value::Float(f) => return Ok(Value::Integer(f.round() as i64)), Value::Integer(n) => return Ok(Value::Integer(n)), _ => return Err(NexoraError::TypeError("round requires a number".to_string())) };
                        }
                        "sin" | "cos" | "tan" | "log" | "ln" => {
                            if args.len() != 1 { return Err(NexoraError::WrongArity { expected: 1, found: args.len() }); }
                            let val = self.eval_expr(&args[0])?;
                            let num = match val { Value::Integer(v) => v as f64, Value::Float(f) => f, _ => return Err(NexoraError::TypeError(format!("{} requires a number", n))) };
                            let result = match n.as_str() { "sin" => num.sin(), "cos" => num.cos(), "tan" => num.tan(), "log" => num.log10(), "ln" => num.ln(), _ => 0.0 };
                            return Ok(Value::Float(result));
                        }
                        // === FILE I/O ===
                        "read_file" => {
                            if args.len() != 1 { return Err(NexoraError::WrongArity { expected: 1, found: args.len() }); }
                            let path = self.eval_expr(&args[0])?;
                            match path {
                                Value::String(p) => {
                                    let content = fs::read_to_string(&p).map_err(|e| NexoraError::TypeError(format!("Cannot read file '{}': {}", p, e)))?;
                                    return Ok(Value::String(content));
                                }
                                _ => return Err(NexoraError::TypeError("read_file requires a string path".to_string())),
                            }
                        }
                        "write_file" => {
                            if args.len() != 2 { return Err(NexoraError::WrongArity { expected: 2, found: args.len() }); }
                            let path = self.eval_expr(&args[0])?;
                            let content = self.eval_expr(&args[1])?;
                            match (&path, &content) {
                                (Value::String(p), Value::String(c)) => {
                                    fs::write(p, c).map_err(|e| NexoraError::TypeError(format!("Cannot write file '{}': {}", p, e)))?;
                                    return Ok(Value::Bool(true));
                                }
                                _ => return Err(NexoraError::TypeError("write_file requires string path and content".to_string())),
                            }
                        }
                        "append_file" => {
                            if args.len() != 2 { return Err(NexoraError::WrongArity { expected: 2, found: args.len() }); }
                            let path = self.eval_expr(&args[0])?;
                            let content = self.eval_expr(&args[1])?;
                            match (&path, &content) {
                                (Value::String(p), Value::String(c)) => {
                                    use std::io::Write;
                                    let mut file = fs::OpenOptions::new().create(true).append(true).open(p).map_err(|e| NexoraError::TypeError(format!("Cannot append to file '{}': {}", p, e)))?;
                                    file.write_all(c.as_bytes()).map_err(|e| NexoraError::TypeError(format!("Cannot write to file '{}': {}", p, e)))?;
                                    return Ok(Value::Bool(true));
                                }
                                _ => return Err(NexoraError::TypeError("append_file requires string path and content".to_string())),
                            }
                        }
                        "file_exists" => {
                            if args.len() != 1 { return Err(NexoraError::WrongArity { expected: 1, found: args.len() }); }
                            let path = self.eval_expr(&args[0])?;
                            match path {
                                Value::String(p) => return Ok(Value::Bool(fs::metadata(&p).is_ok())),
                                _ => return Err(NexoraError::TypeError("file_exists requires a string path".to_string())),
                            }
                        }
                        "read_dir" => {
                            if args.len() != 1 { return Err(NexoraError::WrongArity { expected: 1, found: args.len() }); }
                            let path = self.eval_expr(&args[0])?;
                            match path {
                                Value::String(p) => {
                                    let entries = fs::read_dir(&p).map_err(|e| NexoraError::TypeError(format!("Cannot read directory '{}': {}", p, e)))?;
                                    let mut result = Vec::new();
                                    for entry in entries {
                                        if let Ok(entry) = entry {
                                            result.push(Value::String(entry.file_name().to_string_lossy().to_string()));
                                        }
                                    }
                                    return Ok(Value::Array(result));
                                }
                                _ => return Err(NexoraError::TypeError("read_dir requires a string path".to_string())),
                            }
                        }
                        // === JSON ===
                        "json_parse" => {
                            if args.len() != 1 { return Err(NexoraError::WrongArity { expected: 1, found: args.len() }); }
                            let val = self.eval_expr(&args[0])?;
                            match val {
                                Value::String(s) => {
                                    let json_val: serde_json::Value = serde_json::from_str(&s).map_err(|e| NexoraError::TypeError(format!("JSON parse error: {}", e)))?;
                                    return Ok(self.json_to_value(json_val));
                                }
                                _ => return Err(NexoraError::TypeError("json_parse requires a string".to_string())),
                            }
                        }
                        "json_stringify" => {
                            if args.is_empty() || args.len() > 2 { return Err(NexoraError::WrongArity { expected: 1, found: args.len() }); }
                            let val = self.eval_expr(&args[0])?;
                            let pretty = if args.len() == 2 {
                                match self.eval_expr(&args[1])? { Value::Bool(b) => b, _ => false }
                            } else { false };
                            let json_val = self.value_to_json(&val);
                            let s = if pretty { serde_json::to_string_pretty(&json_val).unwrap() } else { serde_json::to_string(&json_val).unwrap() };
                            return Ok(Value::String(s));
                        }
                        // === HTTP ===
                        "http_get" => {
                            if args.len() != 1 { return Err(NexoraError::WrongArity { expected: 1, found: args.len() }); }
                            let url = self.eval_expr(&args[0])?;
                            match url {
                                Value::String(u) => {
                                    let resp = reqwest::blocking::get(&u).map_err(|e| NexoraError::TypeError(format!("HTTP GET error: {}", e)))?;
                                    let text = resp.text().map_err(|e| NexoraError::TypeError(format!("HTTP read error: {}", e)))?;
                                    return Ok(Value::String(text));
                                }
                                _ => return Err(NexoraError::TypeError("http_get requires a string URL".to_string())),
                            }
                        }
                        "http_post" => {
                            if args.len() != 2 { return Err(NexoraError::WrongArity { expected: 2, found: args.len() }); }
                            let url = self.eval_expr(&args[0])?;
                            let body = self.eval_expr(&args[1])?;
                            match (&url, &body) {
                                (Value::String(u), Value::String(b)) => {
                                    let client = reqwest::blocking::Client::new();
                                    let resp = client.post(u).body(b.clone()).send().map_err(|e| NexoraError::TypeError(format!("HTTP POST error: {}", e)))?;
                                    let text = resp.text().map_err(|e| NexoraError::TypeError(format!("HTTP read error: {}", e)))?;
                                    return Ok(Value::String(text));
                                }
                                _ => return Err(NexoraError::TypeError("http_post requires string URL and body".to_string())),
                            }
                        }
                        // === HTML ===
                        "html" => {
                            if args.len() != 1 { return Err(NexoraError::WrongArity { expected: 1, found: args.len() }); }
                            let val = self.eval_expr(&args[0])?;
                            match val {
                                Value::Object(map) => {
                                    return Ok(Value::String(self.object_to_html(&map)));
                                }
                                Value::String(s) => {
                                    return Ok(Value::String(format!("<html><body>{}</body></html>", s)));
                                }
                                _ => return Err(NexoraError::TypeError("html requires an object or string".to_string())),
                            }
                        }
                        "element" => {
                            if args.len() < 2 { return Err(NexoraError::WrongArity { expected: 2, found: args.len() }); }
                            let tag = self.eval_expr(&args[0])?;
                            let content = self.eval_expr(&args[1])?;
                            let mut attrs = String::new();
                            if args.len() > 2 {
                                let attr_val = self.eval_expr(&args[2])?;
                                if let Value::Object(map) = attr_val {
                                    for (k, v) in &map {
                                        attrs.push_str(&format!(" {}=\"{}\"", k, v));
                                    }
                                }
                            }
                            match content {
                                Value::String(ref text) => {
                                    return Ok(Value::String(format!("<{}{}>{}</{}>", tag, attrs, text, tag)));
                                }
                                Value::Array(items) => {
                                    let inner: String = items.iter().map(|i| i.to_string()).collect::<Vec<_>>().join("");
                                    return Ok(Value::String(format!("<{}{}>{}</{}>", tag, attrs, inner, tag)));
                                }
                                _ => return Ok(Value::String(format!("<{}{}>{}</{}>", tag, attrs, content, tag))),
                            }
                        }
                        "div" | "span" | "p" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "a" | "ul" | "ol" | "li" | "table" | "tr" | "td" | "th" | "button" | "input_el" | "form" | "section" | "nav" | "header" | "footer" | "main" | "article" | "aside" => {
                            if args.is_empty() { return Err(NexoraError::WrongArity { expected: 1, found: 0 }); }
                            let content = self.eval_expr(&args[0])?;
                            let mut attrs = String::new();
                            if args.len() > 1 {
                                let attr_val = self.eval_expr(&args[1])?;
                                if let Value::Object(map) = attr_val {
                                    for (k, v) in &map {
                                        attrs.push_str(&format!(" {}=\"{}\"", k, v));
                                    }
                                }
                            }
                            match content {
                                Value::String(ref text) => return Ok(Value::String(format!("<{}{}>{}</{}>", n, attrs, text, n))),
                                Value::Array(items) => {
                                    let inner: String = items.iter().map(|i| i.to_string()).collect::<Vec<_>>().join("");
                                    return Ok(Value::String(format!("<{}{}>{}</{}>", n, attrs, inner, n)));
                                }
                                _ => return Ok(Value::String(format!("<{}{}>{}</{}>", n, attrs, content, n))),
                            }
                        }
                        "render" => {
                            if args.len() != 1 { return Err(NexoraError::WrongArity { expected: 1, found: args.len() }); }
                            let val = self.eval_expr(&args[0])?;
                            println!("{}", val);
                            return Ok(Value::Null);
                        }
                        "serve" => {
                            if args.len() != 2 { return Err(NexoraError::WrongArity { expected: 2, found: args.len() }); }
                            let port = self.eval_expr(&args[0])?;
                            let handler_val = self.eval_expr(&args[1])?;
                            let port_num = match port { Value::Integer(p) => p as u16, _ => return Err(NexoraError::TypeError("serve requires port number".to_string())) };
                            use std::net::TcpListener;
                            use std::io::Read;
                            let listener = TcpListener::bind(format!("127.0.0.1:{}", port_num))
                                .map_err(|e| NexoraError::TypeError(format!("Cannot start server: {}", e)))?;
                            println!("Server running on http://127.0.0.1:{}", port_num);
                            for stream in listener.incoming() {
                                match stream {
                                    Ok(mut stream) => {
                                        let mut buffer = [0; 4096];
                                        stream.read(&mut buffer).unwrap();
                                        let request = String::from_utf8_lossy(&buffer);
                                        let first_line = request.lines().next().unwrap_or("");
                                        let parts: Vec<&str> = first_line.split_whitespace().collect();
                                        let method = if parts.len() > 0 { parts[0] } else { "GET" };
                                        let path = if parts.len() > 1 { parts[1] } else { "/" };
                                        if let Value::Func { .. } = &handler_val {
                                            let result = self.call_function(&handler_val, vec![Value::String(method.to_string()), Value::String(path.to_string())])?;
                                            let body = match result { Value::String(s) => s, _ => result.to_string() };
                                            let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}", body.len(), body);
                                            stream.write_all(response.as_bytes()).unwrap();
                                        }
                                    }
                                    Err(e) => eprintln!("Connection failed: {}", e),
                                }
                            }
                            return Ok(Value::Null);
                        }
                        // === DATE/TIME ===
                        "now" => {
                            if !args.is_empty() { return Err(NexoraError::WrongArity { expected: 0, found: args.len() }); }
                            let duration = std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap();
                            return Ok(Value::Integer(duration.as_secs() as i64));
                        }
                        "timestamp" => {
                            if !args.is_empty() { return Err(NexoraError::WrongArity { expected: 0, found: args.len() }); }
                            let duration = std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap();
                            return Ok(Value::Float(duration.as_secs_f64()));
                        }
                        "sleep" => {
                            if args.len() != 1 { return Err(NexoraError::WrongArity { expected: 1, found: args.len() }); }
                            let ms = self.eval_expr(&args[0])?;
                            match ms {
                                Value::Integer(n) => {
                                    std::thread::sleep(std::time::Duration::from_millis(n as u64));
                                    return Ok(Value::Null);
                                }
                                Value::Float(f) => {
                                    std::thread::sleep(std::time::Duration::from_millis(f as u64));
                                    return Ok(Value::Null);
                                }
                                _ => return Err(NexoraError::TypeError("sleep requires a number (ms)".to_string())),
                            }
                        }
                        // === COLLECTION FUNCTIONS ===
                        "sort" => {
                            if args.len() != 1 { return Err(NexoraError::WrongArity { expected: 1, found: args.len() }); }
                            let val = self.eval_expr(&args[0])?;
                            match val {
                                Value::Array(mut arr) => {
                                    arr.sort_by(|a, b| a.to_string().partial_cmp(&b.to_string()).unwrap_or(std::cmp::Ordering::Equal));
                                    return Ok(Value::Array(arr));
                                }
                                _ => return Err(NexoraError::TypeError("sort requires an array".to_string())),
                            }
                        }
                        "reverse" => {
                            if args.len() != 1 { return Err(NexoraError::WrongArity { expected: 1, found: args.len() }); }
                            let val = self.eval_expr(&args[0])?;
                            match val {
                                Value::Array(mut arr) => {
                                    arr.reverse();
                                    return Ok(Value::Array(arr));
                                }
                                Value::String(s) => {
                                    return Ok(Value::String(s.chars().rev().collect()));
                                }
                                _ => return Err(NexoraError::TypeError("reverse requires array or string".to_string())),
                            }
                        }
                        "unique" => {
                            if args.len() != 1 { return Err(NexoraError::WrongArity { expected: 1, found: args.len() }); }
                            let val = self.eval_expr(&args[0])?;
                            match val {
                                Value::Array(arr) => {
                                    let mut seen = Vec::new();
                                    let mut result = Vec::new();
                                    for item in arr {
                                        let item_str = item.to_string();
                                        if !seen.contains(&item_str) {
                                            seen.push(item_str);
                                            result.push(item);
                                        }
                                    }
                                    return Ok(Value::Array(result));
                                }
                                _ => return Err(NexoraError::TypeError("unique requires an array".to_string())),
                            }
                        }
                        "flatten" => {
                            if args.len() != 1 { return Err(NexoraError::WrongArity { expected: 1, found: args.len() }); }
                            let val = self.eval_expr(&args[0])?;
                            match val {
                                Value::Array(arr) => {
                                    let mut result = Vec::new();
                                    for item in arr {
                                        match item {
                                            Value::Array(inner) => result.extend(inner),
                                            other => result.push(other),
                                        }
                                    }
                                    return Ok(Value::Array(result));
                                }
                                _ => return Err(NexoraError::TypeError("flatten requires an array".to_string())),
                            }
                        }
                        "keys" => {
                            if args.len() != 1 { return Err(NexoraError::WrongArity { expected: 1, found: args.len() }); }
                            let val = self.eval_expr(&args[0])?;
                            match val {
                                Value::Object(map) => {
                                    let keys: Vec<Value> = map.keys().map(|k| Value::String(k.clone())).collect();
                                    return Ok(Value::Array(keys));
                                }
                                _ => return Err(NexoraError::TypeError("keys requires an object".to_string())),
                            }
                        }
                        "values" => {
                            if args.len() != 1 { return Err(NexoraError::WrongArity { expected: 1, found: args.len() }); }
                            let val = self.eval_expr(&args[0])?;
                            match val {
                                Value::Object(map) => {
                                    let vals: Vec<Value> = map.values().cloned().collect();
                                    return Ok(Value::Array(vals));
                                }
                                _ => return Err(NexoraError::TypeError("values requires an object".to_string())),
                            }
                        }
                        "entries" => {
                            if args.len() != 1 { return Err(NexoraError::WrongArity { expected: 1, found: args.len() }); }
                            let val = self.eval_expr(&args[0])?;
                            match val {
                                Value::Object(map) => {
                                    let entries: Vec<Value> = map.iter().map(|(k, v)| {
                                        let mut pair = HashMap::new();
                                        pair.insert("key".to_string(), Value::String(k.clone()));
                                        pair.insert("value".to_string(), v.clone());
                                        Value::Object(pair)
                                    }).collect();
                                    return Ok(Value::Array(entries));
                                }
                                _ => return Err(NexoraError::TypeError("entries requires an object".to_string())),
                            }
                        }
                        "map" => {
                            if args.len() != 2 { return Err(NexoraError::WrongArity { expected: 2, found: args.len() }); }
                            let arr = self.eval_expr(&args[0])?;
                            let func = self.eval_expr(&args[1])?;
                            match &arr {
                                Value::Array(a) => {
                                    let mut result = Vec::new();
                                    for item in a {
                                        result.push(self.call_function(&func, vec![item.clone()])?);
                                    }
                                    return Ok(Value::Array(result));
                                }
                                _ => return Err(NexoraError::TypeError("map requires array and function".to_string())),
                            }
                        }
                        "filter" => {
                            if args.len() != 2 { return Err(NexoraError::WrongArity { expected: 2, found: args.len() }); }
                            let arr = self.eval_expr(&args[0])?;
                            let func = self.eval_expr(&args[1])?;
                            match &arr {
                                Value::Array(a) => {
                                    let mut result = Vec::new();
                                    for item in a {
                                        let keep = self.call_function(&func, vec![item.clone()])?.is_truthy();
                                        if keep {
                                            result.push(item.clone());
                                        }
                                    }
                                    return Ok(Value::Array(result));
                                }
                                _ => return Err(NexoraError::TypeError("filter requires array and function".to_string())),
                            }
                        }
                        "reduce" => {
                            if args.len() != 3 { return Err(NexoraError::WrongArity { expected: 3, found: args.len() }); }
                            let arr = self.eval_expr(&args[0])?;
                            let func = self.eval_expr(&args[1])?;
                            let init = self.eval_expr(&args[2])?;
                            match &arr {
                                Value::Array(a) => {
                                    let mut acc = init;
                                    for item in a {
                                        acc = self.call_function(&func, vec![acc, item.clone()])?;
                                    }
                                    return Ok(acc);
                                }
                                _ => return Err(NexoraError::TypeError("reduce requires array and function".to_string())),
                            }
                        }
                        "zip" => {
                            if args.len() < 2 { return Err(NexoraError::WrongArity { expected: 2, found: args.len() }); }
                            let mut arrays = Vec::new();
                            for arg in args {
                                let val = self.eval_expr(arg)?;
                                match val {
                                    Value::Array(a) => arrays.push(a),
                                    _ => return Err(NexoraError::TypeError("zip requires arrays".to_string())),
                                }
                            }
                            let min_len = arrays.iter().map(|a| a.len()).min().unwrap_or(0);
                            let mut result = Vec::new();
                            for i in 0..min_len {
                                let mut pair = Vec::new();
                                for arr in &arrays {
                                    pair.push(arr[i].clone());
                                }
                                result.push(Value::Array(pair));
                            }
                            return Ok(Value::Array(result));
                        }
                        "range" => {
                            if args.is_empty() || args.len() > 3 { return Err(NexoraError::WrongArity { expected: "1-3".parse().unwrap_or(2), found: args.len() }); }
                            if args.len() == 1 {
                                let end = self.eval_expr(&args[0])?;
                                match end {
                                    Value::Integer(e) => {
                                        let arr: Vec<Value> = (0..e).map(|i| Value::Integer(i)).collect();
                                        return Ok(Value::Array(arr));
                                    }
                                    _ => return Err(NexoraError::TypeError("range requires integer".to_string())),
                                }
                            } else if args.len() == 2 {
                                let start = self.eval_expr(&args[0])?;
                                let end = self.eval_expr(&args[1])?;
                                match (&start, &end) {
                                    (Value::Integer(s), Value::Integer(e)) => {
                                        let arr: Vec<Value> = (*s..*e).map(|i| Value::Integer(i)).collect();
                                        return Ok(Value::Array(arr));
                                    }
                                    _ => return Err(NexoraError::TypeError("range requires integers".to_string())),
                                }
                            } else {
                                let start = self.eval_expr(&args[0])?;
                                let end = self.eval_expr(&args[1])?;
                                let step = self.eval_expr(&args[2])?;
                                match (&start, &end, &step) {
                                    (Value::Integer(s), Value::Integer(e), Value::Integer(st)) => {
                                        let mut arr = Vec::new();
                                        let mut i = *s;
                                        if *st > 0 {
                                            while i < *e { arr.push(Value::Integer(i)); i += st; }
                                        } else if *st < 0 {
                                            while i > *e { arr.push(Value::Integer(i)); i += st; }
                                        }
                                        return Ok(Value::Array(arr));
                                    }
                                    _ => return Err(NexoraError::TypeError("range requires integers".to_string())),
                                }
                            }
                        }
                        // === ENV/EXEC ===
                        "env" => {
                            if args.len() != 1 { return Err(NexoraError::WrongArity { expected: 1, found: args.len() }); }
                            let key = self.eval_expr(&args[0])?;
                            match key {
                                Value::String(k) => {
                                    match std::env::var(&k) {
                                        Ok(val) => return Ok(Value::String(val)),
                                        Err(_) => return Ok(Value::Null),
                                    }
                                }
                                _ => return Err(NexoraError::TypeError("env requires a string key".to_string())),
                            }
                        }
                        "exec_command" => {
                            if args.len() != 1 { return Err(NexoraError::WrongArity { expected: 1, found: args.len() }); }
                            let cmd = self.eval_expr(&args[0])?;
                            match cmd {
                                Value::String(c) => {
                                    use std::process::Command;
                                    let output = if cfg!(target_os = "windows") {
                                        Command::new("cmd").args(["/c", &c]).output()
                                    } else {
                                        Command::new("sh").args(["-c", &c]).output()
                                    }.map_err(|e| NexoraError::TypeError(format!("Command error: {}", e)))?;
                                    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                                    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                                    let mut result = HashMap::new();
                                    result.insert("stdout".to_string(), Value::String(stdout));
                                    result.insert("stderr".to_string(), Value::String(stderr));
                                    result.insert("status".to_string(), Value::Integer(output.status.code().unwrap_or(-1) as i64));
                                    return Ok(Value::Object(result));
                                }
                                _ => return Err(NexoraError::TypeError("exec_command requires a string".to_string())),
                            }
                        }
                        // === ASSERT (as function) ===
                        "assert" => {
                            if args.len() < 1 || args.len() > 2 { return Err(NexoraError::WrongArity { expected: 1, found: args.len() }); }
                            let cond = self.eval_expr(&args[0])?;
                            if !cond.is_truthy() {
                                let msg = if args.len() > 2 {
                                    let m = self.eval_expr(&args[1])?;
                                    format!("Assert failed: {}", m)
                                } else {
                                    format!("Assert failed: {}", cond)
                                };
                                return Err(NexoraError::TypeError(msg));
                            }
                            return Ok(Value::Null);
                        }
                        _ => {}
                    }
                }

                let func_val = self.eval_expr(name)?;
                // Check if this is a method call (obj.method()) or super() to set 'this'
                let this_val = if let Expr::Property { object, .. } = name.as_ref() {
                    Some(self.eval_expr(object)?)
                } else if let Expr::Super(_) = name.as_ref() {
                    Some(self.get_var("this")?)
                } else {
                    None
                };
                // Special handling for super() calls - execute parent body directly
                // to preserve this modifications
                if let Expr::Super(_method_name) = name.as_ref() {
                    if let Value::Func { params, body, .. } = &func_val {
                        if args.len() != params.len() {
                            return Err(NexoraError::WrongArity { expected: params.len(), found: args.len() });
                        }
                        let mut arg_vals = Vec::new();
                        for arg in args {
                            arg_vals.push(self.eval_expr(arg)?);
                        }
                        // Execute parent body directly in current scope (like New handler does)
                        for (param, val) in params.iter().zip(arg_vals) {
                            self.set_var(param, val);
                        }
                        for stmt in body {
                            match self.exec_stmt(stmt) {
                                Ok(_) => {}
                                Err(NexoraError::ReturnValue(val)) => return Ok(val),
                                Err(e) => return Err(e),
                            }
                        }
                        return Ok(Value::Null);
                    }
                }
                match func_val {
                    Value::Func {
                        name: _func_name,
                        params,
                        body,
                    } => {
                        if args.len() != params.len() {
                            return Err(NexoraError::WrongArity {
                                expected: params.len(),
                                found: args.len(),
                            });
                        }
                        let mut arg_vals = Vec::new();
                        for arg in args {
                            arg_vals.push(self.eval_expr(arg)?);
                        }
                        self.push_scope();
                        if let Some(this_v) = this_val {
                            self.set_var("this", this_v);
                        }
                        for (param, val) in params.iter().zip(arg_vals) {
                            self.set_var(param, val);
                        }
                        let result = self.exec_block(&body);
                        self.pop_scope();
                        match result {
                            Err(NexoraError::ReturnValue(val)) => Ok(val),
                            Ok(_) => Ok(Value::Null),
                            Err(e) => Err(e),
                        }
                    }
                    Value::Closure {
                        params,
                        body,
                        captures,
                        ..
                    } => {
                        if args.len() != params.len() {
                            return Err(NexoraError::WrongArity {
                                expected: params.len(),
                                found: args.len(),
                            });
                        }
                        let mut arg_vals = Vec::new();
                        for arg in args {
                            arg_vals.push(self.eval_expr(arg)?);
                        }
                        self.push_scope();
                        // Restore captured variables
                        for (name, val) in captures.iter() {
                            self.set_var(name, val.clone());
                        }
                        if let Some(this_v) = this_val {
                            self.set_var("this", this_v);
                        }
                        for (param, val) in params.iter().zip(arg_vals) {
                            self.set_var(param, val);
                        }
                        let result = self.eval_expr(&body);
                        self.pop_scope();
                        result
                    }
                    _ => Err(NexoraError::NotCallable(func_val.type_name().to_string())),
                }
            }
            Expr::Index { object, index } => {
                let obj = self.eval_expr(object)?;
                let idx = self.eval_expr(index)?;
                match (&obj, &idx) {
                    (Value::Array(arr), Value::Integer(i)) => {
                        let i = *i as usize;
                        if i < arr.len() {
                            Ok(arr[i].clone())
                        } else {
                            Err(NexoraError::IndexOutOfBounds(format!(
                                "Index {} out of bounds for array of length {}",
                                i,
                                arr.len()
                            )))
                        }
                    }
                    (Value::Object(map), Value::String(key)) => Ok(map
                        .get(key)
                        .cloned()
                        .unwrap_or(Value::Null)),
                    (Value::String(s), Value::Integer(i)) => {
                        let i = *i as usize;
                        if let Some(ch) = s.chars().nth(i) {
                            Ok(Value::String(ch.to_string()))
                        } else {
                            Err(NexoraError::IndexOutOfBounds(format!(
                                "Index {} out of bounds for string of length {}",
                                i,
                                s.len()
                            )))
                        }
                    }
                    _ => Err(NexoraError::TypeError(format!(
                        "Cannot index into {}",
                        obj.type_name()
                    ))),
                }
            }
            Expr::Property { object, prop } => {
                let obj = self.eval_expr(object)?;
                match obj {
                    Value::Object(map) => Ok(map.get(prop).cloned().unwrap_or(Value::Null)),
                    Value::String(s) if prop == "length" => Ok(Value::Integer(s.len() as i64)),
                    Value::Array(a) if prop == "length" => Ok(Value::Integer(a.len() as i64)),
                    Value::ObjectInstance { class_name, fields } => {
                        if prop == "class" {
                            return Ok(Value::String(class_name.clone()));
                        }
                        if let Some(val) = fields.get(prop) {
                            return Ok(val.clone());
                        }
                        // Look up method from class definition
                        if let Ok(Value::ClassDef { methods, parent, .. }) = self.get_var(&class_name) {
                            // Search current class first
                            for method in &methods {
                                if let Stmt::Func { name: mname, params, body } = method {
                                    if mname == prop {
                                        return Ok(Value::Func {
                                            name: mname.clone(),
                                            params: params.clone(),
                                            body: body.clone(),
                                        });
                                    }
                                }
                            }
                            // Search parent classes
                            if let Some(ref parent_name) = parent {
                                if let Ok(Value::ClassDef { methods: parent_methods, .. }) = self.get_var(parent_name) {
                                    for method in &parent_methods {
                                        if let Stmt::Func { name: mname, params, body } = method {
                                            if mname == prop {
                                                return Ok(Value::Func {
                                                    name: mname.clone(),
                                                    params: params.clone(),
                                                    body: body.clone(),
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(NexoraError::UndefinedVariable(format!(
                            "Property '{}' not found on {} instance",
                            prop, class_name
                        )))
                    }
                    Value::ClassDef { name, methods, .. } => {
                        for method in methods {
                            if let Stmt::Func { name: mname, params, body } = method {
                                if mname == *prop {
                                    return Ok(Value::Func {
                                        name: mname.clone(),
                                        params: params.clone(),
                                        body: body.clone(),
                                    });
                                }
                            }
                        }
                        Err(NexoraError::UndefinedVariable(format!(
                            "Method '{}' not found on class {}",
                            prop, name
                        )))
                    }
                    _ => Err(NexoraError::TypeError(format!(
                        "Cannot access property '{}' on {}",
                        prop,
                        obj.type_name()
                    ))),
                }
            }
        }
    }

    fn eval_binop(&self, op: &BinOp, left: &Value, right: &Value) -> Result<Value, NexoraError> {
        match op {
            BinOp::Add => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
                (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
                (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a + *b as f64)),
                (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                (Value::String(a), b) => Ok(Value::String(format!("{}{}", a, b))),
                (a, Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                _ => Err(NexoraError::TypeError(format!(
                    "Cannot add {} and {}",
                    left.type_name(),
                    right.type_name()
                ))),
            },
            BinOp::Sub => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
                (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f64 - b)),
                (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a - *b as f64)),
                _ => Err(NexoraError::TypeError(format!(
                    "Cannot subtract {} from {}",
                    right.type_name(),
                    left.type_name()
                ))),
            },
            BinOp::Mul => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a * b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
                (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f64 * b)),
                (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a * *b as f64)),
                (Value::String(s), Value::Integer(n)) => {
                    Ok(Value::String(s.repeat(*n as usize)))
                }
                _ => Err(NexoraError::TypeError(format!(
                    "Cannot multiply {} and {}",
                    left.type_name(),
                    right.type_name()
                ))),
            },
            BinOp::Div => match (left, right) {
                (Value::Integer(_), Value::Integer(0))
                | (Value::Integer(_), Value::Float(0.0)) => {
                    Err(NexoraError::DivisionByZero)
                }
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a / b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
                (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f64 / b)),
                (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a / *b as f64)),
                _ => Err(NexoraError::TypeError(format!(
                    "Cannot divide {} by {}",
                    left.type_name(),
                    right.type_name()
                ))),
            },
            BinOp::Mod => match (left, right) {
                (Value::Integer(_), Value::Integer(0)) => Err(NexoraError::DivisionByZero),
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a % b)),
                _ => Err(NexoraError::TypeError(format!(
                    "Cannot apply modulo to {} and {}",
                    left.type_name(),
                    right.type_name()
                ))),
            },
            BinOp::Eq => Ok(Value::Bool(left == right)),
            BinOp::NotEq => Ok(Value::Bool(left != right)),
            BinOp::Lt => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Bool(a < b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a < b)),
                (Value::String(a), Value::String(b)) => Ok(Value::Bool(a < b)),
                _ => Err(NexoraError::TypeError(format!(
                    "Cannot compare {} and {}",
                    left.type_name(),
                    right.type_name()
                ))),
            },
            BinOp::Gt => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Bool(a > b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a > b)),
                (Value::String(a), Value::String(b)) => Ok(Value::Bool(a > b)),
                _ => Err(NexoraError::TypeError(format!(
                    "Cannot compare {} and {}",
                    left.type_name(),
                    right.type_name()
                ))),
            },
            BinOp::LtEq => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Bool(a <= b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a <= b)),
                (Value::String(a), Value::String(b)) => Ok(Value::Bool(a <= b)),
                _ => Err(NexoraError::TypeError(format!(
                    "Cannot compare {} and {}",
                    left.type_name(),
                    right.type_name()
                ))),
            },
            BinOp::GtEq => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Bool(a >= b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a >= b)),
                (Value::String(a), Value::String(b)) => Ok(Value::Bool(a >= b)),
                _ => Err(NexoraError::TypeError(format!(
                    "Cannot compare {} and {}",
                    left.type_name(),
                    right.type_name()
                ))),
            },
            BinOp::And => Ok(Value::Bool(left.is_truthy() && right.is_truthy())),
            BinOp::Or => Ok(Value::Bool(left.is_truthy() || right.is_truthy())),
        }
    }

    fn json_to_value(&self, json: serde_json::Value) -> Value {
        match json {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(b) => Value::Bool(b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Value::Integer(i)
                } else if let Some(f) = n.as_f64() {
                    Value::Float(f)
                } else {
                    Value::Null
                }
            }
            serde_json::Value::String(s) => Value::String(s),
            serde_json::Value::Array(arr) => {
                Value::Array(arr.into_iter().map(|v| self.json_to_value(v)).collect())
            }
            serde_json::Value::Object(map) => {
                let mut h = HashMap::new();
                for (k, v) in map {
                    h.insert(k, self.json_to_value(v));
                }
                Value::Object(h)
            }
        }
    }

    fn value_to_json(&self, val: &Value) -> serde_json::Value {
        match val {
            Value::Null => serde_json::Value::Null,
            Value::Bool(b) => serde_json::Value::Bool(*b),
            Value::Integer(n) => serde_json::json!(n),
            Value::Float(f) => serde_json::json!(f),
            Value::String(s) => serde_json::Value::String(s.clone()),
            Value::Array(arr) => {
                serde_json::Value::Array(arr.iter().map(|v| self.value_to_json(v)).collect())
            }
            Value::Object(map) => {
                let mut m = serde_json::Map::new();
                for (k, v) in map {
                    m.insert(k.clone(), self.value_to_json(v));
                }
                serde_json::Value::Object(m)
            }
            Value::Func { .. } => serde_json::Value::String("[Function]".to_string()),
            Value::Closure { .. } => serde_json::Value::String("[Closure]".to_string()),
            Value::ClassDef { name, .. } => serde_json::Value::String(format!("[Class {}]", name)),
            Value::ObjectInstance { class_name, .. } => serde_json::Value::String(format!("[{} instance]", class_name)),
            Value::GenericFunc { name, .. } => serde_json::Value::String(format!("[GenericFn {}]", name)),
            Value::GenericClass { name, .. } => serde_json::Value::String(format!("[GenericClass {}]", name)),
        }
    }

    fn object_to_html(&self, map: &HashMap<String, Value>) -> String {
        let tag = map.get("tag").map(|v| v.to_string()).unwrap_or_else(|| "div".to_string());
        let content = map.get("content").map(|v| v.to_string()).unwrap_or_default();
        let mut attrs = String::new();
        for (k, v) in map {
            if k != "tag" && k != "content" {
                attrs.push_str(&format!(" {}=\"{}\"", k, v));
            }
        }
        format!("<{}{}>{}</{}>", tag, attrs, content, tag)
    }

    pub fn call_function(&mut self, func_val: &Value, arg_vals: Vec<Value>) -> Result<Value, NexoraError> {
        match func_val {
            Value::Func { params, body, .. } => {
                if arg_vals.len() != params.len() {
                    return Err(NexoraError::WrongArity { expected: params.len(), found: arg_vals.len() });
                }
                self.push_scope();
                for (param, val) in params.iter().zip(arg_vals) {
                    self.set_var(param, val);
                }
                let result = self.exec_block(body);
                self.pop_scope();
                match result {
                    Err(NexoraError::ReturnValue(val)) => Ok(val),
                    Ok(_) => Ok(Value::Null),
                    Err(e) => Err(e),
                }
            }
            Value::Closure { params, body, captures, .. } => {
                if arg_vals.len() != params.len() {
                    return Err(NexoraError::WrongArity { expected: params.len(), found: arg_vals.len() });
                }
                self.push_scope();
                // Restore captured variables
                for (name, val) in captures.iter() {
                    self.set_var(name, val.clone());
                }
                // Set function parameters
                for (param, val) in params.iter().zip(arg_vals) {
                    self.set_var(param, val);
                }
                // Evaluate body expression
                let result = self.eval_expr(body);
                self.pop_scope();
                result
            }
            _ => Err(NexoraError::NotCallable(func_val.type_name().to_string())),
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn run_source(source: &str) -> Result<(), NexoraError> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let stmts = parser.parse();
        let mut interp = Interpreter::new();
        interp.run(&stmts)
    }

    #[test]
    fn test_hello_world() {
        assert!(run_source(r#"print "Hello World""#).is_ok());
    }

    #[test]
    fn test_arithmetic() {
        assert!(run_source("print 2 + 3 * 4").is_ok());
    }

    #[test]
    fn test_variables() {
        assert!(run_source("let x = 10; print x").is_ok());
    }

    #[test]
    fn test_if_else() {
        assert!(run_source("if true { print 1 } else { print 2 }").is_ok());
    }

    #[test]
    fn test_while_loop() {
        assert!(run_source("let i = 0; while i < 5 { i = i + 1 }").is_ok());
    }

    #[test]
    fn test_for_loop() {
        assert!(run_source("for i in [1, 2, 3] { print i }").is_ok());
    }

    #[test]
    fn test_function() {
        assert!(run_source("func add(a, b) { return a + b }; print add(1, 2)").is_ok());
    }

    #[test]
    fn test_array() {
        assert!(run_source("let arr = [1, 2, 3]; print arr[0]").is_ok());
    }

    #[test]
    fn test_object() {
        assert!(run_source(r#"let obj = { name: "test" }; print obj.name"#).is_ok());
    }

    #[test]
    fn test_string_concat() {
        assert!(run_source(r#"print "hello" + " " + "world""#).is_ok());
    }

    #[test]
    fn test_class() {
        assert!(run_source(r#"
            class Dog {
                func init(name) {
                    this.name = name
                }
                func bark() {
                    return this.name + " says woof"
                }
            }
            let d = new Dog("Rex")
            print d.bark()
        "#).is_ok());
    }

    #[test]
    fn test_try_catch() {
        assert!(run_source(r#"
            try {
                throw "error!"
            } catch (e) {
                print e
            }
        "#).is_ok());
    }

    #[test]
    fn test_assert() {
        assert!(run_source(r#"assert(1 + 1 == 2, "math works")"#).is_ok());
    }

    #[test]
    fn test_now() {
        assert!(run_source("let t = now(); print t").is_ok());
    }

    #[test]
    fn test_sort() {
        assert!(run_source("print sort([3, 1, 2])").is_ok());
    }

    #[test]
    fn test_map() {
        assert!(run_source("print map([1, 2, 3], func(x) { return x * 2 })").is_ok());
    }

    #[test]
    fn test_filter() {
        assert!(run_source("print filter([1, 2, 3, 4], func(x) { return x > 2 })").is_ok());
    }
}
