use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
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
    Slash,

    // One or Two Character Tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    String,
    Number,
    Identifier,
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // Keywords
    EndOfFile,
}

pub struct Token<'a> {
    pub token_type: TokenType,
    lexeme: &'a str,
    literal: Literal<'a>,

    #[allow(dead_code)]
    line: usize,
}

pub enum Literal<'a> {
    Null,
    String(&'a str),
    Number(f64),
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, lexeme: &'a str, literal: Literal<'a>, line: usize) -> Self {
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

impl Display for Literal<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Literal::Null => write!(f, "null"),
            Literal::String(s) => write!(f, "{}", s),
            Literal::Number(n) => write!(f, "{n:?}"),
        }
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
                TokenType::Slash => "SLASH",
                TokenType::Bang => "BANG",
                TokenType::BangEqual => "BANG_EQUAL",
                TokenType::Equal => "EQUAL",
                TokenType::EqualEqual => "EQUAL_EQUAL",
                TokenType::Greater => "GREATER",
                TokenType::GreaterEqual => "GREATER_EQUAL",
                TokenType::Less => "LESS",
                TokenType::LessEqual => "LESS_EQUAL",
                TokenType::String => "STRING",
                TokenType::Number => "NUMBER",
                TokenType::Identifier => "IDENTIFIER",
                TokenType::EndOfFile => "EOF",
                TokenType::And => "AND",
                TokenType::Class => "CLASS",
                TokenType::Else => "ELSE",
                TokenType::False => "FALSE",
                TokenType::For => "FOR",
                TokenType::Fun => "FUN",
                TokenType::If => "IF",
                TokenType::Nil => "NIL",
                TokenType::Or => "OR",
                TokenType::Print => "PRINT",
                TokenType::Return => "RETURN",
                TokenType::Super => "SUPER",
                TokenType::This => "THIS",
                TokenType::True => "TRUE",
                TokenType::Var => "VAR",
                TokenType::While => "WHILE",
            }
        )
    }
}
