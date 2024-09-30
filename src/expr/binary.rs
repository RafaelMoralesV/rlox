use std::fmt::Display;

use super::{Error, Result};
use crate::token::TokenType;

#[derive(Debug, PartialEq, Eq)]
pub enum BinaryOperator {
    BangEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Minus,
    Plus,
    Division,
    Multiplication,
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                BinaryOperator::BangEqual => "!=",
                BinaryOperator::EqualEqual => "==",
                BinaryOperator::Greater => ">",
                BinaryOperator::GreaterEqual => ">=",
                BinaryOperator::Less => "<",
                BinaryOperator::LessEqual => "<=",
                BinaryOperator::Minus => "-",
                BinaryOperator::Plus => "+",
                BinaryOperator::Division => "/",
                BinaryOperator::Multiplication => "*",
            }
        )
    }
}

impl TryFrom<TokenType> for BinaryOperator {
    type Error = Error;

    fn try_from(token_type: TokenType) -> Result<Self> {
        match token_type {
            TokenType::BangEqual => Ok(BinaryOperator::BangEqual),
            TokenType::EqualEqual => Ok(BinaryOperator::EqualEqual),
            TokenType::Greater => Ok(BinaryOperator::Greater),
            TokenType::GreaterEqual => Ok(BinaryOperator::GreaterEqual),
            TokenType::Less => Ok(BinaryOperator::Less),
            TokenType::LessEqual => Ok(BinaryOperator::LessEqual),
            TokenType::Minus => Ok(BinaryOperator::Minus),
            TokenType::Plus => Ok(BinaryOperator::Plus),
            TokenType::Slash => Ok(BinaryOperator::Division),
            TokenType::Asterisk => Ok(BinaryOperator::Multiplication),
            _ => Err(Self::Error::InvalidTypeForBinaryOperator { token_type }),
        }
    }
}
