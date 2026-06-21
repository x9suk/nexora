use crate::ast::*;
use crate::lexer::{LexerError, SpannedToken, Token};
use std::iter::Peekable;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Unexpected token {0} at line {1}, column {2}")]
    UnexpectedToken(String, usize, usize),
    #[error("Expected {0} at line {1}, column {2}")]
    Expected(String, usize, usize),
    #[error("Unexpected end of file")]
    UnexpectedEOF,
    #[error("Lexer error: {0}")]
    Lexer(#[from] LexerError),
}

pub struct Parser {
    tokens: Vec<SpannedToken>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<SpannedToken>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn peek(&self) -> &SpannedToken {
        self.tokens
            .get(self.position)
            .unwrap_or(&SpannedToken {
                token: Token::EOF,
                line: 0,
                column: 0,
            })
    }

    fn advance(&mut self) -> &SpannedToken {
        let token = self.tokens.get(self.position).unwrap();
        if self.position < self.tokens.len() - 1 {
            self.position += 1;
        }
        token
    }

    fn expect(&mut self, expected: &Token) -> Result<&SpannedToken, ParseError> {
        let token = self.peek();
        if std::mem::discriminant(&token.token) == std::mem::discriminant(expected) {
            Ok(self.advance())
        } else {
            Err(ParseError::Expected(
                expected.to_string(),
                token.line,
                token.column,
            ))
        }
    }

    fn match_token(&mut self, token: &Token) -> bool {
        if std::mem::discriminant(&self.peek().token) == std::mem::discriminant(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn skip_newlines(&mut self) {
        while matches!(self.peek().token, Token::Newline) {
            self.advance();
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut stmts = Vec::new();

        self.skip_newlines();

        while !matches!(self.peek().token, Token::EOF) {
            stmts.push(self.parse_stmt()?);
            self.skip_newlines();
        }

        Ok(Program { stmts })
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        match &self.peek().token {
            Token::Let | Token::Const => self.parse_var_decl(),
            Token::Func => self.parse_func_decl(),
            Token::Return => self.parse_return(),
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::For => self.parse_for(),
            Token::Break => {
                self.advance();
                Ok(Stmt::Break)
            }
            Token::Continue => {
                self.advance();
                Ok(Stmt::Continue)
            }
            Token::Import => self.parse_import(),
            Token::From => self.parse_import_from(),
            Token::Class => self.parse_class_decl(),
            Token::Try => self.parse_try_catch(),
            Token::Throw => {
                self.advance();
                let expr = self.parse_expr()?;
                Ok(Stmt::Throw(expr))
            }
            Token::Module => self.parse_module(),
            Token::Export => self.parse_export(),
            Token::Type => self.parse_type_decl(),
            Token::Interface => self.parse_interface_decl(),
            Token::Enum => self.parse_enum_decl(),
            Token::LeftBrace => {
                let block = self.parse_block()?;
                Ok(Stmt::Block(block))
            }
            _ => {
                let expr = self.parse_expr()?;
                Ok(Stmt::Expr(expr))
            }
        }
    }

    fn parse_var_decl(&mut self) -> Result<Stmt, ParseError> {
        let is_const = matches!(self.peek().token, Token::Const);
        self.advance(); // let or const

        let name = match &self.peek().token {
            Token::Identifier(name) => name.clone(),
            _ => return Err(ParseError::Expected("identifier".into(), self.peek().line, self.peek().column)),
        };
        self.advance();

        let type_annotation = if self.match_token(&Token::Colon) {
            Some(self.parse_type_name()?)
        } else {
            None
        };

        let value = if self.match_token(&Token::Assign) {
            self.parse_expr()?
        } else if is_const {
            return Err(ParseError::Expected("initializer for const".into(), self.peek().line, self.peek().column));
        } else {
            Expr::Null
        };

        Ok(Stmt::VarDecl {
            name,
            type_annotation,
            value,
            is_const,
        })
    }

    fn parse_type_name(&mut self) -> Result<String, ParseError> {
        let mut name = String::new();
        while matches!(self.peek().token, Token::Identifier(_)) {
            if !name.is_empty() {
                name.push('.');
            }
            if let Token::Identifier(n) = &self.peek().token {
                name.push_str(n);
            }
            self.advance();
        }
        if self.match_token(&Token::Question) {
            name.push('?');
        }
        Ok(name)
    }

    fn parse_func_decl(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // func

        let is_async = false; // Handle async separately if needed

        let name = match &self.peek().token {
            Token::Identifier(name) => name.clone(),
            _ => return Err(ParseError::Expected("function name".into(), self.peek().line, self.peek().column)),
        };
        self.advance();

        self.expect(&Token::LeftParen)?;
        let params = self.parse_params()?;
        self.expect(&Token::RightParen)?;

        let return_type = if self.match_token(&Token::Arrow) || self.match_token(&Token::Colon) {
            Some(self.parse_type_name()?)
        } else {
            None
        };

        let body = self.parse_block()?;

        Ok(Stmt::FuncDecl {
            name,
            params,
            return_type,
            body,
            is_async,
        })
    }

    fn parse_params(&mut self) -> Result<Vec<Param>, ParseError> {
        let mut params = Vec::new();

        if matches!(self.peek().token, Token::RightParen) {
            return Ok(params);
        }

        loop {
            let name = match &self.peek().token {
                Token::Identifier(name) => name.clone(),
                _ => return Err(ParseError::Expected("parameter name".into(), self.peek().line, self.peek().column)),
            };
            self.advance();

            let type_annotation = if self.match_token(&Token::Colon) {
                Some(self.parse_type_name()?)
            } else {
                None
            };

            let default = if self.match_token(&Token::Assign) {
                Some(self.parse_expr()?)
            } else {
                None
            };

            params.push(Param {
                name,
                type_annotation,
                default,
            });

            if !self.match_token(&Token::Comma) {
                break;
            }
        }

        Ok(params)
    }

    fn parse_block(&mut self) -> Result<Block, ParseError> {
        self.expect(&Token::LeftBrace)?;
        self.skip_newlines();

        let mut stmts = Vec::new();

        while !matches!(self.peek().token, Token::RightBrace | Token::EOF) {
            stmts.push(self.parse_stmt()?);
            self.skip_newlines();
        }

        self.expect(&Token::RightBrace)?;

        Ok(Block { stmts })
    }

    fn parse_return(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // return

        let value = if matches!(self.peek().token, Token::Newline | Token::Semicolon | Token::RightBrace | Token::EOF) {
            None
        } else {
            Some(self.parse_expr()?)
        };

        Ok(Stmt::Return(value))
    }

    fn parse_if(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // if
        let condition = self.parse_expr()?;
        let then_body = self.parse_block()?;

        let mut elif_clauses = Vec::new();

        while self.match_token(&Token::Elif) {
            let elif_condition = self.parse_expr()?;
            let elif_body = self.parse_block()?;
            elif_clauses.push((elif_condition, elif_body));
        }

        let else_body = if self.match_token(&Token::Else) {
            Some(self.parse_block()?)
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_body,
            elif_clauses,
            else_body,
        })
    }

    fn parse_while(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // while
        let condition = self.parse_expr()?;
        let body = self.parse_block()?;
        Ok(Stmt::While { condition, body })
    }

    fn parse_for(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // for

        let variable = match &self.peek().token {
            Token::Identifier(name) => name.clone(),
            _ => return Err(ParseError::Expected("loop variable".into(), self.peek().line, self.peek().column)),
        };
        self.advance();

        self.expect(&Token::In)?;
        let iterable = self.parse_expr()?;
        let body = self.parse_block()?;

        Ok(Stmt::For {
            variable,
            iterable,
            body,
        })
    }

    fn parse_import(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // import

        let module = match &self.peek().token {
            Token::String(s) => s.clone(),
            Token::Identifier(name) => {
                let mut path = name.clone();
                self.advance();
                while self.match_token(&Token::Dot) {
                    if let Token::Identifier(n) = &self.peek().token {
                        path.push('.');
                        path.push_str(n);
                        self.advance();
                    }
                }
                path
            }
            _ => return Err(ParseError::Expected("module name".into(), self.peek().line, self.peek().column)),
        };
        if matches!(self.peek().token, Token::String(_)) {
            self.advance();
        }

        let alias = if self.match_token(&Token::As) {
            match &self.peek().token {
                Token::Identifier(name) => {
                    let alias = name.clone();
                    self.advance();
                    Some(alias)
                }
                _ => return Err(ParseError::Expected("alias name".into(), self.peek().line, self.peek().column)),
            }
        } else {
            None
        };

        Ok(Stmt::Import { module, alias })
    }

    fn parse_import_from(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // from

        let module = match &self.peek().token {
            Token::String(s) => s.clone(),
            Token::Identifier(name) => name.clone(),
            _ => return Err(ParseError::Expected("module name".into(), self.peek().line, self.peek().column)),
        };
        self.advance();

        self.expect(&Token::Import)?;

        let mut names = Vec::new();
        loop {
            match &self.peek().token {
                Token::Identifier(name) => {
                    names.push(name.clone());
                    self.advance();
                }
                _ => break,
            }
            if !self.match_token(&Token::Comma) {
                break;
            }
        }

        Ok(Stmt::ImportFrom { module, names })
    }

    fn parse_class_decl(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // class

        let name = match &self.peek().token {
            Token::Identifier(name) => name.clone(),
            _ => return Err(ParseError::Expected("class name".into(), self.peek().line, self.peek().column)),
        };
        self.advance();

        let superclass = if self.match_token(&Token::Colon) || self.match_token(&Token::Less) {
            match &self.peek().token {
                Token::Identifier(name) => {
                    let super_name = name.clone();
                    self.advance();
                    Some(super_name)
                }
                _ => return Err(ParseError::Expected("superclass name".into(), self.peek().line, self.peek().column)),
            }
        } else {
            None
        };

        let body = self.parse_class_body()?;

        Ok(Stmt::ClassDecl {
            name,
            superclass,
            body,
        })
    }

    fn parse_class_body(&mut self) -> Result<ClassBody, ParseError> {
        self.expect(&Token::LeftBrace)?;
        self.skip_newlines();

        let mut methods = Vec::new();
        let mut properties = Vec::new();
        let mut constructor = None;

        while !matches!(self.peek().token, Token::RightBrace | Token::EOF) {
            let stmt = self.parse_stmt()?;
            match &stmt {
                Stmt::FuncDecl { name, .. } if name == "init" => {
                    constructor = Some(stmt);
                }
                Stmt::FuncDecl { .. } => {
                    methods.push(stmt);
                }
                Stmt::VarDecl { .. } => {
                    properties.push(stmt);
                }
                _ => {
                    methods.push(stmt);
                }
            }
            self.skip_newlines();
        }

        self.expect(&Token::RightBrace)?;

        Ok(ClassBody {
            methods,
            properties,
            constructor,
        })
    }

    fn parse_try_catch(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // try
        let try_body = self.parse_block()?;

        let mut catch_var = None;
        let mut catch_body = None;

        if self.match_token(&Token::Catch) {
            if self.match_token(&Token::LeftParen) {
                if let Token::Identifier(name) = &self.peek().token {
                    catch_var = Some(name.clone());
                    self.advance();
                }
                self.expect(&Token::RightParen)?;
            }
            catch_body = Some(self.parse_block()?);
        }

        let finally_body = if self.match_token(&Token::Finally) {
            Some(self.parse_block()?)
        } else {
            None
        };

        Ok(Stmt::TryCatch {
            try_body,
            catch_var,
            catch_body,
            finally_body,
        })
    }

    fn parse_module(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // module

        let name = match &self.peek().token {
            Token::Identifier(name) => name.clone(),
            _ => return Err(ParseError::Expected("module name".into(), self.peek().line, self.peek().column)),
        };
        self.advance();

        let body = self.parse_block()?;

        Ok(Stmt::Module { name, body })
    }

    fn parse_export(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // export
        let stmt = self.parse_stmt()?;
        Ok(Stmt::Export(Box::new(stmt)))
    }

    fn parse_type_decl(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // type

        let name = match &self.peek().token {
            Token::Identifier(name) => name.clone(),
            _ => return Err(ParseError::Expected("type name".into(), self.peek().line, self.peek().column)),
        };
        self.advance();

        self.expect(&Token::Assign)?;

        let type_expr = self.parse_type_expr()?;

        Ok(Stmt::TypeDecl { name, type_expr })
    }

    fn parse_type_expr(&mut self) -> Result<TypeExpr, ParseError> {
        let mut types = Vec::new();
        types.push(self.parse_single_type()?);

        while self.match_token(&Token::Pipe) {
            types.push(self.parse_single_type()?);
        }

        if types.len() == 1 {
            Ok(types.pop().unwrap())
        } else {
            Ok(TypeExpr::Union(types))
        }
    }

    fn parse_single_type(&mut self) -> Result<TypeExpr, ParseError> {
        match &self.peek().token {
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();

                if self.match_token(&Token::LeftBracket) {
                    self.expect(&Token::RightBracket)?;
                    Ok(TypeExpr::Array(Box::new(TypeExpr::Simple(name))))
                } else if self.match_token(&Token::Less) {
                    let mut args = Vec::new();
                    args.push(self.parse_type_expr()?);
                    while self.match_token(&Token::Comma) {
                        args.push(self.parse_type_expr()?);
                    }
                    self.expect(&Token::Greater)?;
                    Ok(TypeExpr::Generic { name, args })
                } else {
                    Ok(TypeExpr::Simple(name))
                }
            }
            Token::LeftBracket => {
                self.advance();
                let elem_type = self.parse_type_expr()?;
                self.expect(&Token::RightBracket)?;
                Ok(TypeExpr::Array(Box::new(elem_type)))
            }
            _ => Err(ParseError::Expected("type".into(), self.peek().line, self.peek().column)),
        }
    }

    fn parse_interface_decl(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // interface

        let name = match &self.peek().token {
            Token::Identifier(name) => name.clone(),
            _ => return Err(ParseError::Expected("interface name".into(), self.peek().line, self.peek().column)),
        };
        self.advance();

        self.expect(&Token::LeftBrace)?;
        self.skip_newlines();

        let mut methods = Vec::new();

        while !matches!(self.peek().token, Token::RightBrace | Token::EOF) {
            let method_name = match &self.peek().token {
                Token::Identifier(name) => name.clone(),
                _ => return Err(ParseError::Expected("method name".into(), self.peek().line, self.peek().column)),
            };
            self.advance();

            self.expect(&Token::LeftParen)?;
            let params = self.parse_params()?;
            self.expect(&Token::RightParen)?;

            let return_type = if self.match_token(&Token::Arrow) || self.match_token(&Token::Colon) {
                Some(self.parse_type_expr()?)
            } else {
                None
            };

            methods.push(InterfaceMethod {
                name: method_name,
                params,
                return_type,
            });

            self.skip_newlines();
        }

        self.expect(&Token::RightBrace)?;

        Ok(Stmt::InterfaceDecl { name, methods })
    }

    fn parse_enum_decl(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // enum

        let name = match &self.peek().token {
            Token::Identifier(name) => name.clone(),
            _ => return Err(ParseError::Expected("enum name".into(), self.peek().line, self.peek().column)),
        };
        self.advance();

        self.expect(&Token::LeftBrace)?;
        self.skip_newlines();

        let mut variants = Vec::new();

        while !matches!(self.peek().token, Token::RightBrace | Token::EOF) {
            let variant_name = match &self.peek().token {
                Token::Identifier(name) => name.clone(),
                _ => return Err(ParseError::Expected("variant name".into(), self.peek().line, self.peek().column)),
            };
            self.advance();

            let data = if self.match_token(&Token::LeftParen) {
                let mut types = Vec::new();
                types.push(self.parse_type_expr()?);
                while self.match_token(&Token::Comma) {
                    types.push(self.parse_type_expr()?);
                }
                self.expect(&Token::RightParen)?;
                Some(types)
            } else {
                None
            };

            variants.push(EnumVariant {
                name: variant_name,
                data,
            });

            self.match_token(&Token::Comma);
            self.skip_newlines();
        }

        self.expect(&Token::RightBrace)?;

        Ok(Stmt::EnumDecl { name, variants })
    }

    // Expression parsing with precedence climbing
    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_ternary()?;

        match &self.peek().token {
            Token::Assign => {
                self.advance();
                let value = self.parse_assignment()?;
                Ok(Expr::Assign {
                    target: Box::new(expr),
                    value: Box::new(value),
                })
            }
            Token::PlusAssign => {
                self.advance();
                let value = self.parse_assignment()?;
                Ok(Expr::CompoundAssign {
                    op: BinaryOp::Add,
                    target: Box::new(expr),
                    value: Box::new(value),
                })
            }
            Token::MinusAssign => {
                self.advance();
                let value = self.parse_assignment()?;
                Ok(Expr::CompoundAssign {
                    op: BinaryOp::Subtract,
                    target: Box::new(expr),
                    value: Box::new(value),
                })
            }
            Token::StarAssign => {
                self.advance();
                let value = self.parse_assignment()?;
                Ok(Expr::CompoundAssign {
                    op: BinaryOp::Multiply,
                    target: Box::new(expr),
                    value: Box::new(value),
                })
            }
            Token::SlashAssign => {
                self.advance();
                let value = self.parse_assignment()?;
                Ok(Expr::CompoundAssign {
                    op: BinaryOp::Divide,
                    target: Box::new(expr),
                    value: Box::new(value),
                })
            }
            _ => Ok(expr),
        }
    }

    fn parse_ternary(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_or()?;

        if self.match_token(&Token::Question) {
            let then_expr = self.parse_expr()?;
            self.expect(&Token::Colon)?;
            let else_expr = self.parse_ternary()?;
            Ok(Expr::Ternary {
                condition: Box::new(expr),
                then_expr: Box::new(then_expr),
                else_expr: Box::new(else_expr),
            })
        } else {
            Ok(expr)
        }
    }

    fn parse_or(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_and()?;

        while matches!(self.peek().token, Token::Or) {
            self.advance();
            let right = self.parse_and()?;
            left = Expr::Binary {
                op: BinaryOp::Or,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_bitwise_or()?;

        while matches!(self.peek().token, Token::And) {
            self.advance();
            let right = self.parse_bitwise_or()?;
            left = Expr::Binary {
                op: BinaryOp::And,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_bitwise_or(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_bitwise_xor()?;

        while matches!(self.peek().token, Token::Pipe) {
            self.advance();
            let right = self.parse_bitwise_xor()?;
            left = Expr::Binary {
                op: BinaryOp::BitwiseOr,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_bitwise_xor(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_bitwise_and()?;

        while matches!(self.peek().token, Token::Xor) {
            self.advance();
            let right = self.parse_bitwise_and()?;
            left = Expr::Binary {
                op: BinaryOp::BitwiseXor,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_bitwise_and(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_equality()?;

        while matches!(self.peek().token, Token::Ampersand) {
            self.advance();
            let right = self.parse_equality()?;
            left = Expr::Binary {
                op: BinaryOp::BitwiseAnd,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_comparison()?;

        while matches!(self.peek().token, Token::Equal | Token::NotEqual) {
            let op = match &self.peek().token {
                Token::Equal => BinaryOp::Equal,
                Token::NotEqual => BinaryOp::NotEqual,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_comparison()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_shift()?;

        while matches!(
            self.peek().token,
            Token::Less | Token::Greater | Token::LessEqual | Token::GreaterEqual
        ) {
            let op = match &self.peek().token {
                Token::Less => BinaryOp::Less,
                Token::Greater => BinaryOp::Greater,
                Token::LessEqual => BinaryOp::LessEqual,
                Token::GreaterEqual => BinaryOp::GreaterEqual,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_shift()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_shift(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_additive()?;

        while matches!(self.peek().token, Token::ShiftLeft | Token::ShiftRight) {
            let op = match &self.peek().token {
                Token::ShiftLeft => BinaryOp::ShiftLeft,
                Token::ShiftRight => BinaryOp::ShiftRight,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_additive()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_multiplicative()?;

        while matches!(self.peek().token, Token::Plus | Token::Minus) {
            let op = match &self.peek().token {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Subtract,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_multiplicative()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_power()?;

        while matches!(self.peek().token, Token::Star | Token::Slash | Token::Percent) {
            let op = match &self.peek().token {
                Token::Star => BinaryOp::Multiply,
                Token::Slash => BinaryOp::Divide,
                Token::Percent => BinaryOp::Modulo,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_power()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_power(&mut self) -> Result<Expr, ParseError> {
        let left = self.parse_unary()?;

        if matches!(self.peek().token, Token::Power) {
            self.advance();
            let right = self.parse_power()?; // Right associative
            Ok(Expr::Binary {
                op: BinaryOp::Power,
                left: Box::new(left),
                right: Box::new(right),
            })
        } else {
            Ok(left)
        }
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        match &self.peek().token {
            Token::Minus => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Negate,
                    operand: Box::new(operand),
                })
            }
            Token::Not => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Not,
                    operand: Box::new(operand),
                })
            }
            Token::Increment => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Increment,
                    operand: Box::new(operand),
                })
            }
            Token::Decrement => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Decrement,
                    operand: Box::new(operand),
                })
            }
            Token::At => {
                self.advance();
                if let Token::Identifier(cmd) = &self.peek().token {
                    if cmd == "ai" {
                        self.advance();
                        match &self.peek().token {
                            Token::String(prompt) => {
                                let prompt = prompt.clone();
                                self.advance();
                                Ok(Expr::AiGenerate { prompt })
                            }
                            _ => Err(ParseError::Expected("AI prompt string".into(), self.peek().line, self.peek().column)),
                        }
                    } else {
                        Err(ParseError::Expected("@ai command".into(), self.peek().line, self.peek().column))
                    }
                } else {
                    Err(ParseError::Expected("command".into(), self.peek().line, self.peek().column))
                }
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_primary()?;

        loop {
            match &self.peek().token {
                Token::LeftParen => {
                    self.advance();
                    let args = self.parse_args()?;
                    self.expect(&Token::RightParen)?;
                    expr = Expr::Call {
                        callee: Box::new(expr),
                        args,
                    };
                }
                Token::Dot => {
                    self.advance();
                    let method = match &self.peek().token {
                        Token::Identifier(name) => name.clone(),
                        _ => return Err(ParseError::Expected("property name".into(), self.peek().line, self.peek().column)),
                    };
                    self.advance();

                    if matches!(self.peek().token, Token::LeftParen) {
                        self.advance();
                        let args = self.parse_args()?;
                        self.expect(&Token::RightParen)?;
                        expr = Expr::MethodCall {
                            object: Box::new(expr),
                            method,
                            args,
                        };
                    } else {
                        expr = Expr::PropertyAccess {
                            object: Box::new(expr),
                            property: method,
                        };
                    }
                }
                Token::LeftBracket => {
                    self.advance();
                    let index = self.parse_expr()?;
                    self.expect(&Token::RightBracket)?;
                    expr = Expr::IndexAccess {
                        object: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                Token::Increment => {
                    self.advance();
                    expr = Expr::Unary {
                        op: UnaryOp::Increment,
                        operand: Box::new(expr),
                    };
                }
                Token::Decrement => {
                    self.advance();
                    expr = Expr::Unary {
                        op: UnaryOp::Decrement,
                        operand: Box::new(expr),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_args(&mut self) -> Result<Vec<Expr>, ParseError> {
        let mut args = Vec::new();

        if matches!(self.peek().token, Token::RightParen) {
            return Ok(args);
        }

        args.push(self.parse_expr()?);

        while self.match_token(&Token::Comma) {
            args.push(self.parse_expr()?);
        }

        Ok(args)
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        match &self.peek().token.clone() {
            Token::Integer(n) => {
                self.advance();
                Ok(Expr::Integer(*n))
            }
            Token::Float(n) => {
                self.advance();
                Ok(Expr::Float(*n))
            }
            Token::String(s) => {
                self.advance();
                Ok(Expr::String(s.clone()))
            }
            Token::True => {
                self.advance();
                Ok(Expr::Boolean(true))
            }
            Token::False => {
                self.advance();
                Ok(Expr::Boolean(false))
            }
            Token::Null => {
                self.advance();
                Ok(Expr::Null)
            }
            Token::This => {
                self.advance();
                Ok(Expr::This)
            }
            Token::Self_ => {
                self.advance();
                Ok(Expr::Self_)
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(Expr::Identifier(name))
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(&Token::RightParen)?;
                Ok(expr)
            }
            Token::LeftBracket => {
                self.advance();
                let mut elements = Vec::new();

                if !matches!(self.peek().token, Token::RightBracket) {
                    elements.push(self.parse_expr()?);
                    while self.match_token(&Token::Comma) {
                        if matches!(self.peek().token, Token::RightBracket) {
                            break;
                        }
                        elements.push(self.parse_expr()?);
                    }
                }

                self.expect(&Token::RightBracket)?;
                Ok(Expr::Array(elements))
            }
            Token::LeftBrace => {
                self.advance();
                let mut properties = Vec::new();

                if !matches!(self.peek().token, Token::RightBrace) {
                    loop {
                        let key = match &self.peek().token {
                            Token::Identifier(name) => name.clone(),
                            Token::String(s) => s.clone(),
                            _ => return Err(ParseError::Expected("object key".into(), self.peek().line, self.peek().column)),
                        };
                        self.advance();

                        self.expect(&Token::Colon)?;
                        let value = self.parse_expr()?;
                        properties.push((key, value));

                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                        if matches!(self.peek().token, Token::RightBrace) {
                            break;
                        }
                    }
                }

                self.expect(&Token::RightBrace)?;
                Ok(Expr::Object(properties))
            }
            Token::New => {
                self.advance();
                let class = self.parse_primary()?;
                self.expect(&Token::LeftParen)?;
                let args = self.parse_args()?;
                self.expect(&Token::RightParen)?;
                Ok(Expr::New {
                    class: Box::new(class),
                    args,
                })
            }
            Token::Func => {
                self.advance(); // func
                self.expect(&Token::LeftParen)?;
                let params = self.parse_lambda_params()?;
                self.expect(&Token::RightParen)?;

                if self.match_token(&Token::FatArrow) {
                    let body = self.parse_expr()?;
                    Ok(Expr::Lambda {
                        params,
                        body: Box::new(body),
                    })
                } else {
                    let body = self.parse_block()?;
                    Ok(Expr::Lambda {
                        params,
                        body: Box::new(Expr::Identifier("block".to_string())), // Placeholder
                    })
                }
            }
            Token::Match => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(&Token::LeftBrace)?;
                self.skip_newlines();

                let mut arms = Vec::new();

                while !matches!(self.peek().token, Token::RightBrace | Token::EOF) {
                    let pattern = self.parse_pattern()?;

                    let guard = if self.match_token(&Token::If) {
                        Some(self.parse_expr()?)
                    } else {
                        None
                    };

                    self.expect(&Token::FatArrow)?;
                    let body = self.parse_expr()?;

                    arms.push(MatchArm {
                        pattern,
                        guard,
                        body,
                    });

                    self.skip_newlines();
                }

                self.expect(&Token::RightBrace)?;

                Ok(Expr::Match {
                    expr: Box::new(expr),
                    arms,
                })
            }
            _ => Err(ParseError::UnexpectedToken(
                self.peek().token.to_string(),
                self.peek().line,
                self.peek().column,
            )),
        }
    }

    fn parse_lambda_params(&mut self) -> Result<Vec<String>, ParseError> {
        let mut params = Vec::new();

        if matches!(self.peek().token, Token::RightParen) {
            return Ok(params);
        }

        loop {
            match &self.peek().token {
                Token::Identifier(name) => {
                    params.push(name.clone());
                    self.advance();
                }
                _ => break,
            }
            if !self.match_token(&Token::Comma) {
                break;
            }
        }

        Ok(params)
    }

    fn parse_pattern(&mut self) -> Result<Pattern, ParseError> {
        match &self.peek().token.clone() {
            Token::Integer(n) => {
                self.advance();
                Ok(Pattern::Literal(Expr::Integer(*n)))
            }
            Token::String(s) => {
                self.advance();
                Ok(Pattern::Literal(Expr::String(s.clone())))
            }
            Token::True => {
                self.advance();
                Ok(Pattern::Literal(Expr::Boolean(true)))
            }
            Token::False => {
                self.advance();
                Ok(Pattern::Literal(Expr::Boolean(false)))
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                if name == "_" {
                    Ok(Pattern::Wildcard)
                } else {
                    Ok(Pattern::Identifier(name))
                }
            }
            Token::LeftBracket => {
                self.advance();
                let mut patterns = Vec::new();
                if !matches!(self.peek().token, Token::RightBracket) {
                    patterns.push(self.parse_pattern()?);
                    while self.match_token(&Token::Comma) {
                        if matches!(self.peek().token, Token::RightBracket) {
                            break;
                        }
                        patterns.push(self.parse_pattern()?);
                    }
                }
                self.expect(&Token::RightBracket)?;
                Ok(Pattern::Array(patterns))
            }
            _ => Err(ParseError::Expected("pattern".into(), self.peek().line, self.peek().column)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_var_decl() {
        let mut lexer = Lexer::new("let x = 42");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program().unwrap();
        assert_eq!(program.stmts.len(), 1);
        assert!(matches!(&program.stmts[0], Stmt::VarDecl { name, .. } if name == "x"));
    }

    #[test]
    fn test_func_decl() {
        let input = "func greet(name) { return name }";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program().unwrap();
        assert_eq!(program.stmts.len(), 1);
        assert!(matches!(&program.stmts[0], Stmt::FuncDecl { name, .. } if name == "greet"));
    }

    #[test]
    fn test_if_else() {
        let input = "if true { let x = 1 } else { let x = 2 }";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program().unwrap();
        assert_eq!(program.stmts.len(), 1);
        assert!(matches!(&program.stmts[0], Stmt::If { .. }));
    }

    #[test]
    fn test_binary_expr() {
        let mut lexer = Lexer::new("1 + 2 * 3");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program().unwrap();
        assert_eq!(program.stmts.len(), 1);
        assert!(matches!(&program.stmts[0], Stmt::Expr(Expr::Binary { .. })));
    }
}
