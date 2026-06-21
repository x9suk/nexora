use std::fmt;

use crate::value::Value;

#[derive(Debug, Clone)]
pub enum NexoraError {
    TypeError(String),
    UndefinedVariable(String),
    NotCallable(String),
    WrongArity { expected: usize, found: usize },
    IndexOutOfBounds(String),
    DivisionByZero,
    ReturnValue(Value),
    BreakSignal,
    ContinueSignal,
    ExceptionSignal(Value),
}

impl fmt::Display for NexoraError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NexoraError::TypeError(msg) => write!(f, "Type error: {}", msg),
            NexoraError::UndefinedVariable(name) => write!(f, "Undefined variable: {}", name),
            NexoraError::NotCallable(type_name) => write!(f, "Not callable: {}", type_name),
            NexoraError::WrongArity { expected, found } => {
                write!(f, "Wrong number of arguments: expected {}, found {}", expected, found)
            }
            NexoraError::IndexOutOfBounds(msg) => write!(f, "Index out of bounds: {}", msg),
            NexoraError::DivisionByZero => write!(f, "Division by zero"),
            NexoraError::ReturnValue(val) => write!(f, "Return: {}", val),
            NexoraError::BreakSignal => write!(f, "Break"),
            NexoraError::ContinueSignal => write!(f, "Continue"),
            NexoraError::ExceptionSignal(val) => write!(f, "Exception: {}", val),
        }
    }
}

impl std::error::Error for NexoraError {}
