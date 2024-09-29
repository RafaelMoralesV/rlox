pub mod binary;
pub mod expression;
pub mod unary;

use crate::token::TokenType;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidTypeForBinaryOperator { token_type: TokenType },
    InvalidTypeForUnaryOperator { token_type: TokenType },
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidTypeForUnaryOperator { token_type } => {
                write!(f, "A Binary Operator can't be created from {token_type}")
            }
            Error::InvalidTypeForBinaryOperator { token_type } => {
                write!(f, "An Unary Operator can't be created from {token_type}")
            }
        }
    }
}
