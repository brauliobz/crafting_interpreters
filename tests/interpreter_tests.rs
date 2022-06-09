use rlox::{
    ast::Statement,
    error::{ErrorOrEarlyReturn, RuntimeError},
    interpreter::Interpreter,
    parser, scanner, Result,
};

fn parse_stmts(src: &str) -> Result<Vec<Statement>> {
    let tokens = scanner::scan_tokens(src).unwrap();
    parser::parse(&tokens)
}

/// executes the program and returns the generated output
fn exec_stmts(src: &str) -> Result<String> {
    let statements = parse_stmts(src)?;
    let mut out = Vec::new();
    let mut int = Interpreter::new(&mut out);

    for stmt in statements {
        int.exec_stmt(&stmt)?;
    }

    Ok(String::from_utf8(out).unwrap())
}

#[test]
fn test_expr_stmt() {
    let out = exec_stmts(
        r#"
        1 + 1;
        2 + 2;
    "#,
    )
    .unwrap();
    assert_eq!(out, "");
}

#[test]
fn test_print() {
    let out = exec_stmts(r#" print "Hello, World!"; "#).unwrap();
    assert_eq!(out, "Hello, World!\n");
}

#[test]
fn test_print_expr() {
    let out = exec_stmts("print 10 + 10;").unwrap();
    assert_eq!(out, "20\n");
}

#[test]
fn test_various_prints() {
    let out = exec_stmts(r#"print "Hello, "; print "World!";"#).unwrap();
    assert_eq!(out, "Hello, \nWorld!\n");
}

#[test]
fn test_var_decl() {
    let out = exec_stmts(
        r#"
        var a = 10;
        print a;
    "#,
    )
    .unwrap();
    assert_eq!(out, "10\n");
}

#[test]
fn test_var_decl_no_initializer() {
    let out = exec_stmts(
        r#"
        var a;
        print a;
    "#,
    )
    .unwrap();
    assert_eq!(out, "Nil\n");
}

#[test]
fn test_var_redeclaration() {
    let out = exec_stmts(
        r#"
        var a = 10;
        var a = true;
        print a;
    "#,
    )
    .unwrap();
    assert_eq!(out, "true\n");
}

#[test]
fn test_undefined_var_use() {
    assert!(matches!(
        exec_stmts("print a;"),
        Err(ErrorOrEarlyReturn::RuntimeError(
            RuntimeError::UndefinedVariable(_)
        ))
    ));
}

#[test]
fn test_assignment() {
    let out = exec_stmts(
        r#"
        var a = 100;
        a = false;
        print a;
    "#,
    )
    .unwrap();
    assert_eq!(out, "false\n");
}

#[test]
fn test_assignment_of_undefined_var() {
    assert!(matches!(
        exec_stmts("a = 10;"),
        Err(ErrorOrEarlyReturn::RuntimeError(
            RuntimeError::UndefinedVariable(_)
        ))
    ));
}

#[test]
fn test_assignment_of_assignment() {
    let out = exec_stmts(
        r#"
        var a;
        var b;
        a = b = 10;
        print a;
    "#,
    )
    .unwrap();
    assert_eq!(out, "10\n");
}

#[test]
fn test_block_execution() {
    let out = exec_stmts(
        r#"
        {
            print "Hello, ";
            print "World!";
        }
    "#,
    )
    .unwrap();

    assert_eq!(out, "Hello, \nWorld!\n")
}

#[test]
fn test_shadowing() {
    assert_eq!(
        exec_stmts(
            r#"
            var a = "World!";
            {
                var a = "Hello, ";
                print a;
            }
            print a;
        "#
        )
        .unwrap(),
        "Hello, \nWorld!\n"
    );
}

#[test]
fn test_variable_access_from_outer_scope() {
    assert_eq!(
        exec_stmts(
            r#"
            var a = 10;
            {
                print a;
            }
        "#
        )
        .unwrap(),
        "10\n"
    );
}

#[test]
fn test_variable_access_from_outer_outer_scope() {
    assert_eq!(
        exec_stmts(
            r#"
            var a = 10;
            {
                {
                    print a;
                }
            }
        "#
        )
        .unwrap(),
        "10\n"
    );
}

#[test]
fn test_if_then() {
    assert_eq!(
        exec_stmts(r#" if (true) print "Hello, World!"; "#).unwrap(),
        "Hello, World!\n"
    )
}

#[test]
fn test_if_then_false_condition() {
    assert_eq!(
        exec_stmts(r#" if (false) print "Hello, World!"; "#).unwrap(),
        ""
    )
}

#[test]
fn test_if_then_nontrivial_condition() {
    assert_eq!(
        exec_stmts(r#" if (10 + 10 > 15) print "Hello, World!"; "#).unwrap(),
        "Hello, World!\n"
    )
}

#[test]
fn test_if_then_block() {
    assert_eq!(
        exec_stmts(r#" if (true) { print "Hello, World!"; } "#).unwrap(),
        "Hello, World!\n"
    )
}

#[test]
fn test_else_executes() {
    assert_eq!(
        exec_stmts(
            r#"
            if (false) print "Hello";
            else print "World"; "#
        )
        .unwrap(),
        "World\n"
    )
}

#[test]
fn test_else_do_not_executes() {
    assert_eq!(
        exec_stmts(
            r#"
            if (true) print "Hello";
            else print "World"; "#
        )
        .unwrap(),
        "Hello\n"
    )
}

#[test]
fn test_else_if_executes_if() {
    assert_eq!(
        exec_stmts(
            r#"
            if (false) print "Hello";
            else if (true) print "World";
            else print "!"; "#
        )
        .unwrap(),
        "World\n"
    )
}

#[test]
fn test_else_if_executes_else() {
    assert_eq!(
        exec_stmts(
            r#"
            if (false) print "Hello";
            else if (false) print "World";
            else print "!"; "#
        )
        .unwrap(),
        "!\n"
    )
}

#[test]
fn test_dangling_else_executes() {
    assert_eq!(
        exec_stmts(
            r#"
            if (true)
                if (false) print "Hello";
                else print "World"; "#
        )
        .unwrap(),
        "World\n"
    )
}

#[test]
fn test_truthyness() {
    assert_eq!(
        exec_stmts(
            r#"
            if (0) print "0 is truthy";
            if (1) print "1 is truthy";
            if ("bla") print "a string is truthy";
            if (true) print "true is truthy";
            if (false) print "false is truthy";
            if (nil) print "nil is truthy";
        "#
        )
        .unwrap(),
        r#"0 is truthy
1 is truthy
a string is truthy
true is truthy
"#
    );

    // TODO test object truthyness
}

#[test]
fn test_short_circuit_and() {
    assert_eq!(
        exec_stmts(
            r#"
            var a = true;
            var b = a and (a = "Hello");
            print a; "#
        )
        .unwrap(),
        "Hello\n"
    );
    assert_eq!(
        exec_stmts(
            r#"
            var a = false;
            var b = a and (a = "Hello");
            print a; "#
        )
        .unwrap(),
        "false\n"
    );
}

#[test]
fn test_short_circuit_or() {
    assert_eq!(
        exec_stmts(
            r#"
            var a = false;
            var b = a or (a = "Hello");
            print a; "#
        )
        .unwrap(),
        "Hello\n"
    );
    assert_eq!(
        exec_stmts(
            r#"
            var a = true;
            var b = a or (a = "Hello");
            print a; "#
        )
        .unwrap(),
        "true\n"
    );
}

#[test]
fn test_while_does_not_enter() {
    assert_eq!(exec_stmts(r#" while (false) print "Hello"; "#).unwrap(), "");
}

#[test]
fn test_while_enters_once() {
    assert_eq!(
        exec_stmts(
            r#"
            var a = true;
            while (a) {
                print "Hello";
                a = false;
            } "#
        )
        .unwrap(),
        "Hello\n"
    );
}

#[test]
fn test_while_enters_10_times() {
    assert_eq!(
        exec_stmts(
            r#"
            var a = 0;
            while (a < 10) {
                a = a + 1;
                print a;
            } "#
        )
        .unwrap(),
        r#"1
2
3
4
5
6
7
8
9
10
"#
    );
}

#[test]
fn test_for_loop_with_all_clauses() {
    assert_eq!(
        exec_stmts(
            r#"
            for (var i = 0; i < 10; i = i + 1) {
                print i;
            }
        "#
        )
        .unwrap(),
        "0
1
2
3
4
5
6
7
8
9
"
    )
}

#[test]
fn test_nested_for_loops() {
    assert_eq!(
        exec_stmts(
            r#"
            for (var i = 0; i < 3; i = i + 1) {
                print "i:";
                print i;
                for (var j = 0; j < i; j = j + 1) {
                    print "j:";
                    print j;
                }
            }
        "#
        )
        .unwrap(),
        "i:
0
i:
1
j:
0
i:
2
j:
0
j:
1
"
    )
}

#[test]
fn test_for_loop_variable_scope() {
    assert_eq!(
        exec_stmts(
            r#"
            var i = 10;
            for (var i = 0; i < 3; i = i + 1) {

            }
            print i;
        "#
        )
        .unwrap(),
        "10\n"
    );
}

#[test]
fn test_for_loop_without_initialization() {
    assert_eq!(
        exec_stmts(
            r#"
            var i = 0;
            for (; i < 3; i = i + 1) {
                print i;
            } "#
        )
        .unwrap(),
        r#"0
1
2
"#
    );
}

#[test]
fn test_for_loop_with_expression_initialization() {
    assert_eq!(
        exec_stmts(
            r#"
            var i;
            for (i = 0; i < 3; i = i + 1) {
                print i;
            } "#
        )
        .unwrap(),
        r#"0
1
2
"#
    );
}

#[test]
fn test_for_loop_with_condition_only() {
    assert_eq!(
        exec_stmts(
            r#"
            var i = 0;
            for (; i < 3; ) {
                print i;
                i = i + 1;
            } "#
        )
        .unwrap(),
        r#"0
1
2
"#
    );
}

#[test]
fn test_function_call_executes() {
    assert_eq!(
        exec_stmts(
            r#"
            fun f() {
                print "inside function";
            }
            f();
        "#
        )
        .unwrap(),
        "inside function\n"
    );
}

#[test]
fn test_function_call_returns() {
    assert_eq!(
        exec_stmts(
            r#"
            fun f() {}
            f();
            print "after function";
        "#
        )
        .unwrap(),
        "after function\n"
    );
}

#[test]
fn test_assign_function_to_variable() {
    assert_eq!(
        exec_stmts(
            r#"
            fun f() {
                print "inside function";
            }
            var a = f;
            print a;
        "#
        )
        .unwrap(),
        "fun f\n"
    );
}

#[test]
fn test_function_call_via_variable() {
    assert_eq!(
        exec_stmts(
            r#"
            fun f() {
                print "inside function";
            }
            var a = f;
            a();
        "#
        )
        .unwrap(),
        "inside function\n"
    );
}

#[test]
fn test_function_access_global() {
    assert_eq!(
        exec_stmts(
            r#"
            var a = "hi from global scope";
            fun f() {
                print a;
            }
            f();
        "#
        )
        .unwrap(),
        "hi from global scope\n"
    );
}

#[test]
fn test_function_access_global_not_immediately_above() {
    assert_eq!(
        exec_stmts(
            r#"
            var a = "hi from global scope";
            fun f() {
                print a;
            }
            {
                var a = "hi from inner scope";
                f();
            }
        "#
        )
        .unwrap(),
        "hi from global scope\n"
    );
}

#[test]
fn test_recursion_without_return() {
    assert_eq!(
        exec_stmts(
            r#"
                fun f(x) {
                    print x;
                    if (x > 0) {
                        f(x - 1);
                    }
                }
                f(3);
            "#
        )
        .unwrap(),
        "3
2
1
0
"
    );
}

#[test]
fn test_return_without_expr() {
    assert_eq!(
        exec_stmts(
            r#"
                fun f() {
                    print "Hello";
                    return;
                    print "World";
                }
                f();
            "#
        )
        .unwrap(),
        "Hello\n"
    );
}

#[test]
fn test_return_with_expr() {
    assert_eq!(
        exec_stmts(
            r#"
                fun f() {
                    return "Hello, World";
                }
                print f();
            "#
        )
        .unwrap(),
        "Hello, World\n"
    );
}

#[test]
fn test_recursion() {
    assert_eq!(
        exec_stmts(
            r#"
                fun fib(n) {
                    if (n >= 2) {
                        return fib(n - 1) + fib(n - 2);
                    } else {
                        return n;
                    }
                }
                print fib(10);
            "#
        )
        .unwrap(),
        "55\n"
    );
}

#[test]
fn test_call_with_wrong_number_of_parameters() {
    assert!(matches!(
        exec_stmts(
            "
                fun f(a) { }
                f(1, 2);
            "
        ),
        Err(ErrorOrEarlyReturn::RuntimeError(
            RuntimeError::NumberOfArgumentsMismatch(1, _, 2)
        ))
    ));
}

#[test]
fn test_stack_overflow() {
    assert!(matches!(
        exec_stmts(
            "
                fun f() {
                    f();
                }
                f();
            "
        ),
        Err(ErrorOrEarlyReturn::RuntimeError(
            RuntimeError::StackOverflow
        ))
    ));
}

#[test]
fn test_native_function_call() {
    assert!(exec_stmts(" print clock(); ")
        .unwrap()
        .trim()
        .parse::<f64>()
        .is_ok());
}

#[test]
fn test_simple_closure() {
    assert_eq!(
        exec_stmts(
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
        )
        .unwrap(),
        "1\n"
    );
}

#[test]
fn test_returned_closure() {
    assert_eq!(
        exec_stmts(
            r#"
                fun f() {
                    var i = 0;
                    fun g() {
                        i = i + 1;
                        print i;
                    }
                    return g;
                }
                var h = f();
                h();
                h();
            "#
        )
        .unwrap(),
        "1\n2\n"
    );
}

#[test]
fn test_returned_closure_different_envs() {
    assert_eq!(
        exec_stmts(
            r#"
                fun f() {
                    var i = 0;
                    fun g() {
                        i = i + 1;
                        print i;
                    }
                    return g;
                }
                var h = f();
                h();
                h();
                h = f();
                h();
                h();
            "#
        )
        .unwrap(),
        "1\n2\n1\n2\n"
    );
}
