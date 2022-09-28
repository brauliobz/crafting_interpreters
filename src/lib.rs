pub mod error;

#[cfg(feature = "scanner_recursive_descent")]
pub mod scanner;
#[cfg(feature = "scanner_logos")]
#[path = "./alt/scanner_logos.rs"]
pub mod scanner;

pub mod ast;
pub mod environment;
pub mod interpreter;
pub mod parser;
pub mod resolver;

pub type Result<T> = std::result::Result<T, error::ErrorOrEarlyReturn>;
