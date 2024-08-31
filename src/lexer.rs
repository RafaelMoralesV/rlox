use crate::token::Token;

pub struct Lexer<'a> {
    input: &'a str,
    index: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input, index: 0 }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.input.chars().nth(self.index);
        self.index += 1;

        if self.index == self.input.len() {
            return Some(Token::EndOfFile);
        }

        match c {
            Some('(') => Some(Token::LeftParenthesis),
            Some(')') => Some(Token::RightParenthesis),
            Some(_) => None,
            None => None,
        }
    }
}
