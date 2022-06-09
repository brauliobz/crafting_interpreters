use std::rc::Rc;

use rlox::{
    ast::*,
    error::{CompilationError, ErrorOrEarlyReturn},
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
    assert!(matches!(parse("f(a,);"), Err(_)));
}

#[test]
fn test_call_expression_expected() {
    assert!(matches!(parse("f(, a);"), Err(_)));
}

#[test]
fn test_call_without_closing_parenthesis() {
    assert!(matches!(parse("f(;"), Err(_)))
}

#[test]
fn test_call_without_closing_parenthesis_after_arg() {
    assert!(matches!(parse("f(a;"), Err(_)))
}

#[test]
fn test_call_without_closing_parenthesis_after_args() {
    assert!(matches!(parse("f(a, b, c;"), Err(_)))
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

#[test]
fn test_function_declaration_without_params_and_empty_body() {
    assert_eq!(
        parse("fun f() {}").unwrap(),
        vec![Statement::FunDecl(Rc::new(FunctionDecl {
            name: "f".into(),
            params: vec![],
            body: vec![]
        }))]
    );
}

#[test]
fn test_function_declaration_with_body() {
    assert_eq!(
        parse("fun f() { var a = 1; }").unwrap(),
        vec![Statement::FunDecl(Rc::new(FunctionDecl {
            name: "f".into(),
            params: vec![],
            body: vec![Statement::VariableDecl(
                "a".into(),
                Some(Expr::Literal(LiteralExpr::Number(1.0)))
            )]
        }))]
    );
}

#[test]
fn test_function_decl_without_name() {
    assert!(matches!(parse("fun () {}"), Err(_)));
}

#[test]
fn test_function_decl_with_invalid_name() {
    assert!(matches!(parse("fun 1() {}"), Err(_)));
}

#[test]
fn test_function_decl_without_body() {
    assert!(matches!(parse("fun f () ;"), Err(_)));
}

#[test]
fn test_function_decl_without_params() {
    assert!(matches!(parse("fun f {}"), Err(_)));
}

#[test]
fn test_function_decl_with_one_param() {
    assert_eq!(
        parse("fun f(x) {}").unwrap(),
        vec![Statement::FunDecl(Rc::new(FunctionDecl {
            name: "f".into(),
            params: vec!["x".into()],
            body: vec![]
        }))]
    );
}

#[test]
fn test_function_decl_with_more_than_one_param() {
    assert_eq!(
        parse("fun f(x, y) {}").unwrap(),
        vec![Statement::FunDecl(Rc::new(FunctionDecl {
            name: "f".into(),
            params: vec!["x".into(), "y".into()],
            body: vec![]
        }))]
    );
}

#[test]
fn test_function_decl_with_too_many_params() {
    assert!(matches!(
        parse(
            "
                fun f(
                    p1, p2, p3, p4, p5, p6, p7, p8, p9, p10,
                    p11, p12, p13, p14, p15, p16, p17, p18, p19, p20,
                    p21, p22, p23, p24, p25, p26, p27, p28, p29, p30,
                    p31, p32, p33, p34, p35, p36, p37, p38, p39, p40,
                    p41, p42, p43, p44, p45, p46, p47, p48, p49, p50,
                    p51, p52, p53, p54, p55, p56, p57, p58, p59, p60,
                    p61, p62, p63, p64, p65, p66, p67, p68, p69, p70,
                    p71, p72, p73, p74, p75, p76, p77, p78, p79, p80,
                    p81, p82, p83, p84, p85, p86, p87, p88, p89, p90,
                    p91, p92, p93, p94, p95, p96, p97, p98, p99, p100,
                    p101, p102, p103, p104, p105, p106, p107, p108, p109, p110,
                    p111, p112, p113, p114, p115, p116, p117, p118, p119, p120,
                    p121, p122, p123, p124, p125, p126, p127, p128, p129, p130,
                    p131, p132, p133, p134, p135, p136, p137, p138, p139, p140,
                    p141, p142, p143, p144, p145, p146, p147, p148, p149, p150,
                    p151, p152, p153, p154, p155, p156, p157, p158, p159, p160,
                    p161, p162, p163, p164, p165, p166, p167, p168, p169, p170,
                    p171, p172, p173, p174, p175, p176, p177, p178, p179, p180,
                    p181, p182, p183, p184, p185, p186, p187, p188, p189, p190,
                    p191, p192, p193, p194, p195, p196, p197, p198, p199, p200,
                    p201, p202, p203, p204, p205, p206, p207, p208, p209, p210,
                    p211, p212, p213, p214, p215, p216, p217, p218, p219, p220,
                    p221, p222, p223, p224, p225, p226, p227, p228, p229, p230,
                    p231, p232, p233, p234, p235, p236, p237, p238, p239, p240,
                    p241, p242, p243, p244, p245, p246, p247, p248, p249, p250,
                    p251, p252, p253, p254, p255, p256, p257
                ) {}
            "
        ),
        Err(_)
    ));
}

#[test]
fn test_return_without_expression() {
    assert_eq!(
        parse(
            "
                fun f() {
                    return;
                }
            "
        )
        .unwrap(),
        vec![Statement::FunDecl(Rc::new(FunctionDecl {
            name: "f".into(),
            params: vec![],
            body: vec![Statement::Return(None)]
        }))]
    );
}

#[test]
fn test_return_with_expression() {
    assert_eq!(
        parse(
            "
                fun f() {
                    return 10 + 10;
                }
            "
        )
        .unwrap(),
        vec![Statement::FunDecl(Rc::new(FunctionDecl {
            name: "f".into(),
            params: vec![],
            body: vec![Statement::Return(Some(Expr::Binary(BinaryExpr {
                left: Box::new(Expr::Literal(LiteralExpr::Number(10.0))),
                op: Plus,
                right: Box::new(Expr::Literal(LiteralExpr::Number(10.0))),
            })))]
        }))]
    );
}

#[test]
fn test_return_outside_function() {
    assert!(matches!(
        parse(
            "
                if (true) {
                    return 10;
                }
            "
        ),
        Err(ErrorOrEarlyReturn::CompilationError(
            CompilationError::ReturnOutsideFunction
        ))
    ));
}

#[test]
fn test_return_inside_outer_function() {
    assert!(matches!(
        parse(
            "
                fun f() {
                    fun g() { }
                    return 10;
                }
            "
        ),
        Ok(_)
    ));
}

#[test]
fn test_return_after_functions() {
    assert!(matches!(
        parse(
            "
                fun f() {
                    fun g() {

                    }
                }
                return 10;
            "
        ),
        Err(ErrorOrEarlyReturn::CompilationError(
            CompilationError::ReturnOutsideFunction
        ))
    ));
}
