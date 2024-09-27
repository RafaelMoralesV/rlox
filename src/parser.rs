use crate::{
    expression::Expr,
    token::{Token, TokenType},
};

#[derive(Clone)]
pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    index: usize,
}

pub enum ParserError {
    UnexpectedToken,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self { tokens, index: 0 }
    }

    pub fn parse(&'a mut self) -> Vec<Result<Expr<'a>, ParserError>> {
        let mut exprs = Vec::new();

        for token in self.tokens.iter() {
            use TokenType as TT;

            let expr = match token.token_type {
                TT::True | TT::False | TT::Nil | TT::Number | TT::String => Expr::Literal(token),
                TokenType::EndOfFile => return exprs,
                _ => {
                    exprs.push(Err(ParserError::UnexpectedToken));
                    continue;
                }
            };

            exprs.push(Ok(expr));
        }

        exprs
    }

    fn expression(&'a mut self) -> Expr {
        self.equality()
    }

    fn equality(&'a mut self) -> Expr {
        let expr = self.comparison();

        while self.matches(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = unsafe { self.tokens.get_unchecked(self.index - 1).clone() };

            let right = self.comparison();

            expr = Expr::Binary {
                operator,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        expr
    }

    fn comparison(&'a mut self) -> Expr {
        unimplemented!()
    }

    fn primary(&'a mut self) -> Expr {
        unimplemented!()
    }

    fn matches(&mut self, token_types: Vec<TokenType>) -> bool {
        if let Some(token) = self.tokens.get(self.index) {
            if token_types.contains(&token.token_type) {
                self.index += 1;
                return true;
            }
        }

        false
    }
}
