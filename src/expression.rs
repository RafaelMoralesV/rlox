use std::fmt::Display;

use crate::token::Token;

pub enum Expr<'a> {
    Bool(bool),
    Nil,
    Number(f64),
    String(&'a str),
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
            Expr::Bool(b) => write!(f, "{b}"),
            Expr::Nil => write!(f, "nil"),
            Expr::Number(n) => write!(f, "{n:?}"),
            Expr::String(s) => write!(f, "{s:?}"),
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
