use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Singe Character Tokens
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

    // One or Two Character Tokens

    // Literals

    // Keywords
    EndOfFile,
}

pub struct Token<'a> {
    pub token_type: TokenType,
    lexeme: &'a str,
    literal: Literal,
    line: usize,
}

pub enum Literal {
    Null,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, lexeme: &'a str, literal: Literal, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Literal::Null => "null",
            }
        )
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                TokenType::LeftParenthesis => "LEFT_PAREN",
                TokenType::RightParenthesis => "RIGHT_PAREN",
                TokenType::LeftBracket => "LEFT_BRACE",
                TokenType::RightBracket => "RIGHT_BRACE",
                TokenType::Asterisk => "STAR",
                TokenType::Dot => "DOT",
                TokenType::Comma => "COMMA",
                TokenType::SemiColon => "SEMICOLON",
                TokenType::Plus => "PLUS",
                TokenType::Minus => "MINUS",
                TokenType::EndOfFile => "EOF",
            }
        )
    }
}
