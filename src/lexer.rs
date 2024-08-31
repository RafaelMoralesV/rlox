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

        if self.index == self.input.len() + 1 {
            return Some(Token::EndOfFile);
        }

        match c {
            Some('(') => Some(Token::LeftParenthesis),
            Some(')') => Some(Token::RightParenthesis),
            Some('{') => Some(Token::LeftBracket),
            Some('}') => Some(Token::RightBracket),
            Some('*') => Some(Token::Asterisk),
            Some('.') => Some(Token::Dot),
            Some(',') => Some(Token::Comma),
            Some('+') => Some(Token::Plus),
            Some('-') => Some(Token::Minus),
            Some(';') => Some(Token::SemiColon),
            Some(_) => None,
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_parenthesis() {
        let mut lexer = Lexer::new("(()");

        assert_eq!(Some(Token::LeftParenthesis), lexer.next());
        assert_eq!(Some(Token::LeftParenthesis), lexer.next());
        assert_eq!(Some(Token::RightParenthesis), lexer.next());
        assert_eq!(Some(Token::EndOfFile), lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn parses_brackets() {
        let mut lexer = Lexer::new("{{}}");

        assert_eq!(Some(Token::LeftBracket), lexer.next());
        assert_eq!(Some(Token::LeftBracket), lexer.next());
        assert_eq!(Some(Token::RightBracket), lexer.next());
        assert_eq!(Some(Token::RightBracket), lexer.next());

        assert_eq!(Some(Token::EndOfFile), lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn other_one_character_input() {
        let mut lexer = Lexer::new("*.,+-;");

        assert_eq!(Some(Token::Asterisk), lexer.next());
        assert_eq!(Some(Token::Dot), lexer.next());
        assert_eq!(Some(Token::Comma), lexer.next());
        assert_eq!(Some(Token::Plus), lexer.next());
        assert_eq!(Some(Token::Minus), lexer.next());
        assert_eq!(Some(Token::SemiColon), lexer.next());

        assert_eq!(Some(Token::EndOfFile), lexer.next());
        assert_eq!(None, lexer.next());
    }
}
