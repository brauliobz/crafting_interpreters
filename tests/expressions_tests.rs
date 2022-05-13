use rlox::{environment::Value, scanner, Result, parser, interpreter};

fn eval(expr: &str) -> Result<Value> {
    let tokens = scanner::scan_tokens(expr)?;
    let ast = parser::parse_expr(&tokens)?;
    let stdout = &mut std::io::stdout();
    let mut interpreter = interpreter::Interpreter::new(stdout);
    interpreter.calc_expr(&ast)
}

#[test]
fn test_sums() {
    assert_eq!(eval("1 + 1").unwrap(), Value::Number(2.0));
    assert_eq!(eval("1 + 1 + 1").unwrap(), Value::Number(3.0));
    assert_eq!(eval("1.1 + 2.9").unwrap(), Value::Number(4.0));
    assert_eq!(eval("0 + 1").unwrap(), Value::Number(1.0));
}

#[test]
fn test_subtractions() {
    assert_eq!(eval("1 - 1").unwrap(), Value::Number(0.0));
    assert_eq!(eval("1 - 1 - 1").unwrap(), Value::Number(-1.0));
    assert_eq!(eval("1.0 - 2.9").unwrap(), Value::Number(-1.9));
    assert_eq!(eval("0 - 1").unwrap(), Value::Number(-1.0));
}

#[test]
fn test_precedence() {
    // + vs *
    assert_eq!(eval("1 + 1 * 2").unwrap(), Value::Number(3.0));
    assert_eq!(eval("2 * 1 + 1").unwrap(), Value::Number(3.0));

    // - vs *
    assert_eq!(eval("1 - 1 * 2").unwrap(), Value::Number(-1.0));
    assert_eq!(eval("2 * 1 - 1").unwrap(), Value::Number(1.0));

    // + vs /
    assert_eq!(eval("1 + 1 / 2").unwrap(), Value::Number(1.5));
    assert_eq!(eval("2 / 1 + 1").unwrap(), Value::Number(3.0));

    // - vs /
    assert_eq!(eval("1 - 1 / 2").unwrap(), Value::Number(0.5));
    assert_eq!(eval("2 / 1 - 1").unwrap(), Value::Number(1.0));
}