use thiserror::Error;

// TODO use a vector of errors since I would like to have some error recovering?

#[derive(Debug, Error)]
pub enum LoxError {
    #[error("Unexpected character found")]
    UnexpectedCharacter,
    #[error("Unterminated string")]
    UnterminatedString,
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}
