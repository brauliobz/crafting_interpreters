use rlox::{ast::Statement, interpreter::Interpreter, parser, scanner};

fn parse_stmts(src: &str) -> Vec<Statement> {
    let tokens = scanner::scan_tokens(src).unwrap();
    parser::parse(&tokens)
}

/// executes the program and returns the generated output
fn exec_stmts(src: &str) -> Vec<u8> {
    let statements = parse_stmts(src);
    let mut out = Vec::new();
    let mut int = Interpreter::new(&mut out);

    for stmt in statements {
        int.exec_stmt(&stmt);
    }

    out
}

#[test]
fn test_expr_stmt() {
    let out = exec_stmts(
        r#"
        1 + 1;
        2 + 2;
    "#,
    );
    assert_eq!(out, b"");
}

#[test]
fn test_print() {
    let out = exec_stmts(r#" print "Hello, World!"; "#);
    assert_eq!(out, b"Hello, World!");
}

#[test]
fn test_print_expr() {
    let out = exec_stmts("print 10 + 10;");
    assert_eq!(out, b"20");
}

#[test]
fn test_various_prints() {
    let out = exec_stmts(r#"print "Hello, "; print "World!";"#);
    assert_eq!(out, b"Hello, World!");
}

#[test]
fn test_var_decl() {
    let out = exec_stmts(
        r#"
        var a = 10;
        print a;
    "#,
    );
    assert_eq!(out, b"10");
}

#[test]
fn test_var_decl_no_initializer() {
    let out = exec_stmts(
        r#"
        var a;
        print a;
    "#,
    );
    assert_eq!(out, b"Nil");
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
    assert_eq!(out, b"true");
}

#[test]
#[should_panic]
fn test_undefined_var_use() {
    exec_stmts("print a;");
}
