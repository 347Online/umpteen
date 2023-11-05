use crate::{
    error::{report_line, CompilerError, SyntaxError},
    token::{Token, TokenType},
    value::{self, Value},
};

use super::{Expr, Stmt};

pub enum AstNode<'a> {
    Stmt(Stmt<'a>),
    Expr(Expr<'a>),
}

pub struct Parser<'p> {
    tokens: Vec<Token<'p>>,
    index: usize,
}

impl<'p> Parser<'p> {
    pub fn new(tokens: Vec<Token<'p>>) -> Self {
        Parser { tokens, index: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt<'p>>, SyntaxError> {
        let mut ast = vec![];
        loop {
            let stmt = self.parse_stmt()?;
            ast.push(stmt);
        }

        Ok(ast)
    }

    fn get_token<E>(&mut self, err: E) -> Result<Token<'p>, E> {
        let token = self.tokens.get(self.index).ok_or(err)?;
        self.index += 1;
        Ok(*token)
    }

    fn parse_stmt(&mut self) -> Result<Stmt<'p>, SyntaxError> {
        let token = self.get_token(SyntaxError::UnexpectedEof)?;

        match token.kind {
            TokenType::Print => {
                let expr = self.parse_expr()?;
                Ok(Stmt::Print(expr))
            }
            TokenType::Let => todo!(),
            TokenType::Identifier => todo!(),

            TokenType::Error => todo!(),

            _ => Err(SyntaxError::ExpectedStatement),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr<'p>, SyntaxError> {
        let token = self.get_token(SyntaxError::UnexpectedEof)?;

        match token.kind {
            TokenType::Number => {
                let num: f64 = token
                    .lexeme
                    .parse()
                    .map_err(|e| SyntaxError::Other(Box::new(e)))?;
                Ok(Expr::Value(Value::Number(num)))
            }
            TokenType::String => Ok(Expr::Value(Value::from(token.lexeme))),
            TokenType::Identifier => todo!(),

            _ => Err(SyntaxError::ExpectedExpression),
        }
    }
}
