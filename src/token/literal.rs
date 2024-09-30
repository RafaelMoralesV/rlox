#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Null,
    String(String),
    Number(f64),
    False,
    True,
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Literal::Null => write!(f, "null"),
            Literal::String(s) => write!(f, "{}", s),
            Literal::Number(n) => write!(f, "{n:?}"),
            Literal::True => write!(f, "true"),
            Literal::False => write!(f, "false"),
        }
    }
}
