use std::fmt::Display;

use crate::token::{Token, TokenType};

pub enum Expr<'a> {
    Literal(&'a Token<'a>),
    Unary {
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },
    Binary {
        operator: Token<'a>,
        left: Box<Expr<'a>>,
        right: Box<Expr<'a>>,
    },
    Grouping(Vec<Expr<'a>>),
}

impl<'a> Display for Expr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal(t) => match t.token_type {
                TokenType::True | TokenType::False | TokenType::Nil => write!(f, "{}", t.lexeme),
                _ => write!(f, "{}", t.literal),
            },
            Expr::Unary {
                operator: _,
                right: _,
            } => todo!(),
            Expr::Binary {
                operator: _,
                left: _,
                right: _,
            } => todo!(),
            Expr::Grouping(_) => todo!(),
        }
    }
}
