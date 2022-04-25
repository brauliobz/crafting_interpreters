use rlox::{
    ast::*,
    parser,
    scanner::{self, TokenType::*},
};

#[test]
fn test_sum() {
    let result = parser::parse_expr(&scanner::scan_tokens("1 + 1").unwrap());
    assert_eq!(
        result,
        Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr::Number(1.0))),
            op: Plus,
            right: Box::new(Expr::Literal(LiteralExpr::Number(1.0)))
        })
    );
}

#[test]
fn test_subtraction() {
    let result = parser::parse_expr(&scanner::scan_tokens("1 - 1").unwrap());
    assert_eq!(
        result,
        Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr::Number(1.0))),
            op: Minus,
            right: Box::new(Expr::Literal(LiteralExpr::Number(1.0)))
        })
    );
}
