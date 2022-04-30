pub mod error;

#[cfg(feature = "scanner_clike")]
pub mod scanner;
#[cfg(feature = "scanner_rusty")]
#[path = "./alt/scanner_rusty.rs"]
pub mod scanner;
#[cfg(feature = "scanner_logos")]
#[path = "./alt/scanner_logos.rs"]
pub mod scanner;

pub mod ast;
pub mod parser;
pub mod memory;
pub mod interpreter;

pub type Result<T> = std::result::Result<T, error::LoxError>;
