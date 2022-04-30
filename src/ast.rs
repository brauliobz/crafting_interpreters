use crate::scanner::TokenType;

pub struct Ast;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Identifier(String),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Grouping(Box<Expr>),
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