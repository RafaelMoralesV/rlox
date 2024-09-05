use crate::token::{Literal, Token, TokenType};

pub struct Lexer<'a> {
    pub input: &'a str,
    pub index: usize,
    pub line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input,
            index: 0,
            line: 1,
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
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, AnalisisError>;

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.input.chars().count();

        loop {
            if let Some(c) = self.input.chars().nth(self.index) {
                let initial_index = self.index;
                self.index += 1;

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
                    '"' => loop {
                        self.index += 1;

                        match self.input.chars().nth(self.index - 1) {
                            Some(c) if c == '"' => {
                                return Some(Ok(Token::new(
                                    TokenType::String,
                                    &self.input[initial_index..self.index],
                                    Literal::String(
                                        &self.input[(initial_index + 1)..(self.index - 1)],
                                    ),
                                    self.line,
                                )))
                            }
                            Some(c) if c == '\n' => {
                                self.line += 1;
                            }
                            Some(_) => continue,
                            None => {
                                self.index -= 1;
                                return Some(Err(AnalisisError::UnterminatedString(self.line)));
                            }
                        };
                    },
                    '\n' => {
                        self.line += 1;
                        continue;
                    }
                    c if c.is_whitespace() => continue,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_parenthesis() {
        let mut lexer = Lexer::new("(()");

        //assert_eq!(Some(Ok(Token::LeftParenthesis)), lexer.next());
        //assert_eq!(Some(Ok(Token::LeftParenthesis)), lexer.next());
        //assert_eq!(Some(Ok(Token::RightParenthesis)), lexer.next());
        //assert_eq!(Some(Ok(Token::EndOfFile)), lexer.next());
        //assert_eq!(None, lexer.next());
    }

    #[test]
    fn parses_brackets() {
        let mut lexer = Lexer::new("{{}}");

        //assert_eq!(Some(Ok(Token::LeftBracket)), lexer.next());
        //assert_eq!(Some(Ok(Token::LeftBracket)), lexer.next());
        //assert_eq!(Some(Ok(Token::RightBracket)), lexer.next());
        //assert_eq!(Some(Ok(Token::RightBracket)), lexer.next());
        //
        //assert_eq!(Some(Ok(Token::EndOfFile)), lexer.next());
        //assert_eq!(None, lexer.next());
    }

    #[test]
    fn other_one_character_input() {
        let mut lexer = Lexer::new("*.,+-;");

        //assert_eq!(Some(Ok(Token::Asterisk)), lexer.next());
        //assert_eq!(Some(Ok(Token::Dot)), lexer.next());
        //assert_eq!(Some(Ok(Token::Comma)), lexer.next());
        //assert_eq!(Some(Ok(Token::Plus)), lexer.next());
        //assert_eq!(Some(Ok(Token::Minus)), lexer.next());
        //assert_eq!(Some(Ok(Token::SemiColon)), lexer.next());
        //
        //assert_eq!(Some(Ok(Token::EndOfFile)), lexer.next());
        //assert_eq!(None, lexer.next());
    }

    #[test]
    fn lexical_errors() {
        let mut lexer = Lexer::new(",.$(#");

        //assert_eq!(Some(Ok(Token::Comma)), lexer.next());
        //assert_eq!(Some(Ok(Token::Dot)), lexer.next());
        //assert_eq!(
        //    Some(Err(AnalisisError::UnrecognizedCharacter(1, '$'))),
        //    lexer.next()
        //);
        //assert_eq!(Some(Ok(Token::LeftParenthesis)), lexer.next());
        //assert_eq!(
        //    Some(Err(AnalisisError::UnrecognizedCharacter(1, '#'))),
        //    lexer.next()
        //);
        //
        //assert_eq!(Some(Ok(Token::EndOfFile)), lexer.next());
        //assert_eq!(None, lexer.next());
    }
}
