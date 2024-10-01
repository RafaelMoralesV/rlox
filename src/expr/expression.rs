use std::fmt::Display;

use crate::primitives::Literal;

use super::{binary::BinaryOperator, unary::UnaryOperator};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal {
        literal: Literal,
        line: usize,
    },
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
            Expr::Literal {
                literal: l,
                line: _,
            } => match l {
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
