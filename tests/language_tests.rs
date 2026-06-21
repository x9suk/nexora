use nexora::lexer::Lexer;
use nexora::parser::Parser;
use nexora::interpreter::Interpreter;

fn run(source: &str) -> Result<(), nexora::error::NexoraError> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    let mut interp = Interpreter::new();
    interp.run(&stmts)
}

#[test]
fn test_hello_world() {
    assert!(run(r#"print "Hello World""#).is_ok());
}

#[test]
fn test_arithmetic() {
    assert!(run("print 2 + 3").is_ok());
    assert!(run("print 10 - 5").is_ok());
    assert!(run("print 4 * 5").is_ok());
    assert!(run("print 10 / 2").is_ok());
    assert!(run("print 10 % 3").is_ok());
}

#[test]
fn test_variables() {
    assert!(run("let x = 10").is_ok());
    assert!(run("let name = \"test\"").is_ok());
    assert!(run("let flag = true").is_ok());
    assert!(run("let nothing = null").is_ok());
}

#[test]
fn test_reassignment() {
    assert!(run("let x = 10; x = 20; print x").is_ok());
}

#[test]
fn test_if_else() {
    assert!(run("if true { print 1 }").is_ok());
    assert!(run("if false { print 1 } else { print 2 }").is_ok());
    assert!(run("if 1 > 0 { print \"positive\" }").is_ok());
}

#[test]
fn test_while_loop() {
    assert!(run("let i = 0; while i < 5 { i = i + 1 }").is_ok());
}

#[test]
fn test_for_loop() {
    assert!(run("for i in [1, 2, 3] { print i }").is_ok());
}

#[test]
fn test_functions() {
    assert!(run("func add(a, b) { return a + b }; print add(1, 2)").is_ok());
    assert!(run("func greet() { print \"hi\" }; greet()").is_ok());
}

#[test]
fn test_recursive_function() {
    assert!(run("func fact(n) { if n <= 1 { return 1 }; return n * fact(n - 1) }; print fact(5)").is_ok());
}

#[test]
fn test_arrays() {
    assert!(run("let arr = [1, 2, 3]; print arr[0]").is_ok());
    assert!(run("let arr = [1, 2, 3]; print arr.length").is_ok());
}

#[test]
fn test_objects() {
    assert!(run(r#"let obj = { name: "test" }; print obj.name"#).is_ok());
}

#[test]
fn test_string_operations() {
    assert!(run(r#"print "hello" + " " + "world""#).is_ok());
    assert!(run(r#"print "test".length"#).is_ok());
}

#[test]
fn test_comparison() {
    assert!(run("print 1 == 1").is_ok());
    assert!(run("print 1 != 2").is_ok());
    assert!(run("print 1 < 2").is_ok());
    assert!(run("print 2 > 1").is_ok());
    assert!(run("print 1 <= 1").is_ok());
    assert!(run("print 1 >= 1").is_ok());
}

#[test]
fn test_logical_operators() {
    assert!(run("print true && false").is_ok());
    assert!(run("print true || false").is_ok());
    assert!(run("print !true").is_ok());
}

#[test]
fn test_builtin_str() {
    assert!(run("print str(42)").is_ok());
    assert!(run("print str(true)").is_ok());
}

#[test]
fn test_builtin_len() {
    assert!(run("print len([1, 2, 3])").is_ok());
    assert!(run(r#"print len("hello")"#).is_ok());
}

#[test]
fn test_comment_handling() {
    assert!(run("// this is a comment\nprint 42").is_ok());
    assert!(run("// comment 1\n// comment 2\nprint 42").is_ok());
    assert!(run("// comment\n// comment\n// comment\nprint 42").is_ok());
}
