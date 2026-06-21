pub mod interpreter;
pub mod value;

pub use interpreter::Interpreter;
pub use value::{Environment, RuntimeError, Value};

/// Run Nexora source code
pub fn run(source: &str) -> Result<Value, RuntimeError> {
    let program = nexora_compiler::parse(source)
        .map_err(|e| RuntimeError::Error(e.to_string()))?;

    let mut interpreter = Interpreter::new();
    interpreter.interpret(&program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        let result = run(r#"print "Hello World""#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_variables() {
        let result = run(r#"
let x = 10
let y = 20
print x + y
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_arithmetic() {
        let result = run("print 2 + 3 * 4");
        assert!(result.is_ok());
    }

    #[test]
    fn test_if_else() {
        let result = run(r#"
let age = 20
if age >= 18 {
    print "Adult"
} else {
    print "Minor"
}
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_function() {
        let result = run(r#"
func add(a, b) {
    return a + b
}
print add(5, 3)
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_loop() {
        let result = run(r#"
for i in [1, 2, 3] {
    print i
}
        "#);
        assert!(result.is_ok());
    }
}
