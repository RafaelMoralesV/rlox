use std::fmt::Display;

use crate::{
    expr::{binary::BinaryOperator, expression::Expr, unary::UnaryOperator},
    token::Literal,
};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = core::result::Result<T, Error>;

pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::String(s) => write!(f, "{s}"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::Nil => write!(f, "nil"),
        }
    }
}

pub fn eval(expr: Expr) -> Result<Value> {
    match expr {
        Expr::Literal(lit) => Ok(lit.into()),
        Expr::Grouping(e) => Ok(eval(*e)?),
        Expr::Unary { operator, right } => eval_unary(operator, *right),
        Expr::Binary {
            operator,
            left,
            right,
        } => eval_binary(operator, *left, *right),
    }
}

impl From<Literal> for Value {
    fn from(lit: Literal) -> Self {
        match lit {
            Literal::Null => Value::Nil,
            Literal::String(s) => Value::String(s.into()),
            Literal::Number(n) => Value::Number(n),
            Literal::False => Value::Bool(false),
            Literal::True => Value::Bool(true),
        }
    }
}

fn eval_unary(operator: UnaryOperator, right: Expr) -> Result<Value> {
    let value = eval(right)?;
    match (operator, value) {
        (UnaryOperator::Bang, Value::Bool(b)) => Ok(Value::Bool(!b)),
        (UnaryOperator::Bang, Value::Nil) => Ok(Value::Bool(true)),
        (UnaryOperator::Bang, _) => Ok(Value::Bool(false)),
        (UnaryOperator::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
        _ => Err("Combinacion incompatible!".into()),
    }
}

fn eval_binary(operator: BinaryOperator, left: Expr, right: Expr) -> Result<Value> {
    let left = eval(left)?;
    let right = eval(right)?;

    match (left, right, operator) {
        (Value::Number(left), Value::Number(right), operator) => match operator {
            BinaryOperator::Division => Ok(Value::Number(left / right)),
            BinaryOperator::Multiplication => Ok(Value::Number(left * right)),
            BinaryOperator::Minus => Ok(Value::Number(left - right)),
            BinaryOperator::Plus => Ok(Value::Number(left + right)),
            BinaryOperator::Greater => Ok(Value::Bool(left > right)),
            BinaryOperator::GreaterEqual => Ok(Value::Bool(left >= right)),
            BinaryOperator::Less => Ok(Value::Bool(left < right)),
            BinaryOperator::LessEqual => Ok(Value::Bool(left <= right)),
            BinaryOperator::EqualEqual => Ok(Value::Bool(left == right)),
            BinaryOperator::BangEqual => Ok(Value::Bool(left != right)),
        },

        (Value::String(left), Value::String(right), operator) => match operator {
            BinaryOperator::Plus => Ok(Value::String(format!("{left}{right}"))),
            BinaryOperator::EqualEqual => Ok(Value::Bool(left == right)),
            BinaryOperator::BangEqual => Ok(Value::Bool(left != right)),
            _ => Err("Cualquier wea!".into()),
        },
        (Value::Nil, Value::Nil, BinaryOperator::BangEqual) => Ok(Value::Bool(false)),
        (Value::Nil, Value::Nil, BinaryOperator::EqualEqual) => Ok(Value::Bool(true)),
        (Value::Bool(left), Value::Bool(right), BinaryOperator::BangEqual) => {
            Ok(Value::Bool(left != right))
        }
        (Value::Bool(left), Value::Bool(right), BinaryOperator::EqualEqual) => {
            Ok(Value::Bool(left == right))
        }
        (_, _, BinaryOperator::BangEqual) => Ok(Value::Bool(true)),
        (_, _, BinaryOperator::EqualEqual) => Ok(Value::Bool(false)),
        _ => return Err("Me mandaron cualquier wea".into()),
    }
}
