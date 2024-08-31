use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Token {
    LeftParenthesis,
    RightParenthesis,
    LeftBracket,
    RightBracket,
    Asterisk,
    Dot,
    Comma,
    SemiColon,
    Plus,
    Minus,
    EndOfFile,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Token::LeftParenthesis => write!(f, "LEFT_PAREN ( null"),
            Token::RightParenthesis => write!(f, "RIGHT_PAREN ) null"),

            Token::LeftBracket => write!(f, "LEFT_BRACE {{ null"),
            Token::RightBracket => write!(f, "RIGHT_BRACE }} null"),

            Token::Asterisk => write!(f, "STAR * null"),
            Token::Dot => write!(f, "DOT . null"),
            Token::Comma => write!(f, "COMMA , null"),
            Token::SemiColon => write!(f, "SEMICOLON ; null"),
            Token::Plus => write!(f, "PLUS + null"),
            Token::Minus => write!(f, "MINUS - null"),

            Token::EndOfFile => write!(f, "EOF  null"),
        }
    }
}
