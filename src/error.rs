use std::{error::Error, fmt::Display};

// TODO use a vector of errors since I would like to have some error recovering?

#[derive(Debug)]
pub enum LoxError {
    UnexpectedCharacter,
    UnterminatedString,
    IOError(std::io::Error),
}

impl Display for LoxError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use LoxError::*;
        match self {
            UnexpectedCharacter => write!(fmt, "Unexpected character found"),
            UnterminatedString => write!(fmt, "Unterminated string"),
            IOError(err) => write!(fmt, "IO error: {}", err),
        }
    }
}

impl Error for LoxError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            LoxError::IOError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for LoxError {
    fn from(error: std::io::Error) -> Self {
        LoxError::IOError(error)
    }
}
