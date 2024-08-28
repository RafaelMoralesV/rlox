use std::fmt::Display;

pub enum Token {
    LeftParenthesis,
    RightParenthesis,
    EndOfFile,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Token::LeftParenthesis => write!(f, "LEFT_PAREN ( null"),
            Token::RightParenthesis => write!(f, "RIGHT_PAREN ) null"),
            Token::EndOfFile => write!(f, "EOF  null"),
        }
    }
}
