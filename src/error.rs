use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    UnexpectedCharacter,
    UnterminatedString,
    IOError(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            UnexpectedCharacter => write!(fmt, "Unexpected character found"),
            UnterminatedString => write!(fmt, "Unterminated string"),
            IOError(err) => write!(fmt, "IO error: {}", err),
        }
    }
}

impl std::error::Error for Error {}
