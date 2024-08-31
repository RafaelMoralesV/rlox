use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Token {
    LeftParenthesis,
    RightParenthesis,
    LeftBracket,
    RightBracket,
    EndOfFile,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Token::LeftParenthesis => write!(f, "LEFT_PAREN ( null"),
            Token::RightParenthesis => write!(f, "RIGHT_PAREN ) null"),

            Token::LeftBracket => write!(f, "LEFT_BRACE {{ null"),
            Token::RightBracket => write!(f, "RIGHT_BRACE }} null"),

            Token::EndOfFile => write!(f, "EOF  null"),
        }
    }
}
