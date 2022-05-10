use rlox::{
    ast::*,
    parser,
    scanner::{self, TokenType::*},
};

fn parse(src: &str) -> Vec<Statement> {
    parser::parse(&scanner::scan_tokens(src).unwrap())
}

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

#[test]
fn test_print() {
    let result = parser::parse(&scanner::scan_tokens("print 10;").unwrap());
    assert_eq!(
        result,
        vec![Statement::Print(Expr::Literal(LiteralExpr::Number(10.0)))]
    );
}

#[test]
fn test_print_with_expr() {
    let result = parser::parse(&scanner::scan_tokens("print 10 + 11;").unwrap());
    assert_eq!(
        result,
        vec![Statement::Print(Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr::Number(10.0))),
            op: Plus,
            right: Box::new(Expr::Literal(LiteralExpr::Number(11.0)))
        }))]
    );
}

#[test]
fn test_assignment() {
    assert_eq!(
        parse("a = 10;"),
        vec![Statement::Expr(Expr::Assignment(
            "a".into(),
            Box::new(Expr::Literal(LiteralExpr::Number(10.0)))
        ))]
    );
}

#[test]
fn test_assignment_of_expression() {
    assert_eq!(
        parse("a = 10 + 11;"),
        vec![Statement::Expr(Expr::Assignment(
            "a".into(),
            Box::new(Expr::Binary(BinaryExpr {
                left: Box::new(Expr::Literal(LiteralExpr::Number(10.0))),
                op: Plus,
                right: Box::new(Expr::Literal(LiteralExpr::Number(11.0)))
            }))
        ))]
    );
}

#[test]
#[should_panic]
fn test_assignment_invalid_lvalue() {
    parse("10 = a;");
}

#[test]
fn test_empty_block() {
    assert_eq!(parse("{}"), vec![Statement::Block(vec![])]);
}

#[test]
fn test_nonempty_block() {
    assert_eq!(
        parse("{ var a = 10; a = 1; }"),
        vec![Statement::Block(vec![
            Statement::VariableDecl("a".into(), Some(Expr::Literal(LiteralExpr::Number(10.0)))),
            Statement::Expr(Expr::Assignment(
                "a".into(),
                Box::new(Expr::Literal(LiteralExpr::Number(1.0)))
            ))
        ])]
    );
}

#[test]
#[should_panic]
fn test_unfinished_block() {
    parse("{ a = 10; ");
}
