use std::fmt::Display;

use crate::token::Literal;

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

#[derive(Debug)]
pub enum UnaryOperator {}

#[derive(Debug)]
pub enum BinaryOperator {
    BangEqual,
    EqualEqual,
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal(l) => match l {
                Literal::Null => write!(f, "nil"),
                _ => write!(f, "{l}"),
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
            Expr::Grouping(e) => write!(f, "(group {})", *e),
        }
    }
}
