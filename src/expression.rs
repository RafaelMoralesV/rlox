use std::fmt::Display;

use crate::token::{Literal, TokenType};

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    Unary {
        operator: UnaryOperator,
        right: Box<Expr>,
    },
    Binary {
        operator: BinaryOperator,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal(l) => match l {
                Literal::Null => write!(f, "nil"),
                _ => write!(f, "{l}"),
            },
            Expr::Unary { operator, right } => write!(f, "({operator} {right})"),
            Expr::Binary {
                operator,
                left,
                right,
            } => write!(f, "({operator} {left} {right})"),
            Expr::Grouping(e) => write!(f, "(group {})", *e),
        }
    }
}

#[derive(Debug)]
pub enum OperatorError {
    BadTokenType(TokenType),
}

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
    type Error = OperatorError;

    fn try_from(value: TokenType) -> Result<Self, Self::Error> {
        match value {
            TokenType::Bang => Ok(UnaryOperator::Bang),
            TokenType::Minus => Ok(UnaryOperator::Minus),
            _ => Err(OperatorError::BadTokenType(value)),
        }
    }
}

#[derive(Debug)]
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
    type Error = OperatorError;

    fn try_from(value: TokenType) -> Result<Self, Self::Error> {
        match value {
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
            _ => Err(OperatorError::BadTokenType(value)),
        }
    }
}
