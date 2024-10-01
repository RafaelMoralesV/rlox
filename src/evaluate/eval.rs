use crate::expr::{binary::BinaryOperator, expression::Expr, unary::UnaryOperator};

use super::{Error, Result, Value};

pub fn eval(expr: Expr) -> Result<Value> {
    match expr {
        Expr::Literal { literal, line: _ } => Ok(literal.into()),
        Expr::Grouping(e) => Ok(eval(*e)?),
        Expr::Unary { operator, right } => eval_unary(operator, *right),
        Expr::Binary {
            operator,
            left,
            right,
        } => eval_binary(operator, *left, *right),
    }
}

fn eval_unary(operator: UnaryOperator, right: Expr) -> Result<Value> {
    let value = eval(right)?;
    match operator {
        UnaryOperator::Bang => match value {
            Value::Bool(b) => Ok(Value::Bool(!b)),
            Value::Nil => Ok(Value::Bool(true)),
            _ => Ok(Value::Bool(false)),
        },
        UnaryOperator::Minus => match value {
            Value::Number(n) => Ok(Value::Number(-n)),
            _ => Err(Error::InvalidUnaryOperand),
        },
    }
}

fn eval_binary(operator: BinaryOperator, left: Expr, right: Expr) -> Result<Value> {
    let left = eval(left)?;
    let right = eval(right)?;

    match operator {
        BinaryOperator::Division => match (left, right) {
            (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left / right)),
            _ => Err(Error::InvalidBinaryOperand { operator }),
        },
        BinaryOperator::Multiplication => match (left, right) {
            (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left * right)),
            _ => Err(Error::InvalidBinaryOperand { operator }),
        },
        BinaryOperator::Minus => match (left, right) {
            (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left - right)),
            _ => Err(Error::InvalidBinaryOperand { operator }),
        },
        BinaryOperator::Plus => match (left, right) {
            (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left + right)),
            (Value::String(left), Value::String(right)) => {
                Ok(Value::String(format!("{left}{right}")))
            }
            _ => Err(Error::InvalidBinaryOperand { operator }),
        },
        BinaryOperator::Greater => match (left, right) {
            (Value::Number(left), Value::Number(right)) => Ok(Value::Bool(left > right)),
            _ => Err(Error::InvalidBinaryOperand { operator }),
        },
        BinaryOperator::GreaterEqual => match (left, right) {
            (Value::Number(left), Value::Number(right)) => Ok(Value::Bool(left >= right)),
            _ => Err(Error::InvalidBinaryOperand { operator }),
        },
        BinaryOperator::Less => match (left, right) {
            (Value::Number(left), Value::Number(right)) => Ok(Value::Bool(left < right)),
            _ => Err(Error::InvalidBinaryOperand { operator }),
        },
        BinaryOperator::LessEqual => match (left, right) {
            (Value::Number(left), Value::Number(right)) => Ok(Value::Bool(left <= right)),
            _ => Err(Error::InvalidBinaryOperand { operator }),
        },
        BinaryOperator::BangEqual => Ok(Value::Bool(left != right)),
        BinaryOperator::EqualEqual => Ok(Value::Bool(left == right)),
    }
}
