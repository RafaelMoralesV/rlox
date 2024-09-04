use crate::token::Token;

pub struct Lexer<'a> {
    input: &'a str,
    index: usize,
    line: usize,
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
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, AnalisisError>;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.input.chars().nth(self.index);
        self.index += 1;

        if self.index == self.input.len() + 1 {
            return Some(Ok(Token::EndOfFile));
        }

        match c {
            Some('(') => Some(Ok(Token::LeftParenthesis)),
            Some(')') => Some(Ok(Token::RightParenthesis)),
            Some('{') => Some(Ok(Token::LeftBracket)),
            Some('}') => Some(Ok(Token::RightBracket)),
            Some('*') => Some(Ok(Token::Asterisk)),
            Some('.') => Some(Ok(Token::Dot)),
            Some(',') => Some(Ok(Token::Comma)),
            Some('+') => Some(Ok(Token::Plus)),
            Some('-') => Some(Ok(Token::Minus)),
            Some(';') => Some(Ok(Token::SemiColon)),
            Some('\n') => {
                self.line += 1;
                self.next()
            }
            Some(c) if c.is_whitespace() => self.next(),
            Some(c) => Some(Err(AnalisisError::UnrecognizedCharacter(self.line, c))),
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

        assert_eq!(Some(Ok(Token::LeftParenthesis)), lexer.next());
        assert_eq!(Some(Ok(Token::LeftParenthesis)), lexer.next());
        assert_eq!(Some(Ok(Token::RightParenthesis)), lexer.next());
        assert_eq!(Some(Ok(Token::EndOfFile)), lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn parses_brackets() {
        let mut lexer = Lexer::new("{{}}");

        assert_eq!(Some(Ok(Token::LeftBracket)), lexer.next());
        assert_eq!(Some(Ok(Token::LeftBracket)), lexer.next());
        assert_eq!(Some(Ok(Token::RightBracket)), lexer.next());
        assert_eq!(Some(Ok(Token::RightBracket)), lexer.next());

        assert_eq!(Some(Ok(Token::EndOfFile)), lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn other_one_character_input() {
        let mut lexer = Lexer::new("*.,+-;");

        assert_eq!(Some(Ok(Token::Asterisk)), lexer.next());
        assert_eq!(Some(Ok(Token::Dot)), lexer.next());
        assert_eq!(Some(Ok(Token::Comma)), lexer.next());
        assert_eq!(Some(Ok(Token::Plus)), lexer.next());
        assert_eq!(Some(Ok(Token::Minus)), lexer.next());
        assert_eq!(Some(Ok(Token::SemiColon)), lexer.next());

        assert_eq!(Some(Ok(Token::EndOfFile)), lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn lexical_errors() {
        let mut lexer = Lexer::new(",.$(#");

        assert_eq!(Some(Ok(Token::Comma)), lexer.next());
        assert_eq!(Some(Ok(Token::Dot)), lexer.next());
        assert_eq!(
            Some(Err(AnalisisError::UnrecognizedCharacter(1, '$'))),
            lexer.next()
        );
        assert_eq!(Some(Ok(Token::LeftParenthesis)), lexer.next());
        assert_eq!(
            Some(Err(AnalisisError::UnrecognizedCharacter(1, '#'))),
            lexer.next()
        );

        assert_eq!(Some(Ok(Token::EndOfFile)), lexer.next());
        assert_eq!(None, lexer.next());
    }
}
