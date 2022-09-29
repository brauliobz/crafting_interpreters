use std::rc::Rc;

use rlox::{
    ast::*,
    parser,
    resolver::Resolver,
    scanner::{self, TokenType},
};

fn resolve(src: &str) -> Program {
    let tokens = scanner::scan_tokens(src).unwrap();
    let mut program = Program {
        statements: parser::parse(&tokens).unwrap(),
    };
    Resolver::new().resolve(&mut program);
    program
}

#[test]
fn test_global_variable() {
    assert_eq!(
        resolve(
            r#"
                var a = 0;
                print a;
                {
                    print a;
                }
            "#
        ),
        Program {
            statements: vec![
                Statement::VariableDecl(
                    "a".to_owned(),
                    Some(Expr::Literal(LiteralExpr::Number(0.0)))
                ),
                Statement::Print(Expr::Identifier("a".to_owned(), Some(0))),
                Statement::Block(vec![Statement::Print(Expr::Identifier(
                    "a".to_owned(),
                    Some(1)
                ))])
            ]
        }
    );
}

// TODO functions decls
// TODO function blocks
// TODO other blocks: if, else, while
// TODO functions calls
// TODO closures

#[test]
fn test_simple_closure() {
    assert_eq!(
        resolve(
            r#"
                fun f() {
                    var i = 0;
                    fun g() {
                        i = i + 1;
                        print i;
                    }
                    g();
                }
                f();
            "#
        ),
        Program {
            statements: vec![
                Statement::FunDecl(Rc::new(FunctionDecl {
                    name: "f".to_owned(),
                    params: vec![],
                    body: vec![
                        Statement::VariableDecl(
                            "i".to_owned(),
                            Some(Expr::Literal(LiteralExpr::Number(0.0)))
                        ),
                        Statement::FunDecl(Rc::new(FunctionDecl {
                            name: "g".to_owned(),
                            params: vec![],
                            body: vec![
                                Statement::Expr(Expr::Assignment(
                                    "i".to_owned(),
                                    Some(1),
                                    Box::new(Expr::Binary(BinaryExpr {
                                        left: Box::new(Expr::Identifier("i".to_owned(), Some(1))),
                                        op: TokenType::Plus,
                                        right: Box::new(Expr::Literal(LiteralExpr::Number(1.0))),
                                    }))
                                )),
                                Statement::Print(Expr::Identifier("i".to_owned(), Some(1)))
                            ]
                        })),
                        Statement::Expr(Expr::Call(CallExpr {
                            callee: Box::new(Expr::Identifier("g".to_owned(), Some(0))),
                            args: vec![],
                        }))
                    ],
                })),
                Statement::Expr(Expr::Call(CallExpr {
                    callee: Box::new(Expr::Identifier("f".to_owned(), Some(0))),
                    args: vec![],
                }))
            ]
        }
    );
}

// TODO error conditions
