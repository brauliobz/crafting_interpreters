pub mod error;

#[cfg(feature = "scanner_recursive_descent")]
pub mod scanner;
#[cfg(feature = "scanner_logos")]
#[path = "./alt/scanner_logos.rs"]
pub mod scanner;

pub mod ast;
pub mod parser;
pub mod environment;
pub mod interpreter;

pub type Result<T> = std::result::Result<T, error::Error>;
