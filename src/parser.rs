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
    pub fn parse(&mut self) -> Vec<Result<Expr<'a>, ParserError>> {
        let mut exprs = Vec::new();

        for token in self.tokens.iter() {
            let expr = match token.token_type {
                TokenType::True => Expr::Bool(true),
                TokenType::False => Expr::Bool(false),
                TokenType::Nil => Expr::Nil,
                TokenType::Number => Expr::Number(match token.literal {
                    crate::token::Literal::Number(n) => n,
                    _ => unreachable!(),
                }),
                TokenType::String => Expr::String(match token.literal {
                    crate::token::Literal::String(s) => s,
                    _ => unreachable!(),
                }),
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
