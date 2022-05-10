use rlox::{ast::Statement, interpreter::Interpreter, parser, scanner};

fn parse_stmts(src: &str) -> Vec<Statement> {
    let tokens = scanner::scan_tokens(src).unwrap();
    parser::parse(&tokens)
}

/// executes the program and returns the generated output
fn exec_stmts(src: &str) -> String {
    let statements = parse_stmts(src);
    let mut out = Vec::new();
    let mut int = Interpreter::new(&mut out);

    for stmt in statements {
        int.exec_stmt(&stmt);
    }

    String::from_utf8(out).unwrap()
}

#[test]
fn test_expr_stmt() {
    let out = exec_stmts(
        r#"
        1 + 1;
        2 + 2;
    "#,
    );
    assert_eq!(out, "");
}

#[test]
fn test_print() {
    let out = exec_stmts(r#" print "Hello, World!"; "#);
    assert_eq!(out, "Hello, World!");
}

#[test]
fn test_print_expr() {
    let out = exec_stmts("print 10 + 10;");
    assert_eq!(out, "20");
}

#[test]
fn test_various_prints() {
    let out = exec_stmts(r#"print "Hello, "; print "World!";"#);
    assert_eq!(out, "Hello, World!");
}

#[test]
fn test_var_decl() {
    let out = exec_stmts(
        r#"
        var a = 10;
        print a;
    "#,
    );
    assert_eq!(out, "10");
}

#[test]
fn test_var_decl_no_initializer() {
    let out = exec_stmts(
        r#"
        var a;
        print a;
    "#,
    );
    assert_eq!(out, "Nil");
}

#[test]
fn test_var_redeclaration() {
    let out = exec_stmts(
        r#"
        var a = 10;
        var a = true;
        print a;
    "#,
    );
    assert_eq!(out, "true");
}

#[test]
#[should_panic]
fn test_undefined_var_use() {
    exec_stmts("print a;");
}

#[test]
fn test_assignment() {
    let out = exec_stmts(
        r#"
        var a = 100;
        a = false;
        print a;
    "#,
    );
    assert_eq!(out, "false");
}

#[test]
#[should_panic]
fn test_assignment_of_undefined_var() {
    exec_stmts(
        r#"
        a = 10;
    "#,
    );
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
    );
    assert_eq!(out, "10");
}

#[test]
fn test_block_execution() {
    let out = exec_stmts(r#"
        {
            print "Hello, ";
            print "World!";
        }
    "#);

    assert_eq!(
        out,
        "Hello, World!"
    )
}

#[test]
fn test_shadowing() {
    assert_eq!(
        exec_stmts(r#"
            var a = "World!";
            {
                var a = "Hello, ";
                print a;
            }
            print a;
        "#),
        "Hello, World!"
    );
}

#[test]
fn test_variable_access_from_outer_scope() {
    assert_eq!(
        exec_stmts(r#"
            var a = 10;
            {
                print a;
            }
        "#),
        "10"
    );
}

#[test]
fn test_variable_access_from_outer_outer_scope() {
    assert_eq!(
        exec_stmts(r#"
            var a = 10;
            {
                {
                    print a;
                }
            }
        "#),
        "10"
    );
}
