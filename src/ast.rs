#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Expr {
    Integer(i64),
    Float(f64),
    String(String),
    StringInterp(Vec<InterpPart>),
    Bool(bool),
    Null,
    Ident(String),
    Array(Vec<Expr>),
    Object(Vec<(String, Expr)>),
    BinaryOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    UnaryOp {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Call {
        name: Box<Expr>,
        args: Vec<Expr>,
    },
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
    Property {
        object: Box<Expr>,
        prop: String,
    },
    This,
    Super(String),
    New {
        class: Box<Expr>,
        args: Vec<Expr>,
    },
    AnonFunc {
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    Lambda {
        params: Vec<String>,
        body: Box<Expr>,
    },
    Match {
        value: Box<Expr>,
        arms: Vec<MatchArm>,
    },
    Generic {
        base: Box<Expr>,
        type_args: Vec<Expr>,
    },
    TypeAnnotation {
        name: String,
        type_params: Option<Vec<String>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum InterpPart {
    Text(String),
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Expr,
    pub body: Expr,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct InterfaceMethod {
    pub name: String,
    pub params: Vec<String>,
    pub return_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Var {
        name: String,
        value: Expr,
    },
    Func {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    AsyncFunc {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    Return(Option<Expr>),
    If {
        condition: Expr,
        then_body: Vec<Stmt>,
        else_body: Option<Vec<Stmt>>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    For {
        var: String,
        iterable: Expr,
        body: Vec<Stmt>,
    },
    Break,
    Continue,
    Print(Vec<Expr>),
    Assign {
        name: String,
        value: Expr,
    },
    PropertyAssign {
        object: Expr,
        prop: String,
        value: Expr,
    },
    Import {
        path: String,
        names: Option<Vec<String>>,
        alias: Option<String>,
    },
    Class {
        name: String,
        parent: Option<String>,
        methods: Vec<Stmt>,
    },
    Try {
        body: Vec<Stmt>,
        catch_var: Option<String>,
        catch_body: Option<Vec<Stmt>>,
        finally_body: Option<Vec<Stmt>>,
    },
    Throw(Expr),
    Assert {
        condition: Expr,
        message: Option<Expr>,
    },
    Test {
        name: String,
        body: Vec<Stmt>,
    },
    TypeAlias {
        name: String,
        type_params: Vec<String>,
        body: Expr,
    },
    Interface {
        name: String,
        type_params: Vec<String>,
        methods: Vec<Stmt>,
    },
    GenericFunc {
        name: String,
        type_params: Vec<String>,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    GenericClass {
        name: String,
        type_params: Vec<String>,
        parent: Option<String>,
        methods: Vec<Stmt>,
    },
}
