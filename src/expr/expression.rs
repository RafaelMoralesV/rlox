use std::fmt::Display;

use crate::token::Literal;

use super::{binary::BinaryOperator, unary::UnaryOperator};

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
