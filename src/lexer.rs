#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Null,

    // Identifier
    Ident(String),

    // Keywords
    Let,
    Func,
    Return,
    If,
    Else,
    While,
    For,
    In,
    Break,
    Continue,
    Print,
    Import,
    Class,
    New,
    This,
    Extends,
    Super,
    Try,
    Catch,
    Finally,
    Throw,
    Async,
    Await,
    Assert,
    Test,
    Match,
    Type,
    Interface,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    And,
    Or,
    Not,
    Assign,

    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Semicolon,
    Comma,
    Colon,
    Dot,
    Arrow,

    // Special
    Eof,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    fn peek(&self) -> char {
        self.input.get(self.pos).copied().unwrap_or('\0')
    }

    fn advance(&mut self) -> char {
        let ch = self.peek();
        self.pos += 1;
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        ch
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_whitespace() || self.peek() == '\r' {
            self.advance();
        }
    }

    fn skip_comment(&mut self) {
        if self.peek() == '/' && self.pos + 1 < self.input.len() {
            let next = self.input[self.pos + 1];
            if next == '/' {
                while self.peek() != '\n' && self.peek() != '\0' {
                    self.advance();
                }
                // Consume the newline at end of comment
                if self.peek() == '\n' {
                    self.advance();
                }
                return;
            }
        }
    }

    fn read_string(&mut self) -> String {
        let mut s = String::new();
        self.advance(); // skip opening quote
        while self.peek() != '"' && self.peek() != '\0' {
            if self.peek() == '\\' {
                self.advance();
                match self.peek() {
                    'n' => s.push('\n'),
                    't' => s.push('\t'),
                    '\\' => s.push('\\'),
                    '"' => s.push('"'),
                    c => {
                        s.push('\\');
                        s.push(c);
                    }
                }
            } else {
                s.push(self.peek());
            }
            self.advance();
        }
        self.advance(); // skip closing quote
        s
    }

    fn read_number(&mut self) -> Token {
        let mut num = String::new();
        let mut is_float = false;

        while self.peek().is_ascii_digit() {
            num.push(self.peek());
            self.advance();
        }

        if self.peek() == '.' && self.pos + 1 < self.input.len() && self.input[self.pos + 1].is_ascii_digit() {
            is_float = true;
            num.push('.');
            self.advance();
            while self.peek().is_ascii_digit() {
                num.push(self.peek());
                self.advance();
            }
        }

        if is_float {
            Token::Float(num.parse().unwrap())
        } else {
            Token::Integer(num.parse().unwrap())
        }
    }

    fn read_ident(&mut self) -> Token {
        let mut ident = String::new();
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            ident.push(self.peek());
            self.advance();
        }

        match ident.as_str() {
            "let" => Token::Let,
            "func" => Token::Func,
            "return" => Token::Return,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "in" => Token::In,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "print" => Token::Print,
            "import" => Token::Import,
            "class" => Token::Class,
            "new" => Token::New,
            "this" => Token::This,
            "try" => Token::Try,
            "catch" => Token::Catch,
            "finally" => Token::Finally,
            "throw" => Token::Throw,
            "async" => Token::Async,
            "await" => Token::Await,
            "assert" => Token::Assert,
            "test" => Token::Test,
            "extends" => Token::Extends,
            "super" => Token::Super,
            "match" => Token::Match,
            "type" => Token::Type,
            "interface" => Token::Interface,
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            "null" => Token::Null,
            _ => Token::Ident(ident),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            // Skip all whitespace and comments
            loop {
                let before = self.pos;
                self.skip_whitespace();
                self.skip_comment();
                if self.pos == before {
                    break;
                }
            }

            let ch = self.peek();
            if ch == '\0' {
                tokens.push(Token::Eof);
                break;
            }

            let token = match ch {
                '"' => Token::String(self.read_string()),
                '0'..='9' => self.read_number(),
                'a'..='z' | 'A'..='Z' | '_' => self.read_ident(),
                '+' => { self.advance(); Token::Plus }
                '-' => {
                    self.advance();
                    if self.peek() == '>' {
                        self.advance();
                        Token::Arrow
                    } else {
                        Token::Minus
                    }
                }
                '*' => { self.advance(); Token::Star }
                '/' => {
                    self.advance();
                    Token::Slash
                }
                '%' => { self.advance(); Token::Percent }
                '=' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        Token::Eq
                    } else if self.peek() == '>' {
                        self.advance();
                        Token::Arrow
                    } else {
                        Token::Assign
                    }
                }
                '!' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        Token::NotEq
                    } else {
                        Token::Not
                    }
                }
                '<' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        Token::LtEq
                    } else {
                        Token::Lt
                    }
                }
                '>' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        Token::GtEq
                    } else {
                        Token::Gt
                    }
                }
                '&' => {
                    self.advance();
                    if self.peek() == '&' {
                        self.advance();
                        Token::And
                    } else {
                        panic!("Unexpected character '&' at line {}, col {}", self.line, self.col);
                    }
                }
                '|' => {
                    self.advance();
                    if self.peek() == '|' {
                        self.advance();
                        Token::Or
                    } else {
                        panic!("Unexpected character '|' at line {}, col {}", self.line, self.col);
                    }
                }
                '(' => { self.advance(); Token::LParen }
                ')' => { self.advance(); Token::RParen }
                '{' => { self.advance(); Token::LBrace }
                '}' => { self.advance(); Token::RBrace }
                '[' => { self.advance(); Token::LBracket }
                ']' => { self.advance(); Token::RBracket }
                ';' => { self.advance(); Token::Semicolon }
                ',' => { self.advance(); Token::Comma }
                ':' => { self.advance(); Token::Colon }
                '.' => { self.advance(); Token::Dot }
                _ => {
                    panic!("Unexpected character '{}' at line {}, col {}", ch, self.line, self.col);
                }
            };

            tokens.push(token);
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let mut lexer = Lexer::new("let x = 42");
        let tokens = lexer.tokenize();
        assert_eq!(tokens, vec![
            Token::Let,
            Token::Ident("x".to_string()),
            Token::Assign,
            Token::Integer(42),
            Token::Eof,
        ]);
    }

    #[test]
    fn test_string() {
        let mut lexer = Lexer::new(r#""hello""#);
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0], Token::String("hello".to_string()));
    }

    #[test]
    fn test_operators() {
        let mut lexer = Lexer::new("1 + 2 * 3");
        let tokens = lexer.tokenize();
        assert_eq!(tokens, vec![
            Token::Integer(1),
            Token::Plus,
            Token::Integer(2),
            Token::Star,
            Token::Integer(3),
            Token::Eof,
        ]);
    }

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::new("if else while func return");
        let tokens = lexer.tokenize();
        assert_eq!(tokens, vec![
            Token::If,
            Token::Else,
            Token::While,
            Token::Func,
            Token::Return,
            Token::Eof,
        ]);
    }

    #[test]
    fn test_comparison() {
        let mut lexer = Lexer::new("== != < > <= >=");
        let tokens = lexer.tokenize();
        assert_eq!(tokens, vec![
            Token::Eq,
            Token::NotEq,
            Token::Lt,
            Token::Gt,
            Token::LtEq,
            Token::GtEq,
            Token::Eof,
        ]);
    }

    #[test]
    fn test_delimiters() {
        let mut lexer = Lexer::new("(){}[];,:");
        let tokens = lexer.tokenize();
        assert_eq!(tokens, vec![
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::LBracket,
            Token::RBracket,
            Token::Semicolon,
            Token::Comma,
            Token::Colon,
            Token::Eof,
        ]);
    }
}
