use rlox::{
    ast::*,
    parser,
    scanner::{self, TokenType::*},
    Result,
};

fn parse(src: &str) -> Result<Vec<Statement>> {
    parser::parse(&scanner::scan_tokens(src).unwrap())
}

#[test]
fn test_sum() {
    let result = parse("1 + 1;").unwrap();
    assert_eq!(
        result,
        vec![Statement::Expr(Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr::Number(1.0))),
            op: Plus,
            right: Box::new(Expr::Literal(LiteralExpr::Number(1.0)))
        }))]
    );
}

#[test]
fn test_subtraction() {
    let result = parse("1 - 1;").unwrap();
    assert_eq!(
        result,
        vec![Statement::Expr(Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr::Number(1.0))),
            op: Minus,
            right: Box::new(Expr::Literal(LiteralExpr::Number(1.0)))
        }))]
    );
}

#[test]
fn test_print() {
    let result = parse("print 10;").unwrap();
    assert_eq!(
        result,
        vec![Statement::Print(Expr::Literal(LiteralExpr::Number(10.0)))]
    );
}

#[test]
fn test_print_with_expr() {
    let result = parse("print 10 + 11;").unwrap();
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
        parse("a = 10;").unwrap(),
        vec![Statement::Expr(Expr::Assignment(
            "a".into(),
            Box::new(Expr::Literal(LiteralExpr::Number(10.0)))
        ))]
    );
}

#[test]
fn test_assignment_of_expression() {
    assert_eq!(
        parse("a = 10 + 11;").unwrap(),
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
fn test_assignment_invalid_lvalue() {
    assert!(parse("10 = a;").is_err());
}

#[test]
fn test_empty_block() {
    assert_eq!(parse("{}").unwrap(), vec![Statement::Block(vec![])]);
}

#[test]
fn test_nonempty_block() {
    assert_eq!(
        parse("{ var a = 10; a = 1; }").unwrap(),
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
fn test_unfinished_block() {
    assert!(parse("{ a = 10; ").is_err());
}

#[test]
fn test_if_then() {
    assert_eq!(
        parse(r#" if (true) print "Hello"; "#).unwrap(),
        vec![Statement::If(IfStatement {
            cond: Expr::Literal(LiteralExpr::Boolean(true)),
            then_branch: Box::new(Statement::Print(Expr::Literal(LiteralExpr::String(
                "Hello".into()
            )))),
            else_branch: None,
        })]
    );
}

#[test]
fn test_if_then_with_block() {
    assert_eq!(
        parse(r#" if (true) { print "Hello"; } "#).unwrap(),
        vec![Statement::If(IfStatement {
            cond: Expr::Literal(LiteralExpr::Boolean(true)),
            then_branch: Box::new(Statement::Block(vec![Statement::Print(Expr::Literal(
                LiteralExpr::String("Hello".into())
            ))])),
            else_branch: None,
        })]
    );
}

#[test]
fn test_else() {
    assert_eq!(
        parse(r#" if (true) print "Hello"; else print "World"; "#).unwrap(),
        vec![Statement::If(IfStatement {
            cond: Expr::Literal(LiteralExpr::Boolean(true)),
            then_branch: Box::new(Statement::Print(Expr::Literal(LiteralExpr::String(
                "Hello".into()
            )))),
            else_branch: Some(Box::new(Statement::Print(Expr::Literal(
                LiteralExpr::String("World".into())
            )))),
        })]
    );
}

#[test]
fn test_else_with_block() {
    assert_eq!(
        parse(r#" if (true) print "Hello"; else { print "World"; } "#).unwrap(),
        vec![Statement::If(IfStatement {
            cond: Expr::Literal(LiteralExpr::Boolean(true)),
            then_branch: Box::new(Statement::Print(Expr::Literal(LiteralExpr::String(
                "Hello".into()
            )))),
            else_branch: Some(Box::new(Statement::Block(vec![Statement::Print(
                Expr::Literal(LiteralExpr::String("World".into()))
            )]))),
        })]
    );
}

#[test]
fn test_else_if() {
    assert_eq!(
        parse(
            r#"
            if (true)
                print "Hello";
            else if (true)
                print "World";
            else
                print "!"; "#
        )
        .unwrap(),
        vec![Statement::If(IfStatement {
            cond: Expr::Literal(LiteralExpr::Boolean(true)),
            then_branch: Box::new(Statement::Print(Expr::Literal(LiteralExpr::String(
                "Hello".into()
            )))),
            else_branch: Some(Box::new(Statement::If(IfStatement {
                cond: Expr::Literal(LiteralExpr::Boolean(true)),
                then_branch: Box::new(Statement::Print(Expr::Literal(LiteralExpr::String(
                    "World".into()
                )))),
                else_branch: Some(Box::new(Statement::Print(Expr::Literal(
                    LiteralExpr::String("!".into())
                ))))
            }))),
        })]
    );
}

#[test]
fn test_dangling_else_goes_to_innermost_if() {
    assert_eq!(
        parse(
            r#"
            if (true)
                if (true)
                    print "Hello";
                else
                    print "World"; "#
        )
        .unwrap(),
        vec![Statement::If(IfStatement {
            cond: Expr::Literal(LiteralExpr::Boolean(true)),
            then_branch: Box::new(Statement::If(IfStatement {
                cond: Expr::Literal(LiteralExpr::Boolean(true)),
                then_branch: Box::new(Statement::Print(Expr::Literal(LiteralExpr::String(
                    "Hello".into()
                )))),
                else_branch: Some(Box::new(Statement::Print(Expr::Literal(
                    LiteralExpr::String("World".into())
                )))),
            })),
            else_branch: None,
        })]
    );
}

#[test]
fn test_while() {
    assert_eq!(
        parse(
            r#"
            while (false) print "Hello";
        "#
        )
        .unwrap(),
        vec![Statement::While(WhileStatement {
            cond: Expr::Literal(LiteralExpr::Boolean(false)),
            stmt: Box::new(Statement::Print(Expr::Literal(LiteralExpr::String(
                "Hello".into()
            )))),
        })]
    );
}

#[test]
fn test_while_with_block() {
    assert_eq!(
        parse(
            r#"
            while (false) {
                print "Hello";
            }
        "#
        )
        .unwrap(),
        vec![Statement::While(WhileStatement {
            cond: Expr::Literal(LiteralExpr::Boolean(false)),
            stmt: Box::new(Statement::Block(vec![Statement::Print(Expr::Literal(
                LiteralExpr::String("Hello".into())
            ))])),
        })]
    );
}

#[test]
fn test_for_loop_without_clauses() {
    assert_eq!(
        parse(
            r#"
            for (;;)
                print "Hello";
        "#
        )
        .unwrap(),
        parse(
            r#"
            {
                while (true) {
                    print "Hello";
                }
            }
        "#
        )
        .unwrap()
    );
}

#[test]
fn test_for_loop_without_initialization() {
    assert_eq!(
        parse(
            r#"
            var i = 0;
            for (; i < 10 ; i = i + 1)
                print "Hello";
        "#
        )
        .unwrap(),
        parse(
            r#"
            var i = 0;
            {
                while (i < 10) {
                    print "Hello";
                    i = i + 1;
                }
            }
        "#
        )
        .unwrap()
    );
}

#[test]
fn test_for_loop_with_var_declaration() {
    assert_eq!(
        parse(
            r#"
            for (var i = 0;;)
                print "Hello";
        "#
        )
        .unwrap(),
        parse(
            r#"
            {
                var i = 0;
                while (true) {
                    print "Hello";
                }
            }
        "#
        )
        .unwrap()
    );
}

#[test]
fn test_for_loop_with_expression_initialization() {
    assert_eq!(
        parse(
            r#"
            var i;
            for (i = 0;;)
                print "Hello";
        "#
        )
        .unwrap(),
        parse(
            r#"
            var i;
            {
                i = 0;
                while (true) {
                    print "Hello";
                }
            }
        "#
        )
        .unwrap()
    );
}

#[test]
fn test_for_loop_with_all_clauses() {
    assert_eq!(
        parse(
            r#"
                for (var i = 0; i < 10; i = i + 1) {
                    print i;
                }
            "#
        )
        .unwrap(),
        parse(
            r#"
                {
                    var i = 0;
                    while (i < 10) {
                        {
                            print i;
                        }
                        i = i + 1;
                    }
                }
            "#
        )
        .unwrap()
    );
}

#[test]
fn test_simplest_function_call() {
    assert_eq!(
        parse("f();").unwrap(),
        vec![Statement::Expr(Expr::Call(CallExpr {
            callee: Box::new(Expr::Identifier("f".into())),
            args: vec![]
        }))]
    );
}

#[test]
fn test_call_with_one_argument() {
    assert_eq!(
        parse("f(1);").unwrap(),
        vec![Statement::Expr(Expr::Call(CallExpr {
            callee: Box::new(Expr::Identifier("f".into())),
            args: vec![Expr::Literal(LiteralExpr::Number(1.0))]
        }))]
    );
}

#[test]
fn test_call_with_3_arguments() {
    assert_eq!(
        parse("f(1, 2 + a, 3 * b);").unwrap(),
        vec![Statement::Expr(Expr::Call(CallExpr {
            callee: Box::new(Expr::Identifier("f".into())),
            args: vec![
                Expr::Literal(LiteralExpr::Number(1.0)),
                Expr::Binary(BinaryExpr {
                    left: Box::new(Expr::Literal(LiteralExpr::Number(2.0))),
                    op: Plus,
                    right: Box::new(Expr::Identifier("a".into())),
                }),
                Expr::Binary(BinaryExpr {
                    left: Box::new(Expr::Literal(LiteralExpr::Number(3.0))),
                    op: Star,
                    right: Box::new(Expr::Identifier("b".into())),
                })
            ]
        }))]
    );
}

#[test]
fn test_call_with_nontrivial_callee() {
    assert_eq!(
        parse("(f())();").unwrap(),
        vec![Statement::Expr(Expr::Call(CallExpr {
            callee: Box::new(Expr::Grouping(Box::new(Expr::Call(CallExpr {
                callee: Box::new(Expr::Identifier("f".into())),
                args: vec![],
            })))),
            args: vec![]
        }))]
    );
}

#[test]
fn test_call_after_call() {
    assert_eq!(
        parse("f()();").unwrap(),
        vec![Statement::Expr(Expr::Call(CallExpr {
            callee: Box::new(Expr::Call(CallExpr {
                callee: Box::new(Expr::Identifier("f".into())),
                args: vec![],
            })),
            args: vec![],
        }))]
    );
}

#[test]
fn test_call_trailing_comma() {
    assert!(matches!(parse("f(a,)"), Err(_)));
}

#[test]
fn test_call_expression_expected() {
    assert!(matches!(parse("f(, a)"), Err(_)));
}

#[test]
fn test_call_without_closing_parenthesis() {
    assert!(matches!(parse("f("), Err(_)))
}

#[test]
fn test_call_without_closing_parenthesis_after_arg() {
    assert!(matches!(parse("f(a"), Err(_)))
}

#[test]
fn test_call_without_closing_parenthesis_after_args() {
    assert!(matches!(parse("f(a, b, c"), Err(_)))
}

#[test]
fn test_call_with_too_many_arguments() {
    assert!(matches!(
        parse(
            "f(
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
                11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
                31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
                41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
                51, 52, 53, 54, 55, 56, 57, 58, 59, 60,
                61, 62, 63, 64, 65, 66, 67, 68, 69, 70,
                71, 72, 73, 74, 75, 76, 77, 78, 79, 80,
                81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
                91, 92, 93, 94, 95, 96, 97, 98, 99, 100,
                101, 102, 103, 104, 105, 106, 107, 108, 109, 110,
                111, 112, 113, 114, 115, 116, 117, 118, 119, 120,
                121, 122, 123, 124, 125, 126, 127, 128, 129, 130,
                131, 132, 133, 134, 135, 136, 137, 138, 139, 140,
                141, 142, 143, 144, 145, 146, 147, 148, 149, 150,
                151, 152, 153, 154, 155, 156, 157, 158, 159, 160,
                161, 162, 163, 164, 165, 166, 167, 168, 169, 170,
                171, 172, 173, 174, 175, 176, 177, 178, 179, 180,
                181, 182, 183, 184, 185, 186, 187, 188, 189, 190,
                191, 192, 193, 194, 195, 196, 197, 198, 199, 200,
                201, 202, 203, 204, 205, 206, 207, 208, 209, 210,
                211, 212, 213, 214, 215, 216, 217, 218, 219, 220,
                221, 222, 223, 224, 225, 226, 227, 228, 229, 230,
                231, 232, 233, 234, 235, 236, 237, 238, 239, 240,
                241, 242, 243, 244, 245, 246, 247, 248, 249, 250,
                251, 252, 253, 254, 255, 256, 257
            );"
        ),
        Err(_)
    ));
}
