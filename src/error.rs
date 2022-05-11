use thiserror::Error;

use crate::scanner::TokenType;

// TODO use a vector of errors since I would like to have some error recovering?

#[derive(Debug, Error)]
pub enum Error {
    #[error("Internal compiler error: {0}")]
    ICE(#[from] ICE),
    #[error("Compilation error: {0}")]
    CompilationError(#[from] CompilationError),
    #[error("Runtime error: {0}")]
    RuntimeError(#[from] RuntimeError),
}

#[derive(Debug, Error)]
pub enum CompilationError {
    #[error("Unexpected character found")]
    UnexpectedCharacter,
    #[error("Unterminated string")]
    UnterminatedString,
}

/// Internal Compiler Error
#[derive(Debug, Error)]
pub enum ICE {
    #[error("{0}")]
    Generic(String),
    #[error("IO error: {0}")]
    IOError(std::io::Error),
}

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Type mismatch. Expected '{0}', got '{1}'")]
    TypeMismatch(String, String),
    #[error("Undefined variable '{0}'")]
    UndefinedVariable(String),
    #[error("Invalid operator '{0:?}' for values '{1}' and '{2}'")]
    InvalidOperator(TokenType, String, String),
}

pub fn ice(kind: ICE) -> Error {
    Error::ICE(kind)
}

pub fn compilation_error(kind: CompilationError) -> Error {
    Error::CompilationError(kind)
}

pub fn runtime_error(kind: RuntimeError) -> Error {
    Error::RuntimeError(kind)
}

impl From<std::io::Error> for Error {
    fn from(io_error: std::io::Error) -> Self {
        Error::ICE(ICE::IOError(io_error))
    }
}
