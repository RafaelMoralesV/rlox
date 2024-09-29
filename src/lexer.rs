use std::collections::HashMap;

use crate::token::{Literal, Token, TokenType};

pub struct Lexer<'a> {
    pub input: &'a str,
    pub index: usize,
    pub line: usize,
    pub reserved_words: HashMap<&'a str, TokenType>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let reserved_words = HashMap::from([
            ("and", TokenType::And),
            ("class", TokenType::Class),
            ("else", TokenType::Else),
            ("false", TokenType::False),
            ("for", TokenType::For),
            ("fun", TokenType::Fun),
            ("if", TokenType::If),
            ("nil", TokenType::Nil),
            ("or", TokenType::Or),
            ("print", TokenType::Print),
            ("return", TokenType::Return),
            ("super", TokenType::Super),
            ("this", TokenType::This),
            ("true", TokenType::True),
            ("var", TokenType::Var),
            ("while", TokenType::While),
        ]);

        Lexer {
            input,
            index: 0,
            line: 1,
            reserved_words,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AnalisisError {
    UnrecognizedCharacter(usize, char),
    UnterminatedString(usize),
}

impl<'a> Lexer<'a> {
    fn either(&mut self, target: char, is: TokenType, isnt: TokenType) -> TokenType {
        match self.input.chars().nth(self.index) {
            Some(c) if c == target => {
                self.index += 1;
                is
            }
            _ => isnt,
        }
    }

    fn string(&mut self, initial_index: usize) -> Option<Result<Token<'a>, AnalisisError>> {
        loop {
            self.index += 1;

            match self.input.chars().nth(self.index - 1) {
                Some('"') => {
                    return Some(Ok(Token::new(
                        TokenType::String,
                        &self.input[initial_index..self.index],
                        Literal::String(
                            self.input[(initial_index + 1)..(self.index - 1)].to_string(),
                        ),
                        self.line,
                    )))
                }
                Some('\n') => {
                    self.line += 1;
                }
                Some(_) => continue,
                None => {
                    self.index -= 1;
                    return Some(Err(AnalisisError::UnterminatedString(self.line)));
                }
            };
        }
    }

    fn number(&mut self, initial_index: usize) -> Token<'a> {
        let mut has_dot = false;
        let gen_token = |i: usize| {
            let slice = &self.input[initial_index..i];

            let number = slice
                .parse::<f64>()
                .expect("We know everything before is a number");

            return Token::new(TokenType::Number, slice, Literal::Number(number), self.line);
        };

        loop {
            self.index += 1;
            match self.input.chars().nth(self.index - 1) {
                Some('.') => {
                    if !has_dot {
                        has_dot = true;
                    } else {
                        self.index -= 1;
                        return gen_token(self.index);
                    }
                }
                Some(c) if c.is_numeric() => continue,
                Some(_) => {
                    self.index -= 1;
                    return gen_token(self.index);
                }
                None => {
                    self.index -= 1;
                    return gen_token(self.index);
                }
            }
        }
    }

    fn identifier(&mut self, initial_index: usize) -> Token<'a> {
        loop {
            self.index += 1;

            match self.input.chars().nth(self.index - 1) {
                Some(c) if c.is_alphanumeric() || c == '_' => continue,
                _ => break,
            }
        }

        self.index -= 1;
        let slice = &self.input[initial_index..self.index];

        let token_type = self
            .reserved_words
            .get(slice)
            .unwrap_or(&TokenType::Identifier);

        Token::new(token_type.clone(), slice, Literal::Null, self.line)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, AnalisisError>;

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.input.chars().count();

        loop {
            if let Some(c) = self.input.chars().nth(self.index) {
                let initial_index = self.index;
                self.index += 1;

                if c.is_whitespace() {
                    self.line += (c == '\n') as usize;
                    continue;
                }

                let token_type: TokenType = match c {
                    '(' => TokenType::LeftParenthesis,
                    ')' => TokenType::RightParenthesis,
                    '{' => TokenType::LeftBracket,
                    '}' => TokenType::RightBracket,
                    '*' => TokenType::Asterisk,
                    '.' => TokenType::Dot,
                    ',' => TokenType::Comma,
                    '+' => TokenType::Plus,
                    '-' => TokenType::Minus,
                    ';' => TokenType::SemiColon,
                    '!' => self.either('=', TokenType::BangEqual, TokenType::Bang),
                    '=' => self.either('=', TokenType::EqualEqual, TokenType::Equal),
                    '>' => self.either('=', TokenType::GreaterEqual, TokenType::Greater),
                    '<' => self.either('=', TokenType::LessEqual, TokenType::Less),
                    '/' => match self.input.chars().nth(self.index) {
                        Some('/') => {
                            loop {
                                self.index += 1;
                                match self.input.chars().nth(self.index) {
                                    Some('\n') => break,
                                    None => break,
                                    _ => continue,
                                }
                            }
                            continue;
                        }
                        _ => TokenType::Slash,
                    },
                    '"' => return self.string(initial_index),
                    c if c.is_numeric() => return Some(Ok(self.number(initial_index))),
                    c if c.is_alphabetic() || c == '_' => {
                        return Some(Ok(self.identifier(initial_index)))
                    }
                    c => return Some(Err(AnalisisError::UnrecognizedCharacter(self.line, c))),
                };

                let token = Token::new(
                    token_type,
                    &self.input[initial_index..self.index],
                    Literal::Null,
                    self.line,
                );

                return Some(Ok(token));
            } else {
                if self.index == len {
                    self.index += 1;

                    return Some(Ok(Token::new(
                        TokenType::EndOfFile,
                        "",
                        Literal::Null,
                        self.line,
                    )));
                }

                return None;
            }
        }
    }
}
