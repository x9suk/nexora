use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::bytecode::{CompiledFunction, CompiledModule, Constant};
use crate::opcodes::Opcode;

/// Runtime value
#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    Function(Arc<CompiledFunction>),
    Closure(Arc<Closure>),
    NativeFunction(Arc<dyn Fn(&[Value]) -> Result<Value, RuntimeError> + Send + Sync>),
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
            Value::Integer(_) => "Int",
            Value::Float(_) => "Float",
            Value::String(_) => "String",
            Value::Boolean(_) => "Bool",
            Value::Null => "Null",
            Value::Array(_) => "Array",
            Value::Object(_) => "Object",
            Value::Function(_) => "Function",
            Value::Closure(_) => "Closure",
            Value::NativeFunction(_) => "NativeFunction",
        }
    }

    pub fn add(&self, other: &Value) -> Result<Value, RuntimeError> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a + *b as f64)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::String(a), b) => Ok(Value::String(format!("{}{}", a, b))),
            (a, Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
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

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
            Value::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", items.join(", "))
            }
            Value::Object(obj) => {
                let items: Vec<String> = obj.iter().map(|(k, v)| format!("{}: {}", k, v)).collect();
                write!(f, "{{{}}}", items.join(", "))
            }
            Value::Function(func) => write!(f, "<fn {}>", func.name),
            Value::Closure(closure) => write!(f, "<fn {}>", closure.function.name),
            Value::NativeFunction(func) => write!(f, "<native fn>"),
        }
    }
}

/// Closure for capturing variables
#[derive(Debug, Clone)]
pub struct Closure {
    pub function: Arc<CompiledFunction>,
    pub upvalues: Vec<Arc<Mutex<Value>>>,
}

/// Runtime error
#[derive(Debug, Clone)]
pub enum RuntimeError {
    TypeError(String),
    UndefinedVariable(String),
    UndefinedProperty(String),
    DivisionByZero,
    StackOverflow,
    IndexOutOfBounds(String),
    MissingArgument(String),
    NotCallable(String),
    Return(Value),
    Break,
    Continue,
    Error(String),
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::TypeError(msg) => write!(f, "Type error: {}", msg),
            RuntimeError::UndefinedVariable(name) => write!(f, "Undefined variable: {}", name),
            RuntimeError::UndefinedProperty(name) => write!(f, "Undefined property: {}", name),
            RuntimeError::DivisionByZero => write!(f, "Division by zero"),
            RuntimeError::StackOverflow => write!(f, "Stack overflow"),
            RuntimeError::IndexOutOfBounds(msg) => write!(f, "Index out of bounds: {}", msg),
            RuntimeError::MissingArgument(msg) => write!(f, "Missing argument: {}", msg),
            RuntimeError::NotCallable(msg) => write!(f, "Not callable: {}", msg),
            RuntimeError::Return(_) => write!(f, "Return"),
            RuntimeError::Break => write!(f, "Break"),
            RuntimeError::Continue => write!(f, "Continue"),
            RuntimeError::Error(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for RuntimeError {}

/// Call frame for function calls
#[derive(Debug, Clone)]
struct CallFrame {
    function: Arc<CompiledFunction>,
    ip: usize,
    stack_base: usize,
}

/// Virtual Machine for executing bytecode
pub struct VM {
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
    frames: Vec<CallFrame>,
    open_upvalues: Vec<Arc<Mutex<Value>>>,
}

impl VM {
    pub fn new() -> Self {
        let mut vm = VM {
            stack: Vec::new(),
            globals: HashMap::new(),
            frames: Vec::new(),
            open_upvalues: Vec::new(),
        };

        // Register built-in functions
        vm.register_builtins();

        vm
    }

    fn register_builtins(&mut self) {
        // print function
        self.globals.insert(
            "print".to_string(),
            Value::NativeFunction(Arc::new(|args| {
                let output: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                println!("{}", output.join(" "));
                Ok(Value::Null)
            })),
        );

        // len function
        self.globals.insert(
            "len".to_string(),
            Value::NativeFunction(Arc::new(|args| {
                if args.len() != 1 {
                    return Err(RuntimeError::MissingArgument(
                        "len requires 1 argument".to_string(),
                    ));
                }
                match &args[0] {
                    Value::Array(arr) => Ok(Value::Integer(arr.len() as i64)),
                    Value::String(s) => Ok(Value::Integer(s.len() as i64)),
                    _ => Err(RuntimeError::TypeError(format!(
                        "Cannot get length of {}",
                        args[0].type_name()
                    ))),
                }
            })),
        );

        // str function
        self.globals.insert(
            "str".to_string(),
            Value::NativeFunction(Arc::new(|args| {
                if args.len() != 1 {
                    return Err(RuntimeError::MissingArgument(
                        "str requires 1 argument".to_string(),
                    ));
                }
                Ok(Value::String(args[0].to_string()))
            })),
        );

        // parseInt function
        self.globals.insert(
            "parseInt".to_string(),
            Value::NativeFunction(Arc::new(|args| {
                if args.len() != 1 {
                    return Err(RuntimeError::MissingArgument(
                        "parseInt requires 1 argument".to_string(),
                    ));
                }
                match &args[0] {
                    Value::String(s) => s
                        .parse::<i64>()
                        .map(Value::Integer)
                        .map_err(|_| RuntimeError::TypeError("Invalid integer".to_string())),
                    Value::Integer(n) => Ok(Value::Integer(*n)),
                    _ => Err(RuntimeError::TypeError(format!(
                        "Cannot parse {} as integer",
                        args[0].type_name()
                    ))),
                }
            })),
        );

        // Math object
        let mut math = HashMap::new();
        math.insert("PI".to_string(), Value::Float(std::f64::consts::PI));
        math.insert("E".to_string(), Value::Float(std::f64::consts::E));
        math.insert(
            "abs".to_string(),
            Value::NativeFunction(Arc::new(|args| {
                if args.len() != 1 {
                    return Err(RuntimeError::MissingArgument(
                        "abs requires 1 argument".to_string(),
                    ));
                }
                match &args[0] {
                    Value::Integer(n) => Ok(Value::Integer(n.abs())),
                    Value::Float(n) => Ok(Value::Float(n.abs())),
                    _ => Err(RuntimeError::TypeError("abs requires a number".to_string())),
                }
            })),
        );
        math.insert(
            "sqrt".to_string(),
            Value::NativeFunction(Arc::new(|args| {
                if args.len() != 1 {
                    return Err(RuntimeError::MissingArgument(
                        "sqrt requires 1 argument".to_string(),
                    ));
                }
                let n = match &args[0] {
                    Value::Integer(n) => *n as f64,
                    Value::Float(n) => *n,
                    _ => {
                        return Err(RuntimeError::TypeError(
                            "sqrt requires a number".to_string(),
                        ))
                    }
                };
                Ok(Value::Float(n.sqrt()))
            })),
        );
        self.globals.insert("Math".to_string(), Value::Object(math));
    }

    /// Run a compiled module
    pub fn run(&mut self, module: &CompiledModule) -> Result<Value, RuntimeError> {
        // Push main function onto stack
        let main = Arc::new(module.main.clone());
        self.push_frame(main, 0);

        self.execute()
    }

    /// Execute bytecode
    fn execute(&mut self) -> Result<Value, RuntimeError> {
        loop {
            if self.frames.is_empty() {
                return Ok(self.pop());
            }

            let frame = self.current_frame();
            let function = frame.function.clone();
            let ip = frame.ip;

            if ip >= function.bytecode.len() {
                self.pop_frame();
                continue;
            }

            let instruction = &function.bytecode[ip];
            let opcode = instruction.opcode;

            // Increment IP
            self.current_frame_mut().ip += 1;

            match opcode {
                Opcode::NOP => {}

                Opcode::POP => {
                    self.pop();
                }

                Opcode::DUP => {
                    let value = self.peek().clone();
                    self.push(value);
                }

                Opcode::SWAP => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(a);
                    self.push(b);
                }

                Opcode::CONST => {
                    let index = instruction.operands[0] as usize;
                    let constant = function.constants[index].clone();
                    let value = self.constant_to_value(&constant);
                    self.push(value);
                }

                Opcode::LOCAL => {
                    let slot = instruction.operands[0] as usize;
                    let stack_base = self.current_frame().stack_base;
                    let value = self.stack[stack_base + slot].clone();
                    self.push(value);
                }

                Opcode::SET_LOCAL => {
                    let slot = instruction.operands[0] as usize;
                    let stack_base = self.current_frame().stack_base;
                    let value = self.peek().clone();
                    self.stack[stack_base + slot] = value;
                }

                Opcode::LOCAL_0 | Opcode::LOCAL_1 | Opcode::LOCAL_2 | Opcode::LOCAL_3 => {
                    let slot = (opcode as u8 - Opcode::LOCAL_0 as u8) as usize;
                    let stack_base = self.current_frame().stack_base;
                    let value = self.stack[stack_base + slot].clone();
                    self.push(value);
                }

                Opcode::SET_LOCAL_0
                | Opcode::SET_LOCAL_1
                | Opcode::SET_LOCAL_2
                | Opcode::SET_LOCAL_3 => {
                    let slot = (opcode as u8 - Opcode::SET_LOCAL_0 as u8) as usize;
                    let stack_base = self.current_frame().stack_base;
                    let value = self.peek().clone();
                    self.stack[stack_base + slot] = value;
                }

                Opcode::GLOBAL => {
                    let name = match &function.constants[instruction.operands[0] as usize] {
                        Constant::String(s) => s.clone(),
                        _ => return Err(RuntimeError::Error("Invalid global name".to_string())),
                    };
                    let value = self.globals.get(&name).cloned().ok_or_else(|| {
                        RuntimeError::UndefinedVariable(name)
                    })?;
                    self.push(value);
                }

                Opcode::SET_GLOBAL => {
                    let name = match &function.constants[instruction.operands[0] as usize] {
                        Constant::String(s) => s.clone(),
                        _ => return Err(RuntimeError::Error("Invalid global name".to_string())),
                    };
                    let value = self.peek().clone();
                    self.globals.insert(name, value);
                }

                Opcode::ADD => {
                    let b = self.pop();
                    let a = self.pop();
                    let result = a.add(&b)?;
                    self.push(result);
                }

                Opcode::SUB => {
                    let b = self.pop();
                    let a = self.pop();
                    let result = a.subtract(&b)?;
                    self.push(result);
                }

                Opcode::MUL => {
                    let b = self.pop();
                    let a = self.pop();
                    let result = a.multiply(&b)?;
                    self.push(result);
                }

                Opcode::DIV => {
                    let b = self.pop();
                    let a = self.pop();
                    let result = a.divide(&b)?;
                    self.push(result);
                }

                Opcode::MOD => {
                    let b = self.pop();
                    let a = self.pop();
                    let result = a.modulo(&b)?;
                    self.push(result);
                }

                Opcode::POW => {
                    let b = self.pop();
                    let a = self.pop();
                    let result = a.power(&b)?;
                    self.push(result);
                }

                Opcode::NEG => {
                    let value = self.pop();
                    let result = value.negate()?;
                    self.push(result);
                }

                Opcode::EQ => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Boolean(a.equal(&b)));
                }

                Opcode::NEQ => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Boolean(!a.equal(&b)));
                }

                Opcode::LT => {
                    let b = self.pop();
                    let a = self.pop();
                    let result = a.less_than(&b)?;
                    self.push(Value::Boolean(result));
                }

                Opcode::GT => {
                    let b = self.pop();
                    let a = self.pop();
                    let result = a.greater_than(&b)?;
                    self.push(Value::Boolean(result));
                }

                Opcode::LTE => {
                    let b = self.pop();
                    let a = self.pop();
                    let result = a.less_than(&b)? || a.equal(&b);
                    self.push(Value::Boolean(result));
                }

                Opcode::GTE => {
                    let b = self.pop();
                    let a = self.pop();
                    let result = a.greater_than(&b)? || a.equal(&b);
                    self.push(Value::Boolean(result));
                }

                Opcode::LAND => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Boolean(a.is_truthy() && b.is_truthy()));
                }

                Opcode::LOR => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Boolean(a.is_truthy() || b.is_truthy()));
                }

                Opcode::LNOT => {
                    let value = self.pop();
                    self.push(Value::Boolean(!value.is_truthy()));
                }

                Opcode::JUMP => {
                    let offset = instruction.operands[0] as usize;
                    self.current_frame_mut().ip += offset;
                }

                Opcode::JUMP_IF => {
                    let offset = instruction.operands[0] as usize;
                    let condition = self.pop();
                    if condition.is_truthy() {
                        self.current_frame_mut().ip += offset;
                    }
                }

                Opcode::JUMP_IF_NOT => {
                    let offset = instruction.operands[0] as usize;
                    let condition = self.pop();
                    if !condition.is_truthy() {
                        self.current_frame_mut().ip += offset;
                    }
                }

                Opcode::LOOP => {
                    let offset = instruction.operands[0] as usize;
                    self.current_frame_mut().ip -= offset;
                }

                Opcode::CALL => {
                    let arg_count = instruction.operands[0] as usize;
                    let callee = self.stack[self.stack.len() - 1 - arg_count].clone();

                    match callee {
                        Value::Function(func) => {
                            let args: Vec<Value> = self
                                .stack
                                .drain(self.stack.len() - arg_count..)
                                .collect();
                            self.pop(); // pop function

                            if arg_count != func.arity {
                                return Err(RuntimeError::MissingArgument(format!(
                                    "Expected {} arguments, got {}",
                                    func.arity, arg_count
                                )));
                            }

                            self.push_frame(func, self.stack.len());
                        }
                        Value::NativeFunction(func) => {
                            let args: Vec<Value> = self
                                .stack
                                .drain(self.stack.len() - arg_count..)
                                .collect();
                            self.pop(); // pop function

                            let result = func(&args)?;
                            self.push(result);
                        }
                        Value::Closure(closure) => {
                            let args: Vec<Value> = self
                                .stack
                                .drain(self.stack.len() - arg_count..)
                                .collect();
                            self.pop(); // pop closure

                            if arg_count != closure.function.arity {
                                return Err(RuntimeError::MissingArgument(format!(
                                    "Expected {} arguments, got {}",
                                    closure.function.arity, arg_count
                                )));
                            }

                            self.push_frame(closure.function.clone(), self.stack.len());
                        }
                        _ => {
                            return Err(RuntimeError::NotCallable(callee.type_name().to_string()));
                        }
                    }
                }

                Opcode::RETURN => {
                    let value = self.pop();
                    self.pop_frame();
                    if self.frames.is_empty() {
                        return Ok(value);
                    }
                    self.push(value);
                }

                Opcode::CLOSURE => {
                    let func_index = instruction.operands[0] as usize;
                    let func = match &function.constants[func_index] {
                        Constant::Function(f) => Arc::new(f.clone()),
                        _ => {
                            return Err(RuntimeError::Error(
                                "Invalid closure target".to_string(),
                            ))
                        }
                    };

                    let closure = Closure {
                        function: func,
                        upvalues: Vec::new(),
                    };

                    self.push(Value::Closure(Arc::new(closure)));
                }

                Opcode::GET_PROPERTY => {
                    let object = self.pop();
                    let property = self.pop();

                    match (&object, &property) {
                        (Value::Object(obj), Value::String(key)) => {
                            let value = obj.get(key).cloned().unwrap_or(Value::Null);
                            self.push(value);
                        }
                        _ => {
                            return Err(RuntimeError::TypeError(format!(
                                "Cannot get property on {}",
                                object.type_name()
                            )));
                        }
                    }
                }

                Opcode::SET_PROPERTY => {
                    let value = self.pop();
                    let property = self.pop();
                    let object = self.pop();

                    if let (Value::Object(mut obj), Value::String(key)) = (object, property) {
                        obj.insert(key, value.clone());
                        self.push(Value::Object(obj));
                        self.push(value);
                    } else {
                        return Err(RuntimeError::TypeError(
                            "Cannot set property".to_string(),
                        ));
                    }
                }

                Opcode::GET_INDEX => {
                    let index = self.pop();
                    let object = self.pop();

                    match (&object, &index) {
                        (Value::Array(arr), Value::Integer(i)) => {
                            let idx = *i as usize;
                            if idx < arr.len() {
                                self.push(arr[idx].clone());
                            } else {
                                return Err(RuntimeError::IndexOutOfBounds(format!(
                                    "Index {} out of bounds for array of length {}",
                                    idx,
                                    arr.len()
                                )));
                            }
                        }
                        (Value::Object(obj), Value::String(key)) => {
                            let value = obj.get(key).cloned().unwrap_or(Value::Null);
                            self.push(value);
                        }
                        (Value::String(s), Value::Integer(i)) => {
                            let idx = *i as usize;
                            if let Some(ch) = s.chars().nth(idx) {
                                self.push(Value::String(ch.to_string()));
                            } else {
                                return Err(RuntimeError::IndexOutOfBounds(format!(
                                    "Index {} out of bounds for string of length {}",
                                    idx,
                                    s.len()
                                )));
                            }
                        }
                        _ => {
                            return Err(RuntimeError::TypeError(format!(
                                "Cannot index into {}",
                                object.type_name()
                            )));
                        }
                    }
                }

                Opcode::SET_INDEX => {
                    let value = self.pop();
                    let index = self.pop();
                    let object = self.pop();

                    if let (Value::Array(mut arr), Value::Integer(i)) = (object, index) {
                        let idx = *i as usize;
                        if idx < arr.len() {
                            arr[idx] = value.clone();
                            self.push(Value::Array(arr));
                            self.push(value);
                        } else {
                            return Err(RuntimeError::IndexOutOfBounds(format!(
                                "Index {} out of bounds",
                                idx
                            )));
                        }
                    } else {
                        return Err(RuntimeError::TypeError(
                            "Cannot set index".to_string(),
                        ));
                    }
                }

                Opcode::NEW_ARRAY => {
                    let count = instruction.operands[0] as usize;
                    let elements: Vec<Value> = self
                        .stack
                        .drain(self.stack.len() - count..)
                        .collect();
                    self.push(Value::Array(elements));
                }

                Opcode::NEW_OBJECT => {
                    self.push(Value::Object(HashMap::new()));
                }

                Opcode::CLASS => {
                    // Create a new class
                    self.push(Value::Object(HashMap::new()));
                }

                Opcode::METHOD => {
                    // Define a method on a class
                    let name = self.pop();
                    let method = self.pop();
                    let class = self.peek();

                    if let (Value::Object(mut obj), Value::String(key)) = (class.clone(), name) {
                        obj.insert(key, method);
                        *self.peek_mut() = Value::Object(obj);
                    }
                }

                Opcode::ASYNC => {
                    // Mark current function as async
                }

                Opcode::AWAIT => {
                    let value = self.pop();
                    // For now, just push the value
                    self.push(value);
                }

                _ => {
                    return Err(RuntimeError::Error(format!(
                        "Unknown opcode: {:?}",
                        opcode
                    )));
                }
            }
        }
    }

    fn constant_to_value(&self, constant: &Constant) -> Value {
        match constant {
            Constant::Integer(n) => Value::Integer(*n),
            Constant::Float(n) => Value::Float(*n),
            Constant::String(s) => Value::String(s.clone()),
            Constant::Boolean(b) => Value::Boolean(*b),
            Constant::Null => Value::Null,
            Constant::Function(f) => Value::Function(Arc::new(f.clone())),
        }
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().expect("Stack underflow")
    }

    fn peek(&self) -> &Value {
        self.stack.last().expect("Stack is empty")
    }

    fn peek_mut(&mut self) -> &mut Value {
        self.stack.last_mut().expect("Stack is empty")
    }

    fn current_frame(&self) -> &CallFrame {
        self.frames.last().expect("No call frame")
    }

    fn current_frame_mut(&mut self) -> &mut CallFrame {
        self.frames.last_mut().expect("No call frame")
    }

    fn push_frame(&mut self, function: Arc<CompiledFunction>, stack_base: usize) {
        let frame = CallFrame {
            function,
            ip: 0,
            stack_base,
        };
        self.frames.push(frame);
    }

    fn pop_frame(&mut self) -> Option<CallFrame> {
        self.frames.pop()
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::Compiler;
    use nexora_compiler::{Lexer, Parser};

    fn run_source(source: &str) -> Result<Value, RuntimeError> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program().unwrap();

        let mut compiler = Compiler::new();
        let module = compiler.compile(&program).map_err(|e| {
            RuntimeError::Error(e.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", "))
        })?;

        let mut vm = VM::new();
        vm.run(&module)
    }

    #[test]
    fn test_integer() {
        let result = run_source("42").unwrap();
        assert!(matches!(result, Value::Integer(42)));
    }

    #[test]
    fn test_string() {
        let result = run_source(r#""hello""#).unwrap();
        assert!(matches!(result, Value::String(s) if s == "hello"));
    }

    #[test]
    fn test_addition() {
        let result = run_source("1 + 2").unwrap();
        assert!(matches!(result, Value::Integer(3)));
    }

    #[test]
    fn test_subtraction() {
        let result = run_source("5 - 3").unwrap();
        assert!(matches!(result, Value::Integer(2)));
    }

    #[test]
    fn test_multiplication() {
        let result = run_source("4 * 5").unwrap();
        assert!(matches!(result, Value::Integer(20)));
    }

    #[test]
    fn test_division() {
        let result = run_source("10 / 2").unwrap();
        assert!(matches!(result, Value::Integer(5)));
    }

    #[test]
    fn test_variable() {
        let result = run_source("let x = 10; x").unwrap();
        assert!(matches!(result, Value::Integer(10)));
    }

    #[test]
    fn test_if_else() {
        let result = run_source("if true { 1 } else { 2 }").unwrap();
        assert!(matches!(result, Value::Integer(1)));
    }

    #[test]
    fn test_while() {
        let result = run_source("let i = 0; while i < 5 { i = i + 1 }; i").unwrap();
        assert!(matches!(result, Value::Integer(5)));
    }
}
