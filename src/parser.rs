use crate::{
    expression::Expr,
    token::{Token, TokenType},
};

pub struct Parser<'a> {
    pub tokens: Vec<Token<'a>>,
}

pub enum ParserError {
    UnexpectedToken,
}

impl<'a> Parser<'a> {
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
}
