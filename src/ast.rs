use crate::scanner::TokenType;

#[derive(Debug, PartialEq)]
pub struct Program {
    statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Expr(Expr),
    Print(Expr),
    VariableDecl(String, Option<Expr>),
    Block(Vec<Statement>),
    If(IfStatement),
    While(WhileStatement),
    FunDecl(FunctionDecl),
}

#[derive(Debug, PartialEq)]
pub struct IfStatement {
    pub cond: Expr,
    pub then_branch: Box<Statement>,
    pub else_branch: Option<Box<Statement>>,
}

#[derive(Debug, PartialEq)]
pub struct WhileStatement {
    pub cond: Expr,
    pub stmt: Box<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionDecl {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Identifier(String),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Grouping(Box<Expr>),
    Assignment(String, Box<Expr>),
    Call(CallExpr),
}

#[derive(Debug, PartialEq)]
pub enum LiteralExpr {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

#[derive(Debug, PartialEq)]
pub struct UnaryExpr {
    pub op: TokenType,
    pub expr: Box<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub op: TokenType,
    pub right: Box<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
}
