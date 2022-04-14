use std::io::Write;

mod error;
mod scanner;

pub type Result<T> = std::result::Result<T, error::Error>;

fn main() -> Result<()> {
    let mut args = std::env::args();
    match args.len() {
        1 => run_prompt(),
        2 => run_file(args.nth(1).unwrap().as_str()),
        _ => {
            println!("Usage: rlox [script]");
            std::process::exit(64);
        }
    }
}

fn run_file(filename: &str) -> Result<()> {
    let src = std::fs::read_to_string(filename).expect("Could not read file");
    run(src.as_str())
}

fn run_prompt() -> Result<()> {
    let mut src = String::new();

    loop {
        print!("> ");
        std::io::stdout().lock().flush();

        src.clear();
        match std::io::stdin().read_line(&mut src) {
            Ok(_) => {
                let result = run(src.as_str());
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

fn run(src: &str) -> Result<()> {
    let tokens = scanner::scan_tokens(src)?;
    println!("{:?}", tokens);
    Ok(())
}
