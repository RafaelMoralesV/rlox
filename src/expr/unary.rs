use std::fmt::Display;

use super::{Error, Result};
use crate::token::TokenType;

#[derive(Debug)]
pub enum UnaryOperator {
    Bang,
    Minus,
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                UnaryOperator::Bang => '!',
                UnaryOperator::Minus => '-',
            }
        )
    }
}

impl TryFrom<TokenType> for UnaryOperator {
    type Error = Error;

    fn try_from(token_type: TokenType) -> Result<Self> {
        match token_type {
            TokenType::Bang => Ok(UnaryOperator::Bang),
            TokenType::Minus => Ok(UnaryOperator::Minus),
            _ => Err(Self::Error::InvalidTypeForUnaryOperator { token_type }),
        }
    }
}
