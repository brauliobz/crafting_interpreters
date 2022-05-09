use std::io::Write;

use rlox::{interpreter::Interpreter, parser, scanner, Result};

fn main() -> Result<()> {
    let mut args = std::env::args();
    let mut interpreter = Interpreter::new();
    match args.len() {
        1 => run_prompt(&mut interpreter),
        2 => run_file(args.nth(1).unwrap().as_str(), &mut interpreter),
        _ => {
            println!("Usage: rlox [script]");
            std::process::exit(64);
        }
    }
}

fn run_file(filename: &str, interpreter: &mut Interpreter) -> Result<()> {
    let src = std::fs::read_to_string(filename).expect("Could not read file");
    run(src.as_str(), interpreter)
}

fn run_prompt(interpreter: &mut Interpreter) -> Result<()> {
    let mut src = String::new();

    loop {
        print!("> ");
        std::io::stdout().flush()?;

        src.clear();
        match std::io::stdin().read_line(&mut src) {
            Ok(_) => {
                let result = run(src.as_str(), interpreter);
                match result {
                    Ok(_value) => (),
                    Err(error) => eprintln!("Error: {}", error),
                }
            }
            // TODO show error if return is not Ok
            Err(_) => panic!("Input error"),
        }
    }
}

fn run(src: &str, interpreter: &mut Interpreter) -> Result<()> {
    let tokens = scanner::scan_tokens(src)?;
    println!("tokens: {:?}", tokens);

    let statements = parser::parse_statements(&tokens);
    println!("ast: {:?}", statements);

    for stmt in statements {
        interpreter.exec_stmt(&stmt);
    }

    Ok(())
}
