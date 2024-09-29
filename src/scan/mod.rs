use std::fmt::Display;

pub mod lexer;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnrecognizedCharacter { line: usize, which: char },
    UnterminatedString { line: usize },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnrecognizedCharacter { line, which } => {
                write!(f, "[line {line}] Error: Unexpected character: {which}")
            }
            Error::UnterminatedString { line } => {
                write!(f, "[line {line}] Error: Unterminated string.")
            }
        }
    }
}

impl std::error::Error for Error {}
