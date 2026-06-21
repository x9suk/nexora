use crate::ast::*;
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> Token {
        let tok = self.tokens.get(self.pos).cloned().unwrap_or(Token::Eof);
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        tok
    }

    fn expect(&mut self, expected: &Token) {
        let tok = self.advance();
        if std::mem::discriminant(&tok) != std::mem::discriminant(expected) {
            panic!("Expected {:?}, got {:?}", expected, tok);
        }
    }

    fn skip_semicolons(&mut self) {
        while matches!(self.peek(), Token::Semicolon) {
            self.advance();
        }
    }

    fn skip_type_expr(&mut self) {
        match self.peek().clone() {
            Token::Func => {
                self.advance();
                self.expect(&Token::LParen);
                if !matches!(self.peek(), Token::RParen) {
                    loop {
                        self.skip_type_expr();
                        if matches!(self.peek(), Token::RParen) {
                            break;
                        }
                        self.expect(&Token::Comma);
                    }
                }
                self.expect(&Token::RParen);
                if matches!(self.peek(), Token::Colon) {
                    self.advance();
                    self.skip_type_expr();
                }
            }
            _ => {
                self.advance();
                if matches!(self.peek(), Token::Lt) {
                    self.advance();
                    if !matches!(self.peek(), Token::Gt) {
                        loop {
                            self.skip_type_expr();
                            if matches!(self.peek(), Token::Gt) {
                                break;
                            }
                            self.expect(&Token::Comma);
                        }
                    }
                    self.expect(&Token::Gt);
                }
                while matches!(self.peek(), Token::LBracket) {
                    self.advance();
                    self.expect(&Token::RBracket);
                }
            }
        }
    }

    fn parse_type_params(&mut self) -> Vec<String> {
        if !matches!(self.peek(), Token::Lt) {
            return Vec::new();
        }
        self.advance(); // skip '<'
        let mut params = Vec::new();
        if !matches!(self.peek(), Token::Gt) {
            loop {
                match self.advance() {
                    Token::Ident(s) => params.push(s),
                    tok => panic!("Expected type parameter name, got {:?}", tok),
                }
                if matches!(self.peek(), Token::Gt) {
                    break;
                }
                self.expect(&Token::Comma);
            }
        }
        self.expect(&Token::Gt);
        params
    }

    fn _parse_interface_methods(&mut self) -> Vec<InterfaceMethod> {
        let mut methods = Vec::new();
        while !matches!(self.peek(), Token::RBrace) {
            self.expect(&Token::Func);
            let name = match self.advance() {
                Token::Ident(s) => s,
                tok => panic!("Expected method name, got {:?}", tok),
            };
            self.expect(&Token::LParen);
            let mut params = Vec::new();
            if !matches!(self.peek(), Token::RParen) {
                loop {
                    match self.advance() {
                        Token::Ident(s) => params.push(s),
                        tok => panic!("Expected parameter name, got {:?}", tok),
                    }
                    if matches!(self.peek(), Token::Colon) {
                        self.advance();
                        self.skip_type_expr();
                    }
                    if matches!(self.peek(), Token::RParen) {
                        break;
                    }
                    self.expect(&Token::Comma);
                }
            }
            self.expect(&Token::RParen);
            let return_type = if matches!(self.peek(), Token::Colon) {
                self.advance();
                let rt = match self.peek().clone() {
                    Token::Ident(s) => Some(s),
                    _ => None,
                };
                if rt.is_some() {
                    self.advance();
                }
                // Skip complex return type parts
                while matches!(self.peek(), Token::LBracket) {
                    self.advance();
                    self.expect(&Token::RBracket);
                }
                rt
            } else {
                None
            };
            methods.push(InterfaceMethod { name, params, return_type });
            self.skip_semicolons();
        }
        methods
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while !matches!(self.peek(), Token::Eof) {
            stmts.push(self.parse_stmt());
            self.skip_semicolons();
        }
        stmts
    }

    fn parse_stmt(&mut self) -> Stmt {
        match self.peek().clone() {
            Token::Let => self.parse_var(),
            Token::Func => self.parse_func(),
            Token::Async => self.parse_async_func(),
            Token::Return => self.parse_return(),
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::For => self.parse_for(),
            Token::Print => self.parse_print(),
            Token::Import => self.parse_import(),
            Token::Class => self.parse_class(),
            Token::Try => self.parse_try(),
            Token::Throw => self.parse_throw(),
            Token::Test => self.parse_test(),
            Token::Type => {
                self.advance(); // skip 'type'
                let name = match self.advance() {
                    Token::Ident(s) => s,
                    tok => panic!("Expected type name, got {:?}", tok),
                };
                let mut type_params = Vec::new();
                if matches!(self.peek(), Token::Lt) {
                    self.advance(); // skip '<'
                    loop {
                        match self.advance() {
                            Token::Ident(s) => type_params.push(s),
                            tok => panic!("Expected type parameter, got {:?}", tok),
                        }
                        if matches!(self.peek(), Token::Gt) {
                            self.advance(); // skip '>'
                            break;
                        }
                        self.expect(&Token::Comma);
                    }
                }
                self.expect(&Token::Assign);
                let body = self.parse_expr();
                Stmt::TypeAlias { name, type_params, body }
            }
            Token::Interface => {
                self.advance(); // skip 'interface'
                let name = match self.advance() {
                    Token::Ident(s) => s,
                    tok => panic!("Expected interface name, got {:?}", tok),
                };
                let mut type_params = Vec::new();
                if matches!(self.peek(), Token::Lt) {
                    self.advance(); // skip '<'
                    loop {
                        match self.advance() {
                            Token::Ident(s) => type_params.push(s),
                            tok => panic!("Expected type parameter, got {:?}", tok),
                        }
                        if matches!(self.peek(), Token::Gt) {
                            self.advance(); // skip '>'
                            break;
                        }
                        self.expect(&Token::Comma);
                    }
                }
                self.expect(&Token::LBrace);
                let mut methods = Vec::new();
                while !matches!(self.peek(), Token::RBrace) {
                    methods.push(self.parse_stmt());
                    self.skip_semicolons();
                }
                self.expect(&Token::RBrace);
                Stmt::Interface { name, type_params, methods }
            }
            Token::Break => {
                self.advance();
                Stmt::Break
            }
            Token::Continue => {
                self.advance();
                Stmt::Continue
            }
            _ => {
                // Handle assert as function call: assert(expr, "msg")
                if matches!(self.peek(), Token::Assert) {
                    self.advance(); // skip 'assert'
                    self.expect(&Token::LParen);
                    let condition = self.parse_expr();
                    let message = if matches!(self.peek(), Token::Comma) {
                        self.advance();
                        Some(self.parse_expr())
                    } else {
                        None
                    };
                    self.expect(&Token::RParen);
                    return Stmt::Assert { condition, message };
                }
                // Check for reassignment: Ident = expr
                if let Token::Ident(name) = self.peek().clone() {
                    let saved_pos2 = self.pos;
                    self.advance(); // skip ident
                    if matches!(self.peek(), Token::Assign) {
                        self.advance(); // skip =
                        let value = self.parse_expr();
                        return Stmt::Assign { name, value };
                    }
                    // Check for property assignment: ident.prop = expr
                    if matches!(self.peek(), Token::Dot) {
                        self.advance(); // skip .
                        if let Token::Ident(prop) = self.advance() {
                            if matches!(self.peek(), Token::Assign) {
                                self.advance(); // skip =
                                let value = self.parse_expr();
                                return Stmt::PropertyAssign {
                                    object: Expr::Ident(name),
                                    prop,
                                    value,
                                };
                            }
                        }
                    }
                    self.pos = saved_pos2; // backtrack
                }
                let expr = self.parse_expr();
                // Check for property assignment: expr.prop = value (the . was already consumed by parse_call)
                if let Expr::Property { object, prop } = expr {
                    if matches!(self.peek(), Token::Assign) {
                        self.advance(); // skip =
                        let value = self.parse_expr();
                        return Stmt::PropertyAssign { object: *object, prop, value };
                    }
                    // Not an assignment, reconstruct as expression
                    return Stmt::Expr(Expr::Property { object, prop });
                }
                Stmt::Expr(expr)
            }
        }
    }

    fn parse_var(&mut self) -> Stmt {
        self.advance(); // skip 'let'
        let name = match self.advance() {
            Token::Ident(s) => s,
            tok => panic!("Expected identifier, got {:?}", tok),
        };
        if matches!(self.peek(), Token::Colon) {
            self.advance(); // skip ':'
            self.skip_type_expr();
        }
        self.expect(&Token::Assign);
        let value = self.parse_expr();
        Stmt::Var { name, value }
    }

    fn parse_func(&mut self) -> Stmt {
        self.advance(); // skip 'func'
        let name = match self.advance() {
            Token::Ident(s) => s,
            tok => panic!("Expected function name, got {:?}", tok),
        };
        let type_params = self.parse_type_params();
        self.expect(&Token::LParen);
        let mut params = Vec::new();
        if !matches!(self.peek(), Token::RParen) {
            loop {
                match self.advance() {
                    Token::Ident(s) => params.push(s),
                    tok => panic!("Expected parameter name, got {:?}", tok),
                }
                if matches!(self.peek(), Token::Colon) {
                    self.advance();
                    self.skip_type_expr();
                }
                if matches!(self.peek(), Token::RParen) {
                    break;
                }
                self.expect(&Token::Comma);
            }
        }
        self.expect(&Token::RParen);
        if matches!(self.peek(), Token::Colon) {
            self.advance();
            self.skip_type_expr();
        }
        self.expect(&Token::LBrace);
        let mut body = Vec::new();
        while !matches!(self.peek(), Token::RBrace) {
            body.push(self.parse_stmt());
            self.skip_semicolons();
        }
        self.expect(&Token::RBrace);
        if type_params.is_empty() {
            Stmt::Func { name, params, body }
        } else {
            Stmt::GenericFunc { name, type_params, params, body }
        }
    }

    fn parse_async_func(&mut self) -> Stmt {
        self.advance(); // skip 'async'
        self.expect(&Token::Func);
        let name = match self.advance() {
            Token::Ident(s) => s,
            tok => panic!("Expected function name, got {:?}", tok),
        };
        self.expect(&Token::LParen);
        let mut params = Vec::new();
        if !matches!(self.peek(), Token::RParen) {
            loop {
                match self.advance() {
                    Token::Ident(s) => params.push(s),
                    tok => panic!("Expected parameter name, got {:?}", tok),
                }
                if matches!(self.peek(), Token::Colon) {
                    self.advance();
                    self.skip_type_expr();
                }
                if matches!(self.peek(), Token::RParen) {
                    break;
                }
                self.expect(&Token::Comma);
            }
        }
        self.expect(&Token::RParen);
        if matches!(self.peek(), Token::Colon) {
            self.advance();
            self.skip_type_expr();
        }
        self.expect(&Token::LBrace);
        let mut body = Vec::new();
        while !matches!(self.peek(), Token::RBrace) {
            body.push(self.parse_stmt());
            self.skip_semicolons();
        }
        self.expect(&Token::RBrace);
        Stmt::AsyncFunc { name, params, body }
    }

    fn parse_return(&mut self) -> Stmt {
        self.advance(); // skip 'return'
        if matches!(self.peek(), Token::Semicolon)
            || matches!(self.peek(), Token::RBrace)
            || matches!(self.peek(), Token::Eof)
        {
            Stmt::Return(None)
        } else {
            let expr = self.parse_expr();
            Stmt::Return(Some(expr))
        }
    }

    fn parse_if(&mut self) -> Stmt {
        self.advance(); // skip 'if'
        let condition = self.parse_expr();
        self.expect(&Token::LBrace);
        let mut then_body = Vec::new();
        while !matches!(self.peek(), Token::RBrace) {
            then_body.push(self.parse_stmt());
            self.skip_semicolons();
        }
        self.expect(&Token::RBrace);

        let else_body = if matches!(self.peek(), Token::Else) {
            self.advance();
            self.expect(&Token::LBrace);
            let mut body = Vec::new();
            while !matches!(self.peek(), Token::RBrace) {
                body.push(self.parse_stmt());
                self.skip_semicolons();
            }
            self.expect(&Token::RBrace);
            Some(body)
        } else {
            None
        };

        Stmt::If {
            condition,
            then_body,
            else_body,
        }
    }

    fn parse_while(&mut self) -> Stmt {
        self.advance(); // skip 'while'
        let condition = self.parse_expr();
        self.expect(&Token::LBrace);
        let mut body = Vec::new();
        while !matches!(self.peek(), Token::RBrace) {
            body.push(self.parse_stmt());
            self.skip_semicolons();
        }
        self.expect(&Token::RBrace);
        Stmt::While { condition, body }
    }

    fn parse_for(&mut self) -> Stmt {
        self.advance(); // skip 'for'
        let var = match self.advance() {
            Token::Ident(s) => s,
            tok => panic!("Expected variable name, got {:?}", tok),
        };
        self.expect(&Token::In);
        let iterable = self.parse_expr();
        self.expect(&Token::LBrace);
        let mut body = Vec::new();
        while !matches!(self.peek(), Token::RBrace) {
            body.push(self.parse_stmt());
            self.skip_semicolons();
        }
        self.expect(&Token::RBrace);
        Stmt::For {
            var,
            iterable,
            body,
        }
    }

    fn parse_print(&mut self) -> Stmt {
        self.advance(); // skip 'print'
        let mut args = Vec::new();
        if !matches!(self.peek(), Token::Semicolon)
            && !matches!(self.peek(), Token::RBrace)
            && !matches!(self.peek(), Token::Eof)
        {
            args.push(self.parse_expr());
            while matches!(self.peek(), Token::Comma) {
                self.advance();
                args.push(self.parse_expr());
            }
        }
        Stmt::Print(args)
    }

    fn parse_import(&mut self) -> Stmt {
        self.advance(); // skip 'import'
        match self.peek().clone() {
            // import "math" or import "file.nx"
            Token::String(path) => {
                self.advance();
                Stmt::Import { path, names: None, alias: None }
            }
            // import { sqrt, pow } from "math"
            Token::LBrace => {
                self.advance(); // skip '{'
                let mut names = Vec::new();
                loop {
                    match self.advance() {
                        Token::Ident(name) => names.push(name),
                        tok => panic!("Expected identifier in import, got {:?}", tok),
                    }
                    if matches!(self.peek(), Token::RBrace) {
                        self.advance(); // skip '}'
                        break;
                    }
                    self.expect(&Token::Comma);
                }
                // Expect 'from'
                if matches!(self.peek(), Token::Ident(s) if s == "from") {
                    self.advance();
                }
                let path = match self.advance() {
                    Token::String(p) => p,
                    tok => panic!("Expected string path after 'from', got {:?}", tok),
                };
                Stmt::Import { path, names: Some(names), alias: None }
            }
            // import math from "math"
            Token::Ident(alias) => {
                self.advance();
                // Expect 'from'
                if matches!(self.peek(), Token::Ident(s) if s == "from") {
                    self.advance();
                }
                let path = match self.advance() {
                    Token::String(p) => p,
                    tok => panic!("Expected string path after 'from', got {:?}", tok),
                };
                Stmt::Import { path, names: None, alias: Some(alias) }
            }
            tok => panic!("Expected import statement, got {:?}", tok),
        }
    }

    fn parse_class(&mut self) -> Stmt {
        self.advance(); // skip 'class'
        let name = match self.advance() {
            Token::Ident(s) => s,
            tok => panic!("Expected class name, got {:?}", tok),
        };
        let type_params = self.parse_type_params();
        let parent = if matches!(self.peek(), Token::Extends) {
            self.advance(); // skip 'extends'
            Some(match self.advance() {
                Token::Ident(s) => s,
                tok => panic!("Expected parent class name, got {:?}", tok),
            })
        } else {
            None
        };
        self.expect(&Token::LBrace);
        let mut methods = Vec::new();
        while !matches!(self.peek(), Token::RBrace) {
            methods.push(self.parse_func());
            self.skip_semicolons();
        }
        self.expect(&Token::RBrace);
        if type_params.is_empty() {
            Stmt::Class { name, parent, methods }
        } else {
            Stmt::GenericClass { name, type_params, parent, methods }
        }
    }

    fn parse_try(&mut self) -> Stmt {
        self.advance(); // skip 'try'
        self.expect(&Token::LBrace);
        let mut body = Vec::new();
        while !matches!(self.peek(), Token::RBrace) {
            body.push(self.parse_stmt());
            self.skip_semicolons();
        }
        self.expect(&Token::RBrace);

        let catch_var;
        let catch_body;
        if matches!(self.peek(), Token::Catch) {
            self.advance();
            self.expect(&Token::LParen);
            catch_var = Some(match self.advance() {
                Token::Ident(s) => s,
                tok => panic!("Expected variable name in catch, got {:?}", tok),
            });
            self.expect(&Token::RParen);
            self.expect(&Token::LBrace);
            let mut cb = Vec::new();
            while !matches!(self.peek(), Token::RBrace) {
                cb.push(self.parse_stmt());
                self.skip_semicolons();
            }
            self.expect(&Token::RBrace);
            catch_body = Some(cb);
        } else {
            catch_var = None;
            catch_body = None;
        }

        let finally_body = if matches!(self.peek(), Token::Finally) {
            self.advance();
            self.expect(&Token::LBrace);
            let mut fb = Vec::new();
            while !matches!(self.peek(), Token::RBrace) {
                fb.push(self.parse_stmt());
                self.skip_semicolons();
            }
            self.expect(&Token::RBrace);
            Some(fb)
        } else {
            None
        };

        Stmt::Try {
            body,
            catch_var,
            catch_body,
            finally_body,
        }
    }

    fn parse_throw(&mut self) -> Stmt {
        self.advance(); // skip 'throw'
        let expr = self.parse_expr();
        Stmt::Throw(expr)
    }

    fn parse_test(&mut self) -> Stmt {
        self.advance(); // skip 'test'
        let name = match self.advance() {
            Token::String(s) => s,
            tok => panic!("Expected test name string, got {:?}", tok),
        };
        self.expect(&Token::LBrace);
        let mut body = Vec::new();
        while !matches!(self.peek(), Token::RBrace) {
            body.push(self.parse_stmt());
            self.skip_semicolons();
        }
        self.expect(&Token::RBrace);
        Stmt::Test { name, body }
    }

    fn parse_expr(&mut self) -> Expr {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Expr {
        let mut left = self.parse_and();
        while matches!(self.peek(), Token::Or) {
            self.advance();
            let right = self.parse_and();
            left = Expr::BinaryOp {
                op: BinOp::Or,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_and(&mut self) -> Expr {
        let mut left = self.parse_equality();
        while matches!(self.peek(), Token::And) {
            self.advance();
            let right = self.parse_equality();
            left = Expr::BinaryOp {
                op: BinOp::And,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_equality(&mut self) -> Expr {
        let mut left = self.parse_comparison();
        while matches!(self.peek(), Token::Eq | Token::NotEq) {
            let op = match self.advance() {
                Token::Eq => BinOp::Eq,
                Token::NotEq => BinOp::NotEq,
                _ => unreachable!(),
            };
            let right = self.parse_comparison();
            left = Expr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_comparison(&mut self) -> Expr {
        let mut left = self.parse_addition();
        while matches!(
            self.peek(),
            Token::Lt | Token::Gt | Token::LtEq | Token::GtEq
        ) {
            let op = match self.advance() {
                Token::Lt => BinOp::Lt,
                Token::Gt => BinOp::Gt,
                Token::LtEq => BinOp::LtEq,
                Token::GtEq => BinOp::GtEq,
                _ => unreachable!(),
            };
            let right = self.parse_addition();
            left = Expr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_addition(&mut self) -> Expr {
        let mut left = self.parse_multiplication();
        while matches!(self.peek(), Token::Plus | Token::Minus) {
            let op = match self.advance() {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Sub,
                _ => unreachable!(),
            };
            let right = self.parse_multiplication();
            left = Expr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_multiplication(&mut self) -> Expr {
        let mut left = self.parse_unary();
        while matches!(self.peek(), Token::Star | Token::Slash | Token::Percent) {
            let op = match self.advance() {
                Token::Star => BinOp::Mul,
                Token::Slash => BinOp::Div,
                Token::Percent => BinOp::Mod,
                _ => unreachable!(),
            };
            let right = self.parse_unary();
            left = Expr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_unary(&mut self) -> Expr {
        match self.peek().clone() {
            Token::Minus => {
                self.advance();
                let expr = self.parse_unary();
                Expr::UnaryOp {
                    op: UnaryOp::Neg,
                    expr: Box::new(expr),
                }
            }
            Token::Not => {
                self.advance();
                let expr = self.parse_unary();
                Expr::UnaryOp {
                    op: UnaryOp::Not,
                    expr: Box::new(expr),
                }
            }
            Token::Await => {
                self.advance();
                let expr = self.parse_unary();
                expr
            }
            Token::Func => {
                self.advance(); // skip 'func'
                self.expect(&Token::LParen);
                let mut params = Vec::new();
                if !matches!(self.peek(), Token::RParen) {
                    loop {
                        match self.advance() {
                            Token::Ident(s) => params.push(s),
                            tok => panic!("Expected parameter name, got {:?}", tok),
                        }
                        if matches!(self.peek(), Token::RParen) {
                            break;
                        }
                        self.expect(&Token::Comma);
                    }
                }
                self.expect(&Token::RParen);
                self.expect(&Token::LBrace);
                let mut body = Vec::new();
                while !matches!(self.peek(), Token::RBrace) {
                    body.push(self.parse_stmt());
                    self.skip_semicolons();
                }
                self.expect(&Token::RBrace);
                Expr::AnonFunc { params, body }
            }
            _ => self.parse_call(),
        }
    }

    fn parse_call(&mut self) -> Expr {
        let mut expr = self.parse_primary();

        loop {
            if matches!(self.peek(), Token::LParen) {
                self.advance();
                let mut args = Vec::new();
                if !matches!(self.peek(), Token::RParen) {
                    loop {
                        args.push(self.parse_expr());
                        if matches!(self.peek(), Token::RParen) {
                            break;
                        }
                        self.expect(&Token::Comma);
                    }
                }
                self.expect(&Token::RParen);
                expr = Expr::Call {
                    name: Box::new(expr),
                    args,
                };
            } else if matches!(self.peek(), Token::LBracket) {
                self.advance();
                let index = self.parse_expr();
                self.expect(&Token::RBracket);
                expr = Expr::Index {
                    object: Box::new(expr),
                    index: Box::new(index),
                };
            } else if matches!(self.peek(), Token::Dot) {
                self.advance();
                let prop = match self.advance() {
                    Token::Ident(s) => s,
                    tok => panic!("Expected property name, got {:?}", tok),
                };
                expr = Expr::Property {
                    object: Box::new(expr),
                    prop,
                };
            } else {
                break;
            }
        }

        expr
    }

    fn parse_primary(&mut self) -> Expr {
        match self.peek().clone() {
            Token::Integer(n) => {
                self.advance();
                Expr::Integer(n)
            }
            Token::Float(n) => {
                self.advance();
                Expr::Float(n)
            }
            Token::String(s) => {
                self.advance();
                // Check for string interpolation: "Hello ${name}"
                if s.contains("${") {
                    let mut parts = Vec::new();
                    let mut remaining = s.clone();
                    while let Some(start) = remaining.find("${") {
                        if start > 0 {
                            parts.push(crate::ast::InterpPart::Text(remaining[..start].to_string()));
                        }
                        let rest = remaining[start + 2..].to_string();
                        remaining = rest;
                        if let Some(end) = remaining.find('}') {
                            let expr_str = remaining[..end].to_string();
                            remaining = remaining[end + 1..].to_string();
                            let mut inner_lexer = crate::lexer::Lexer::new(&expr_str);
                            let inner_tokens = inner_lexer.tokenize();
                            let mut inner_parser = crate::parser::Parser::new(inner_tokens);
                            let inner_expr = inner_parser.parse_expr();
                            parts.push(crate::ast::InterpPart::Expr(inner_expr));
                        } else {
                            parts.push(crate::ast::InterpPart::Text(format!("${{{}", remaining)));
                            remaining = String::new();
                        }
                    }
                    if !remaining.is_empty() {
                        parts.push(crate::ast::InterpPart::Text(remaining));
                    }
                    Expr::StringInterp(parts)
                } else {
                    Expr::String(s)
                }
            }
            Token::Bool(b) => {
                self.advance();
                Expr::Bool(b)
            }
            Token::Null => {
                self.advance();
                Expr::Null
            }
            Token::This => {
                self.advance();
                Expr::This
            }
            Token::New => {
                self.advance();
                let class = self.parse_primary();
                self.expect(&Token::LParen);
                let mut args = Vec::new();
                if !matches!(self.peek(), Token::RParen) {
                    loop {
                        args.push(self.parse_expr());
                        if matches!(self.peek(), Token::RParen) {
                            break;
                        }
                        self.expect(&Token::Comma);
                    }
                }
                self.expect(&Token::RParen);
                Expr::New {
                    class: Box::new(class),
                    args,
                }
            }
            Token::Super => {
                self.advance(); // skip 'super'
                if matches!(self.peek(), Token::Dot) {
                    self.advance(); // skip '.'
                    let method = match self.advance() {
                        Token::Ident(s) => s,
                        tok => panic!("Expected method name after super, got {:?}", tok),
                    };
                    Expr::Super(method)
                } else if matches!(self.peek(), Token::LParen) {
                    self.advance(); // skip '('
                    let mut args = Vec::new();
                    if !matches!(self.peek(), Token::RParen) {
                        loop {
                            args.push(self.parse_expr());
                            if matches!(self.peek(), Token::RParen) {
                                break;
                            }
                            self.expect(&Token::Comma);
                        }
                    }
                    self.expect(&Token::RParen);
                    Expr::Call {
                        name: Box::new(Expr::Super("init".to_string())),
                        args,
                    }
                } else {
                    panic!("Expected '.' or '(' after super, got {:?}", self.peek());
                }
            }
            Token::Ident(s) => {
                self.advance();
                // Lambda: x => expr
                if matches!(self.peek(), Token::Arrow) {
                    self.advance(); // skip =>
                    let body = self.parse_expr();
                    return Expr::Lambda {
                        params: vec![s],
                        body: Box::new(body),
                    };
                }
                Expr::Ident(s)
            }
            Token::Match => {
                self.advance(); // skip 'match'
                let value = self.parse_expr();
                self.expect(&Token::LBrace);
                let mut arms = Vec::new();
                while !matches!(self.peek(), Token::RBrace) {
                    let pattern = self.parse_match_pattern();
                    self.expect(&Token::Arrow);
                    let body = self.parse_expr();
                    // Skip comma or semicolon between arms
                    while matches!(self.peek(), Token::Comma | Token::Semicolon) {
                        self.advance();
                    }
                    arms.push(crate::ast::MatchArm { pattern, body });
                }
                self.expect(&Token::RBrace);
                Expr::Match {
                    value: Box::new(value),
                    arms,
                }
            }
            Token::LParen => {
                let saved_pos = self.pos;
                self.advance();
                // Check for empty-param lambda: () => expr
                if matches!(self.peek(), Token::RParen) {
                    self.advance(); // skip )
                    if matches!(self.peek(), Token::Arrow) {
                        self.advance(); // skip =>
                        let body = self.parse_expr();
                        return Expr::Lambda { params: vec![], body: Box::new(body) };
                    }
                    self.pos = saved_pos;
                }
                // Check for multi-param lambda: (x, y) => expr
                if let Token::Ident(_) = self.peek().clone() {
                    let _param_pos = self.pos;
                    let mut params = Vec::new();
                    if let Token::Ident(s) = self.advance() {
                        params.push(s);
                    }
                    while matches!(self.peek(), Token::Comma) {
                        self.advance(); // skip comma
                        if let Token::Ident(s) = self.advance() {
                            params.push(s);
                        }
                    }
                    if matches!(self.peek(), Token::RParen) {
                        self.advance(); // skip )
                        if matches!(self.peek(), Token::Arrow) {
                            self.advance(); // skip =>
                            let body = self.parse_expr();
                            return Expr::Lambda { params, body: Box::new(body) };
                        }
                    }
                    self.pos = saved_pos;
                }
                let expr = self.parse_expr();
                self.expect(&Token::RParen);
                expr
            }
            Token::LBracket => {
                self.advance();
                let mut elements = Vec::new();
                if !matches!(self.peek(), Token::RBracket) {
                    loop {
                        elements.push(self.parse_expr());
                        if matches!(self.peek(), Token::RBracket) {
                            break;
                        }
                        self.expect(&Token::Comma);
                    }
                }
                self.expect(&Token::RBracket);
                Expr::Array(elements)
            }
            Token::LBrace => {
                self.advance();
                let mut pairs = Vec::new();
                if !matches!(self.peek(), Token::RBrace) {
                    loop {
                        let key = match self.advance() {
                            Token::Ident(s) => s,
                            Token::String(s) => s,
                            tok => panic!("Expected key, got {:?}", tok),
                        };
                        self.expect(&Token::Colon);
                        let value = self.parse_expr();
                        pairs.push((key, value));
                        if matches!(self.peek(), Token::RBrace) {
                            break;
                        }
                        self.expect(&Token::Comma);
                    }
                }
                self.expect(&Token::RBrace);
                Expr::Object(pairs)
            }
            tok => panic!("Unexpected token {:?}", tok),
        }
    }

    fn parse_match_pattern(&mut self) -> Expr {
        match self.peek().clone() {
            Token::Ident(s) => {
                self.advance();
                Expr::Ident(s)
            }
            Token::String(s) => {
                self.advance();
                Expr::String(s)
            }
            Token::Integer(n) => {
                self.advance();
                Expr::Integer(n)
            }
            Token::Float(n) => {
                self.advance();
                Expr::Float(n)
            }
            Token::Bool(b) => {
                self.advance();
                Expr::Bool(b)
            }
            Token::Null => {
                self.advance();
                Expr::Null
            }
            Token::Minus => {
                self.advance();
                let inner = self.parse_match_pattern();
                Expr::UnaryOp {
                    op: UnaryOp::Neg,
                    expr: Box::new(inner),
                }
            }
            _ => {
                panic!("Unexpected token in match pattern: {:?}", self.peek())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_integer() {
        let tokens = vec![Token::Integer(42), Token::Eof];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        assert_eq!(ast.len(), 1);
        assert_eq!(ast[0], Stmt::Expr(Expr::Integer(42)));
    }

    #[test]
    fn test_parse_let() {
        let tokens = vec![
            Token::Let,
            Token::Ident("x".to_string()),
            Token::Assign,
            Token::Integer(10),
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        assert_eq!(
            ast[0],
            Stmt::Var {
                name: "x".to_string(),
                value: Expr::Integer(10),
            }
        );
    }

    #[test]
    fn test_parse_func() {
        let tokens = vec![
            Token::Func,
            Token::Ident("add".to_string()),
            Token::LParen,
            Token::Ident("a".to_string()),
            Token::Comma,
            Token::Ident("b".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::Ident("a".to_string()),
            Token::Plus,
            Token::Ident("b".to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        assert_eq!(
            ast[0],
            Stmt::Func {
                name: "add".to_string(),
                params: vec!["a".to_string(), "b".to_string()],
                body: vec![Stmt::Return(Some(Expr::BinaryOp {
                    op: BinOp::Add,
                    left: Box::new(Expr::Ident("a".to_string())),
                    right: Box::new(Expr::Ident("b".to_string())),
                }))]
            }
        );
    }

    #[test]
    fn test_parse_class() {
        let tokens = vec![
            Token::Class,
            Token::Ident("Dog".to_string()),
            Token::LBrace,
            Token::Func,
            Token::Ident("bark".to_string()),
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::Print,
            Token::String("woof".to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::RBrace,
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        match &ast[0] {
            Stmt::Class { name, methods, .. } => {
                assert_eq!(name, "Dog");
                assert_eq!(methods.len(), 1);
            }
            _ => panic!("Expected Class"),
        }
    }

    #[test]
    fn test_parse_try_catch() {
        let tokens = vec![
            Token::Try,
            Token::LBrace,
            Token::Throw,
            Token::String("error".to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::Catch,
            Token::LParen,
            Token::Ident("e".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Print,
            Token::Ident("e".to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        match &ast[0] {
            Stmt::Try {
                catch_var,
                catch_body,
                ..
            } => {
                assert_eq!(catch_var.as_deref(), Some("e"));
                assert!(catch_body.is_some());
            }
            _ => panic!("Expected Try"),
        }
    }
}
