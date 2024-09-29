use std::fmt::Display;

pub mod parser;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UnexpectedToken { line: usize, lexeme: String },
    MissingToken,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnexpectedToken { line, lexeme } => {
                write!(f, "[line {line}] error at '{lexeme}': Expect expression.")
            }
            Error::MissingToken => write!(f, "A token was expected, but none were found."),
        }
    }
}
