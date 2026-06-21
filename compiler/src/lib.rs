pub mod ast;
pub mod lexer;
pub mod parser;

pub use ast::{Expr, Program, Stmt};
pub use lexer::{Lexer, LexerError, SpannedToken, Token};
pub use parser::{ParseError, Parser};

/// Parse source code into an AST
pub fn parse(source: &str) -> Result<Program, ParseError> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    parser.parse_program()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hello_world() {
        let source = r#"print "Hello World""#;
        let program = parse(source).unwrap();
        assert_eq!(program.stmts.len(), 1);
    }

    #[test]
    fn test_parse_variables() {
        let source = r#"
let name = "Ashish"
let age = 20
        "#;
        let program = parse(source).unwrap();
        assert_eq!(program.stmts.len(), 2);
    }

    #[test]
    fn test_parse_if_else() {
        let source = r#"
let age = 20
if age >= 18 {
    print "Adult"
} else {
    print "Minor"
}
        "#;
        let program = parse(source).unwrap();
        assert_eq!(program.stmts.len(), 2);
    }

    #[test]
    fn test_parse_function() {
        let source = r#"
func greet(user) {
    print "Hello " + user
}
        "#;
        let program = parse(source).unwrap();
        assert_eq!(program.stmts.len(), 1);
    }
}
