mod eval;
mod value;

pub use eval::eval;
pub use value::Value;

use crate::expr::binary::BinaryOperator;

pub type Result<T> = core::result::Result<T, Error>;
pub enum Error {
    InvalidUnaryOperand,
    InvalidBinaryOperand { operator: BinaryOperator },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidUnaryOperand => write!(f, "Operand must be Numeric."),
            Error::InvalidBinaryOperand { operator } => {
                write!(f, "Operator '{operator}' used with invalid data types.")
            }
        }
    }
}
