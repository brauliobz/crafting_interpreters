mod error;
mod scanner;

fn main() {
    scanner::scan_tokens("print \"Hello, world\"");
    todo!()
}

fn run_file() {
    todo!()
}

fn run_prompt() {
    todo!()
}

fn error(line: u32, message: &str) {
    todo!()
}

fn report(line: i32, where_: &str, message: &str) {
    todo!()
}
