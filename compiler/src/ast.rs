use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    ShiftLeft,
    ShiftRight,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOp {
    Negate,
    Not,
    Increment,
    Decrement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    // Literals
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
    Array(Vec<Expr>),
    Object(Vec<(String, Expr)>),

    // Identifiers
    Identifier(String),

    // Binary operation
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },

    // Unary operation
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
    },

    // Function call
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },

    // Method call
    MethodCall {
        object: Box<Expr>,
        method: String,
        args: Vec<Expr>,
    },

    // Property access
    PropertyAccess {
        object: Box<Expr>,
        property: String,
    },

    // Index access
    IndexAccess {
        object: Box<Expr>,
        index: Box<Expr>,
    },

    // Lambda / Arrow function
    Lambda {
        params: Vec<String>,
        body: Box<Expr>,
    },

    // Ternary conditional
    Ternary {
        condition: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
    },

    // Assignment
    Assign {
        target: Box<Expr>,
        value: Box<Expr>,
    },

    // Compound assignment
    CompoundAssign {
        op: BinaryOp,
        target: Box<Expr>,
        value: Box<Expr>,
    },

    // Await expression
    Await {
        expr: Box<Expr>,
    },

    // New instance
    New {
        class: Box<Expr>,
        args: Vec<Expr>,
    },

    // This
    This,

    // Self
    Self_,

    // Match expression
    Match {
        expr: Box<Expr>,
        arms: Vec<MatchArm>,
    },

    // AI expression
    AiGenerate {
        prompt: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expr>,
    pub body: Expr,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Pattern {
    Literal(Expr),
    Identifier(String),
    Array(Vec<Pattern>),
    Object(Vec<(String, Pattern)>),
    Wildcard,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Stmt {
    // Expression statement
    Expr(Expr),

    // Variable declaration
    VarDecl {
        name: String,
        type_annotation: Option<String>,
        value: Expr,
        is_const: bool,
    },

    // Function declaration
    FuncDecl {
        name: String,
        params: Vec<Param>,
        return_type: Option<String>,
        body: Block,
        is_async: bool,
    },

    // Return statement
    Return(Option<Expr>),

    // If statement
    If {
        condition: Expr,
        then_body: Block,
        elif_clauses: Vec<(Expr, Block)>,
        else_body: Option<Block>,
    },

    // While loop
    While {
        condition: Expr,
        body: Block,
    },

    // For loop
    For {
        variable: String,
        iterable: Expr,
        body: Block,
    },

    // Break
    Break,

    // Continue
    Continue,

    // Block
    Block(Block),

    // Import
    Import {
        module: String,
        alias: Option<String>,
    },

    // Import from
    ImportFrom {
        module: String,
        names: Vec<String>,
    },

    // Class declaration
    ClassDecl {
        name: String,
        superclass: Option<String>,
        body: ClassBody,
    },

    // Try-catch
    TryCatch {
        try_body: Block,
        catch_var: Option<String>,
        catch_body: Option<Block>,
        finally_body: Option<Block>,
    },

    // Throw
    Throw(Expr),

    // Async block
    AsyncBlock(Block),

    // Module declaration
    Module {
        name: String,
        body: Block,
    },

    // Export
    Export(Box<Stmt>),

    // Type declaration
    TypeDecl {
        name: String,
        type_expr: TypeExpr,
    },

    // Interface declaration
    InterfaceDecl {
        name: String,
        methods: Vec<InterfaceMethod>,
    },

    // Enum declaration
    EnumDecl {
        name: String,
        variants: Vec<EnumVariant>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Param {
    pub name: String,
    pub type_annotation: Option<String>,
    pub default: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassBody {
    pub methods: Vec<Stmt>,
    pub properties: Vec<Stmt>,
    pub constructor: Option<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeExpr {
    Simple(String),
    Generic {
        name: String,
        args: Vec<TypeExpr>,
    },
    Array(Box<TypeExpr>),
    Optional(Box<TypeExpr>),
    Function {
        params: Vec<TypeExpr>,
        return_type: Box<TypeExpr>,
    },
    Union(Vec<TypeExpr>),
    Intersection(Vec<TypeExpr>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterfaceMethod {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<TypeExpr>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: String,
    pub data: Option<Vec<TypeExpr>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}
