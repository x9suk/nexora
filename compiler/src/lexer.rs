use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Token {
    // Literals
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,

    // Identifiers and keywords
    Identifier(String),
    Let,
    Const,
    Func,
    Return,
    If,
    Elif,
    Else,
    While,
    For,
    In,
    Break,
    Continue,
    True,
    False,
    Import,
    From,
    As,
    Class,
    New,
    This,
    Self_,
    Async,
    Await,
    Try,
    Catch,
    Finally,
    Throw,
    Module,
    Export,
    Default,
    Type,
    Interface,
    Enum,
    Match,
    When,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Power,
    Assign,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Not,
    Ampersand,
    Pipe,
    Xor,
    ShiftLeft,
    ShiftRight,
    Increment,
    Decrement,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    ModuloAssign,
    PowerAssign,
    Arrow,
    FatArrow,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Colon,
    Comma,
    Dot,
    Question,
    DoubleColon,
    Hash,
    At,

    // Special
    Newline,
    EOF,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Integer(n) => write!(f, "{}", n),
            Token::Float(n) => write!(f, "{}", n),
            Token::String(s) => write!(f, "\"{}\"", s),
            Token::Boolean(b) => write!(f, "{}", b),
            Token::Null => write!(f, "null"),
            Token::Identifier(s) => write!(f, "{}", s),
            Token::Let => write!(f, "let"),
            Token::Const => write!(f, "const"),
            Token::Func => write!(f, "func"),
            Token::Return => write!(f, "return"),
            Token::If => write!(f, "if"),
            Token::Elif => write!(f, "elif"),
            Token::Else => write!(f, "else"),
            Token::While => write!(f, "while"),
            Token::For => write!(f, "for"),
            Token::In => write!(f, "in"),
            Token::Break => write!(f, "break"),
            Token::Continue => write!(f, "continue"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::Import => write!(f, "import"),
            Token::From => write!(f, "from"),
            Token::As => write!(f, "as"),
            Token::Class => write!(f, "class"),
            Token::New => write!(f, "new"),
            Token::This => write!(f, "this"),
            Token::Self_ => write!(f, "self"),
            Token::Async => write!(f, "async"),
            Token::Await => write!(f, "await"),
            Token::Try => write!(f, "try"),
            Token::Catch => write!(f, "catch"),
            Token::Finally => write!(f, "finally"),
            Token::Throw => write!(f, "throw"),
            Token::Module => write!(f, "module"),
            Token::Export => write!(f, "export"),
            Token::Default => write!(f, "default"),
            Token::Type => write!(f, "type"),
            Token::Interface => write!(f, "interface"),
            Token::Enum => write!(f, "enum"),
            Token::Match => write!(f, "match"),
            Token::When => write!(f, "when"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Percent => write!(f, "%"),
            Token::Power => write!(f, "**"),
            Token::Assign => write!(f, "="),
            Token::Equal => write!(f, "=="),
            Token::NotEqual => write!(f, "!="),
            Token::Less => write!(f, "<"),
            Token::Greater => write!(f, ">"),
            Token::LessEqual => write!(f, "<="),
            Token::GreaterEqual => write!(f, ">="),
            Token::And => write!(f, "&&"),
            Token::Or => write!(f, "||"),
            Token::Not => write!(f, "!"),
            Token::Ampersand => write!(f, "&"),
            Token::Pipe => write!(f, "|"),
            Token::Xor => write!(f, "^"),
            Token::ShiftLeft => write!(f, "<<"),
            Token::ShiftRight => write!(f, ">>"),
            Token::Increment => write!(f, "++"),
            Token::Decrement => write!(f, "--"),
            Token::PlusAssign => write!(f, "+="),
            Token::MinusAssign => write!(f, "-="),
            Token::StarAssign => write!(f, "*="),
            Token::SlashAssign => write!(f, "/="),
            Token::ModuloAssign => write!(f, "%="),
            Token::PowerAssign => write!(f, "**="),
            Token::Arrow => write!(f, "->"),
            Token::FatArrow => write!(f, "=>"),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::LeftBracket => write!(f, "["),
            Token::RightBracket => write!(f, "]"),
            Token::Semicolon => write!(f, ";"),
            Token::Colon => write!(f, ":"),
            Token::Comma => write!(f, ","),
            Token::Dot => write!(f, "."),
            Token::Question => write!(f, "?"),
            Token::DoubleColon => write!(f, "::"),
            Token::Hash => write!(f, "#"),
            Token::At => write!(f, "@"),
            Token::Newline => write!(f, "\\n"),
            Token::EOF => write!(f, "EOF"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpannedToken {
    pub token: Token,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, thiserror::Error)]
pub enum LexerError {
    #[error("Unexpected character '{0}' at line {1}, column {2}")]
    UnexpectedChar(char, usize, usize),
    #[error("Unterminated string at line {0}, column {1}")]
    UnterminatedString(usize, usize),
    #[error("Invalid number at line {0}, column {1}")]
    InvalidNumber(usize, usize),
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.input.get(self.position).copied()?;
        self.position += 1;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(ch)
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() && ch != '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) -> bool {
        if let Some(ch) = self.peek() {
            if ch == '/' {
                if let Some(next) = self.peek_next() {
                    if next == '/' {
                        // Single line comment
                        while let Some(ch) = self.peek() {
                            if ch == '\n' {
                                break;
                            }
                            self.advance();
                        }
                        return true;
                    } else if next == '*' {
                        // Multi-line comment
                        self.advance(); // /
                        self.advance(); // *
                        while let Some(ch) = self.advance() {
                            if ch == '*' {
                                if let Some(next) = self.peek() {
                                    if next == '/' {
                                        self.advance(); // /
                                        return true;
                                    }
                                }
                            }
                        }
                        return true;
                    }
                }
            }
        }
        false
    }

    fn read_string(&mut self, quote: char) -> Result<String, LexerError> {
        let mut string = String::new();
        let start_line = self.line;
        let start_col = self.column;

        self.advance(); // opening quote

        while let Some(ch) = self.advance() {
            if ch == quote {
                return Ok(string);
            }
            if ch == '\\' {
                match self.advance() {
                    Some('n') => string.push('\n'),
                    Some('t') => string.push('\t'),
                    Some('r') => string.push('\r'),
                    Some('\\') => string.push('\\'),
                    Some(c) if c == quote => string.push(c),
                    Some(c) => {
                        string.push('\\');
                        string.push(c);
                    }
                    None => return Err(LexerError::UnterminatedString(start_line, start_col)),
                }
            } else {
                string.push(ch);
            }
        }

        Err(LexerError::UnterminatedString(start_line, start_col))
    }

    fn read_number(&mut self) -> Result<Token, LexerError> {
        let mut number = String::new();
        let mut is_float = false;
        let start_line = self.line;
        let start_col = self.column;

        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                number.push(ch);
                self.advance();
            } else if ch == '.' && !is_float {
                is_float = true;
                number.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        if is_float {
            number
                .parse::<f64>()
                .map(Token::Float)
                .map_err(|_| LexerError::InvalidNumber(start_line, start_col))
        } else {
            number
                .parse::<i64>()
                .map(Token::Integer)
                .map_err(|_| LexerError::InvalidNumber(start_line, start_col))
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        identifier
    }

    fn keyword_or_identifier(ident: &str) -> Token {
        match ident {
            "let" => Token::Let,
            "const" => Token::Const,
            "func" => Token::Func,
            "return" => Token::Return,
            "if" => Token::If,
            "elif" => Token::Elif,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "in" => Token::In,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "true" => Token::True,
            "false" => Token::False,
            "null" => Token::Null,
            "import" => Token::Import,
            "from" => Token::From,
            "as" => Token::As,
            "class" => Token::Class,
            "new" => Token::New,
            "this" => Token::This,
            "self" => Token::Self_,
            "async" => Token::Async,
            "await" => Token::Await,
            "try" => Token::Try,
            "catch" => Token::Catch,
            "finally" => Token::Finally,
            "throw" => Token::Throw,
            "module" => Token::Module,
            "export" => Token::Export,
            "default" => Token::Default,
            "type" => Token::Type,
            "interface" => Token::Interface,
            "enum" => Token::Enum,
            "match" => Token::Match,
            "when" => Token::When,
            _ => Token::Identifier(ident.to_string()),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<SpannedToken>, LexerError> {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace();

            // Skip comments
            while self.skip_comment() {
                self.skip_whitespace();
            }

            let line = self.line;
            let column = self.column;

            let token = match self.peek() {
                None => {
                    tokens.push(SpannedToken {
                        token: Token::EOF,
                        line,
                        column,
                    });
                    break;
                }
                Some(ch) => {
                    self.advance();
                    match ch {
                        '\n' => Token::Newline,
                        '+' => {
                            if self.peek() == Some('+') {
                                self.advance();
                                Token::Increment
                            } else if self.peek() == Some('=') {
                                self.advance();
                                Token::PlusAssign
                            } else {
                                Token::Plus
                            }
                        }
                        '-' => {
                            if self.peek() == Some('>') {
                                self.advance();
                                Token::Arrow
                            } else if self.peek() == Some('-') {
                                self.advance();
                                Token::Decrement
                            } else if self.peek() == Some('=') {
                                self.advance();
                                Token::MinusAssign
                            } else {
                                Token::Minus
                            }
                        }
                        '*' => {
                            if self.peek() == Some('*') {
                                self.advance();
                                if self.peek() == Some('=') {
                                    self.advance();
                                    Token::PowerAssign
                                } else {
                                    Token::Power
                                }
                            } else if self.peek() == Some('=') {
                                self.advance();
                                Token::StarAssign
                            } else {
                                Token::Star
                            }
                        }
                        '/' => {
                            if self.peek() == Some('=') {
                                self.advance();
                                Token::SlashAssign
                            } else {
                                Token::Slash
                            }
                        }
                        '%' => {
                            if self.peek() == Some('=') {
                                self.advance();
                                Token::ModuloAssign
                            } else {
                                Token::Percent
                            }
                        }
                        '=' => {
                            if self.peek() == Some('=') {
                                self.advance();
                                Token::Equal
                            } else if self.peek() == Some('>') {
                                self.advance();
                                Token::FatArrow
                            } else {
                                Token::Assign
                            }
                        }
                        '!' => {
                            if self.peek() == Some('=') {
                                self.advance();
                                Token::NotEqual
                            } else {
                                Token::Not
                            }
                        }
                        '<' => {
                            if self.peek() == Some('=') {
                                self.advance();
                                Token::LessEqual
                            } else if self.peek() == Some('<') {
                                self.advance();
                                Token::ShiftLeft
                            } else {
                                Token::Less
                            }
                        }
                        '>' => {
                            if self.peek() == Some('=') {
                                self.advance();
                                Token::GreaterEqual
                            } else if self.peek() == Some('>') {
                                self.advance();
                                Token::ShiftRight
                            } else {
                                Token::Greater
                            }
                        }
                        '&' => {
                            if self.peek() == Some('&') {
                                self.advance();
                                Token::And
                            } else {
                                Token::Ampersand
                            }
                        }
                        '|' => {
                            if self.peek() == Some('|') {
                                self.advance();
                                Token::Or
                            } else {
                                Token::Pipe
                            }
                        }
                        '^' => Token::Xor,
                        '(' => Token::LeftParen,
                        ')' => Token::RightParen,
                        '{' => Token::LeftBrace,
                        '}' => Token::RightBrace,
                        '[' => Token::LeftBracket,
                        ']' => Token::RightBracket,
                        ';' => Token::Semicolon,
                        ':' => {
                            if self.peek() == Some(':') {
                                self.advance();
                                Token::DoubleColon
                            } else {
                                Token::Colon
                            }
                        }
                        ',' => Token::Comma,
                        '.' => Token::Dot,
                        '?' => Token::Question,
                        '#' => Token::Hash,
                        '@' => Token::At,
                        '"' | '\'' => {
                            let string = self.read_string(ch)?;
                            Token::String(string)
                        }
                        c if c.is_ascii_digit() => self.read_number()?,
                        c if c.is_alphabetic() || c == '_' => {
                            let mut ident = String::new();
                            ident.push(c);
                            while let Some(ch) = self.peek() {
                                if ch.is_alphanumeric() || ch == '_' {
                                    ident.push(ch);
                                    self.advance();
                                } else {
                                    break;
                                }
                            }
                            Self::keyword_or_identifier(&ident)
                        }
                        _ => return Err(LexerError::UnexpectedChar(ch, line, column)),
                    }
                }
            };

            tokens.push(SpannedToken {
                token,
                line,
                column,
            });
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let mut lexer = Lexer::new("let x = 42");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 5); // let, x, =, 42, EOF
    }

    #[test]
    fn test_string() {
        let mut lexer = Lexer::new(r#""hello world""#);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token, Token::String("hello world".to_string()));
    }

    #[test]
    fn test_operators() {
        let mut lexer = Lexer::new("a + b * c");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[1].token, Token::Plus);
        assert_eq!(tokens[3].token, Token::Star);
    }

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::new("if else while func");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token, Token::If);
        assert_eq!(tokens[1].token, Token::Else);
        assert_eq!(tokens[2].token, Token::While);
        assert_eq!(tokens[3].token, Token::Func);
    }
}
