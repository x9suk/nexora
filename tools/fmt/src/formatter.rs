pub struct Formatter {
    indent_size: usize,
}

impl Formatter {
    pub fn new() -> Self {
        Self { indent_size: 4 }
    }

    pub fn format(&self, source: &str) -> String {
        let tokens = self.tokenize(source);
        self.format_tokens(&tokens)
    }

    fn tokenize(&self, source: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let chars: Vec<char> = source.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            // Skip whitespace but preserve newlines for formatting
            if chars[i].is_whitespace() && chars[i] != '\n' {
                i += 1;
                continue;
            }

            // Newlines
            if chars[i] == '\n' {
                tokens.push(Token::Newline);
                i += 1;
                continue;
            }

            // Single-line comments
            if chars[i] == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
                let start = i;
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
                tokens.push(Token::Comment(source[start..i].to_string()));
                continue;
            }

            // Multi-line comments
            if chars[i] == '/' && i + 1 < chars.len() && chars[i + 1] == '*' {
                let start = i;
                i += 2;
                while i < chars.len() - 1 && !(chars[i] == '*' && chars[i + 1] == '/') {
                    i += 1;
                }
                i += 2;
                tokens.push(Token::Comment(source[start..i].to_string()));
                continue;
            }

            // Strings
            if chars[i] == '"' || chars[i] == '\'' {
                let quote = chars[i];
                let start = i;
                i += 1;
                while i < chars.len() && chars[i] != quote {
                    if chars[i] == '\\' {
                        i += 1;
                    }
                    i += 1;
                }
                i += 1;
                tokens.push(Token::String(source[start..i].to_string()));
                continue;
            }

            // Template literals with interpolation
            if chars[i] == '`' {
                let start = i;
                i += 1;
                while i < chars.len() && chars[i] != '`' {
                    if chars[i] == '\\' {
                        i += 1;
                    }
                    i += 1;
                }
                i += 1;
                tokens.push(Token::String(source[start..i].to_string()));
                continue;
            }

            // Numbers
            if chars[i].is_numeric() {
                let start = i;
                while i < chars.len() && (chars[i].is_numeric() || chars[i] == '.') {
                    i += 1;
                }
                tokens.push(Token::Number(source[start..i].to_string()));
                continue;
            }

            // Identifiers and keywords
            if chars[i].is_alphabetic() || chars[i] == '_' {
                let start = i;
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let word = &source[start..i];
                match word {
                    "fn" | "let" | "const" | "mut" | "if" | "else" | "while" | "for" |
                    "return" | "class" | "new" | "this" | "import" | "from" | "export" |
                    "match" | "case" | "default" | "break" | "continue" | "try" | "catch" |
                    "throw" | "async" | "await" | "true" | "false" | "null" | "undefined" |
                    "print" | "true" | "false" => {
                        tokens.push(Token::Keyword(word.to_string()));
                    }
                    _ => {
                        tokens.push(Token::Identifier(word.to_string()));
                    }
                }
                continue;
            }

            // Operators
            match chars[i] {
                '+' | '-' | '*' | '/' | '%' | '=' | '!' | '<' | '>' | '&' | '|' | '^' | '~' => {
                    let start = i;
                    i += 1;
                    // Handle multi-char operators
                    if i < chars.len() && chars[i] == '=' {
                        i += 1;
                    }
                    tokens.push(Token::Operator(source[start..i].to_string()));
                }
                '(' | ')' | '[' | ']' | '{' | '}' => {
                    tokens.push(Token::Bracket(chars[i].to_string()));
                    i += 1;
                }
                ';' | ':' | ',' | '.' | '?' => {
                    tokens.push(Token::Punctuation(chars[i].to_string()));
                    i += 1;
                }
                _ => {
                    tokens.push(Token::Other(chars[i].to_string()));
                    i += 1;
                }
            }
        }

        tokens
    }

    fn format_tokens(&self, tokens: &[Token]) -> String {
        let mut output = String::new();
        let mut indent_level = 0;
        let mut i = 0;
        let mut prev_was_newline = false;
        let mut after_keyword = false;

        while i < tokens.len() {
            match &tokens[i] {
                Token::Newline => {
                    // Collapse multiple newlines into one
                    if !prev_was_newline {
                        output.push('\n');
                        prev_was_newline = true;
                    }
                }
                Token::Comment(text) => {
                    output.push_str(&text);
                    prev_was_newline = false;
                }
                Token::Keyword(kw) => {
                    if prev_was_newline {
                        output.push_str(&" ".repeat(indent_level * self.indent_size));
                    }
                    output.push_str(kw);
                    after_keyword = matches!(kw.as_str(), "fn" | "class" | "if" | "else" | "while" | "for" | "match" | "try" | "catch");
                    prev_was_newline = false;
                }
                Token::Bracket(b) => {
                    match b.as_str() {
                        "{" => {
                            // Same line brace style
                            output.push_str(" {");
                            indent_level += 1;
                            output.push('\n');
                            prev_was_newline = true;
                        }
                        "}" => {
                            indent_level = indent_level.saturating_sub(1);
                            if !prev_was_newline {
                                output.push('\n');
                            }
                            output.push_str(&" ".repeat(indent_level * self.indent_size));
                            output.push('}');
                            prev_was_newline = false;
                        }
                        "(" | "[" => {
                            output.push_str(&format!("{} ", b));
                            prev_was_newline = false;
                        }
                        ")" | "]" => {
                            output.push_str(&format!(" {}", b));
                            prev_was_newline = false;
                        }
                        _ => {
                            output.push_str(b);
                            prev_was_newline = false;
                        }
                    }
                }
                Token::Operator(op) => {
                    match op.as_str() {
                        "=" | "==" | "!=" | "<=" | ">=" | "&&" | "||" | "=>" => {
                            output.push_str(&format!(" {} ", op));
                        }
                        "+" | "-" | "*" | "/" | "%" => {
                            output.push_str(&format!(" {} ", op));
                        }
                        _ => {
                            output.push_str(op);
                        }
                    }
                    prev_was_newline = false;
                }
                Token::Punctuation(p) => {
                    match p.as_str() {
                        ";" => {
                            output.push(';');
                            if !after_keyword {
                                output.push('\n');
                                prev_was_newline = true;
                            }
                        }
                        "," => {
                            output.push_str(", ");
                        }
                        ":" => {
                            output.push_str(": ");
                        }
                        "?" | "." => {
                            output.push_str(p);
                        }
                        _ => {
                            output.push_str(p);
                        }
                    }
                    after_keyword = false;
                    prev_was_newline = false;
                }
                Token::String(s) | Token::Number(s) | Token::Identifier(s) | Token::Other(s) => {
                    if prev_was_newline {
                        output.push_str(&" ".repeat(indent_level * self.indent_size));
                    }
                    output.push_str(s);
                    prev_was_newline = false;
                }
            }

            i += 1;
        }

        // Ensure file ends with newline
        if !output.ends_with('\n') {
            output.push('\n');
        }

        output
    }
}

enum Token {
    Keyword(String),
    Identifier(String),
    String(String),
    Number(String),
    Operator(String),
    Bracket(String),
    Punctuation(String),
    Comment(String),
    Newline,
    Other(String),
}
