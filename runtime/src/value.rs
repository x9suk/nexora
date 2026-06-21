use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};

use crate::ast::{Block, Param};

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    Function(Function),
    NativeFunction(NativeFunction),
    Class(Class),
    Instance(Instance),
    Closure(Closure),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub params: Vec<Param>,
    pub body: Block,
    pub name: String,
    pub is_async: bool,
}

#[derive(Debug, Clone)]
pub struct Closure {
    pub params: Vec<Param>,
    pub body: Block,
    pub env: Environment,
}

#[derive(Debug, Clone)]
pub struct NativeFunction {
    pub name: String,
    pub func: Arc<dyn Fn(&[Value]) -> Result<Value, RuntimeError> + Send + Sync>,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub superclass: Option<Box<Class>>,
    pub methods: HashMap<String, Value>,
    pub properties: HashMap<String, Value>,
    pub constructor: Option<Function>,
}

#[derive(Debug, Clone)]
pub struct Instance {
    pub class: Class,
    pub properties: HashMap<String, Value>,
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Boolean(b) => *b,
            Value::Integer(n) => *n != 0,
            Value::Float(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(arr) => !arr.is_empty(),
            Value::Object(obj) => !obj.is_empty(),
            _ => true,
        }
    }

    pub fn type_name(&self) -> &str {
        match self {
            Value::Integer(_) => "integer",
            Value::Float(_) => "float",
            Value::String(_) => "string",
            Value::Boolean(_) => "boolean",
            Value::Null => "null",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
            Value::Function(_) => "function",
            Value::NativeFunction(_) => "native_function",
            Value::Class(_) => "class",
            Value::Instance(_) => "instance",
            Value::Closure(_) => "closure",
        }
    }

    pub fn to_string_value(&self) -> String {
        match self {
            Value::Integer(n) => n.to_string(),
            Value::Float(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Boolean(b) => b.to_string(),
            Value::Null => "null".to_string(),
            Value::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_string_value()).collect();
                format!("[{}]", items.join(", "))
            }
            Value::Object(obj) => {
                let items: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string_value()))
                    .collect();
                format!("{{{}}}", items.join(", "))
            }
            Value::Function(f) => format!("<function {}>", f.name),
            Value::NativeFunction(f) => format!("<native {}>", f.name),
            Value::Class(c) => format!("<class {}>", c.name),
            Value::Instance(i) => format!("<instance of {}>", i.class.name),
            Value::Closure(_) => "<closure>".to_string(),
        }
    }

    pub fn to_number(&self) -> Result<f64, RuntimeError> {
        match self {
            Value::Integer(n) => Ok(*n as f64),
            Value::Float(n) => Ok(*n),
            _ => Err(RuntimeError::TypeError(format!(
                "Cannot convert {} to number",
                self.type_name()
            ))),
        }
    }

    pub fn add(&self, other: &Value) -> Result<Value, RuntimeError> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a + *b as f64)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::String(a), b) => Ok(Value::String(format!("{}{}", a, b.to_string_value()))),
            (a, Value::String(b)) => Ok(Value::String(format!("{}{}", a.to_string_value(), b))),
            (Value::Array(a), Value::Array(b)) => {
                let mut result = a.clone();
                result.extend(b.clone());
                Ok(Value::Array(result))
            }
            _ => Err(RuntimeError::TypeError(format!(
                "Cannot add {} and {}",
                self.type_name(),
                other.type_name()
            ))),
        }
    }

    pub fn subtract(&self, other: &Value) -> Result<Value, RuntimeError> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f64 - b)),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a - *b as f64)),
            _ => Err(RuntimeError::TypeError(format!(
                "Cannot subtract {} from {}",
                other.type_name(),
                self.type_name()
            ))),
        }
    }

    pub fn multiply(&self, other: &Value) -> Result<Value, RuntimeError> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f64 * b)),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a * *b as f64)),
            (Value::String(s), Value::Integer(n)) => Ok(Value::String(s.repeat(*n as usize))),
            _ => Err(RuntimeError::TypeError(format!(
                "Cannot multiply {} and {}",
                self.type_name(),
                other.type_name()
            ))),
        }
    }

    pub fn divide(&self, other: &Value) -> Result<Value, RuntimeError> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => {
                if *b == 0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::Integer(a / b))
            }
            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::Float(a / b))
            }
            (Value::Integer(a), Value::Float(b)) => {
                if *b == 0.0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::Float(*a as f64 / b))
            }
            (Value::Float(a), Value::Integer(b)) => {
                if *b == 0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::Float(a / *b as f64))
            }
            _ => Err(RuntimeError::TypeError(format!(
                "Cannot divide {} by {}",
                self.type_name(),
                other.type_name()
            ))),
        }
    }

    pub fn modulo(&self, other: &Value) -> Result<Value, RuntimeError> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => {
                if *b == 0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::Integer(a % b))
            }
            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::Float(a % b))
            }
            _ => Err(RuntimeError::TypeError(format!(
                "Cannot apply modulo to {} and {}",
                self.type_name(),
                other.type_name()
            ))),
        }
    }

    pub fn power(&self, other: &Value) -> Result<Value, RuntimeError> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a.pow(*b as u32))),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.powf(*b))),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float((*a as f64).powf(*b))),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a.powf(*b as f64))),
            _ => Err(RuntimeError::TypeError(format!(
                "Cannot raise {} to power {}",
                self.type_name(),
                other.type_name()
            ))),
        }
    }

    pub fn negate(&self) -> Result<Value, RuntimeError> {
        match self {
            Value::Integer(n) => Ok(Value::Integer(-n)),
            Value::Float(n) => Ok(Value::Float(-n)),
            _ => Err(RuntimeError::TypeError(format!(
                "Cannot negate {}",
                self.type_name()
            ))),
        }
    }

    pub fn equal(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Integer(a), Value::Float(b)) => *a as f64 == *b,
            (Value::Float(a), Value::Integer(b)) => *a == *b as f64,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }

    pub fn less_than(&self, other: &Value) -> Result<bool, RuntimeError> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(a < b),
            (Value::Float(a), Value::Float(b)) => Ok(a < b),
            (Value::Integer(a), Value::Float(b)) => Ok(*a as f64 < *b),
            (Value::Float(a), Value::Integer(b)) => Ok(*a < *b as f64),
            (Value::String(a), Value::String(b)) => Ok(a < b),
            _ => Err(RuntimeError::TypeError(format!(
                "Cannot compare {} and {}",
                self.type_name(),
                other.type_name()
            ))),
        }
    }

    pub fn greater_than(&self, other: &Value) -> Result<bool, RuntimeError> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(a > b),
            (Value::Float(a), Value::Float(b)) => Ok(a > b),
            (Value::Integer(a), Value::Float(b)) => Ok(*a as f64 > *b),
            (Value::Float(a), Value::Integer(b)) => Ok(*a > *b as f64),
            (Value::String(a), Value::String(b)) => Ok(a > b),
            _ => Err(RuntimeError::TypeError(format!(
                "Cannot compare {} and {}",
                self.type_name(),
                other.type_name()
            ))),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_value())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("Runtime error: {0}")]
    Error(String),
    #[error("Type error: {0}")]
    TypeError(String),
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Undefined variable: {0}")]
    UndefinedVariable(String),
    #[error("Undefined function: {0}")]
    UndefinedFunction(String),
    #[error("Undefined property: {0}")]
    UndefinedProperty(String),
    #[error("Index out of bounds: {0}")]
    IndexOutOfBounds(String),
    #[error("Maximum call stack size exceeded")]
    StackOverflow,
    #[error("Return value: {0}")]
    Return(Value),
    #[error("Break")]
    Break,
    #[error("Continue")]
    Continue,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone)]
pub struct Environment {
    variables: HashMap<String, Value>,
    parent: Option<Arc<Mutex<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Arc<Mutex<Environment>>) -> Self {
        Environment {
            variables: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.variables.get(name) {
            Some(value.clone())
        } else if let Some(parent) = &self.parent {
            parent.lock().unwrap().get(name)
        } else {
            None
        }
    }

    pub fn set(&mut self, name: &str, value: Value) -> bool {
        if self.variables.contains_key(name) {
            self.variables.insert(name.to_string(), value);
            true
        } else if let Some(parent) = &mut self.parent {
            parent.lock().unwrap().set(name, value)
        } else {
            false
        }
    }
}
