use std::collections::HashMap;
use std::fmt;

use crate::ast::{Expr, Stmt};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Null,
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    Func {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    Closure {
        name: String,
        params: Vec<String>,
        body: Box<Expr>,
        captures: HashMap<String, Value>,
    },
    ClassDef {
        name: String,
        parent: Option<String>,
        methods: Vec<Stmt>,
    },
    ObjectInstance {
        class_name: String,
        fields: HashMap<String, Value>,
    },
    GenericFunc {
        name: String,
        type_params: Vec<String>,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    GenericClass {
        name: String,
        type_params: Vec<String>,
        parent: Option<String>,
        methods: Vec<Stmt>,
    },
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Bool(b) => *b,
            Value::Integer(n) => *n != 0,
            Value::Float(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(a) => !a.is_empty(),
            Value::Object(o) => !o.is_empty(),
            Value::Func { .. } => true,
            Value::Closure { .. } => true,
            Value::ClassDef { .. } => true,
            Value::ObjectInstance { .. } => true,
            Value::GenericFunc { .. } => true,
            Value::GenericClass { .. } => true,
        }
    }

    pub fn type_name(&self) -> &str {
        match self {
            Value::Integer(_) => "Integer",
            Value::Float(_) => "Float",
            Value::String(_) => "String",
            Value::Bool(_) => "Boolean",
            Value::Null => "Null",
            Value::Array(_) => "Array",
            Value::Object(_) => "Object",
            Value::Func { .. } => "Function",
            Value::Closure { .. } => "Closure",
            Value::ClassDef { .. } => "Class",
            Value::ObjectInstance { .. } => "Object",
            Value::GenericFunc { .. } => "GenericFunction",
            Value::GenericClass { .. } => "GenericClass",
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
            Value::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", items.join(", "))
            }
            Value::Object(obj) => {
                let items: Vec<String> = obj.iter().map(|(k, v)| format!("{}: {}", k, v)).collect();
                write!(f, "{{{}}}", items.join(", "))
            }
            Value::Func { name, .. } => write!(f, "<fn {}>", name),
            Value::Closure { name, .. } => write!(f, "<closure {}>", name),
            Value::ClassDef { name, .. } => write!(f, "<class {}>", name),
            Value::ObjectInstance { class_name, .. } => write!(f, "<{} instance>", class_name),
            Value::GenericFunc { name, .. } => write!(f, "<generic fn {}>", name),
            Value::GenericClass { name, .. } => write!(f, "<generic class {}>", name),
        }
    }
}
