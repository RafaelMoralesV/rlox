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
}
