use std::collections::VecDeque;

use crate::{
    expression::{BinaryOperator, Expr},
    token::{Literal, Token, TokenType},
};

#[derive(Clone)]
pub struct Parser<'a> {
    tokens: VecDeque<Token<'a>>,
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken,
    MissingToken,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        let mut tokens: VecDeque<_> = tokens.into();
        if let Some(last) = tokens.pop_back() {
            // If the last token is EOF, we delete it.
            if last.token_type != TokenType::EndOfFile {
                tokens.push_back(last);
            }
        }

        Self { tokens }
    }

    pub fn parse(&'a mut self) -> Vec<Result<Expr, ParserError>> {
        let mut exprs = Vec::new();

        exprs.push(self.expression());

        exprs
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.comparison()?;

        // Esto tiene que estar mal.
        while self.matches_type(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self
                .tokens
                .pop_front()
                .expect("This can't be None, we just checked.")
                .token_type
                .try_into()
                .expect("This is a valid Token Type");

            let right = Box::new(self.comparison()?);

            expr = Expr::Binary {
                operator,
                left: Box::new(expr),
                right,
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.term()?;

        while self.matches_type(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self
                .tokens
                .pop_front()
                .expect("This can't be None, we just checked.")
                .token_type
                .try_into()
                .expect("This is a valid Token Type");

            let right = Box::new(self.term()?);

            expr = Expr::Binary {
                operator,
                left: Box::new(expr),
                right,
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.factor()?;

        while self.matches_type(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self
                .tokens
                .pop_front()
                .expect("This can't be None, we just checked.")
                .token_type
                .try_into()
                .expect("This is a valid Token Type");

            let right = Box::new(self.factor()?);

            expr = Expr::Binary {
                operator,
                left: Box::new(expr),
                right,
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary()?;

        while self.matches_type(vec![TokenType::Slash, TokenType::Asterisk]) {
            let operator = self
                .tokens
                .pop_front()
                .expect("This can't be None, we just checked.")
                .token_type
                .try_into()
                .expect("This is a valid Token Type");

            let right = Box::new(self.unary()?);

            expr = Expr::Binary {
                operator,
                left: Box::new(expr),
                right,
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        if self.matches_type(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self
                .tokens
                .pop_front()
                .expect("This can't be None, we just checked.")
                .token_type
                .try_into()
                .expect("This is a valid Token Type");

            let right = Box::new(self.unary()?);

            return Ok(Expr::Unary { operator, right });
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        let token = self.tokens.pop_front().ok_or(ParserError::MissingToken)?;

        if token.token_type == TokenType::False {
            return Ok(Expr::Literal(Literal::False));
        }

        if token.token_type == TokenType::True {
            return Ok(Expr::Literal(Literal::True));
        }

        if token.token_type == TokenType::Nil {
            return Ok(Expr::Literal(Literal::Null));
        }

        if token.token_type == TokenType::Number || token.token_type == TokenType::String {
            return Ok(Expr::Literal(token.literal.to_owned()));
        }

        if token.token_type == TokenType::LeftParenthesis {
            let expr = Box::new(self.expression()?);

            if self
                .tokens
                .pop_front()
                .ok_or(ParserError::MissingToken)?
                .token_type
                != TokenType::RightParenthesis
            {
                return Err(ParserError::UnexpectedToken);
            }

            return Ok(Expr::Grouping(expr));
        }

        Err(ParserError::UnexpectedToken)
    }

    fn matches_type(&mut self, types: Vec<TokenType>) -> bool {
        match self.tokens.pop_front() {
            None => false,
            Some(t) => {
                let answer = types.contains(&t.token_type);
                self.tokens.push_front(t);

                answer
            }
        }
    }
}
