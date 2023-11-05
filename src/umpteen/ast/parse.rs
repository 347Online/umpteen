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
            if self.index == self.tokens.len() {
                break;
            }
        }

        Ok(ast)
    }

    fn consume(&mut self) -> Result<Token<'p>, SyntaxError> {
        let tk = self
            .tokens
            .get(self.index)
            .ok_or(SyntaxError::UnexpectedEof)?;
        self.index += 1;
        Ok(*tk)
    }

    fn consume_if(&mut self, kind: TokenType) -> Result<Token<'p>, SyntaxError> {
        match self.tokens.get(self.index) {
            Some(tk) => {
                if tk.kind == kind {
                    self.index += 1;
                    Ok(*tk)
                } else {
                    Err(SyntaxError::ExpectedToken(kind))?
                }
            }
            None => Err(SyntaxError::UnexpectedEof),
        }
    }

    fn parse_stmt(&mut self) -> Result<Stmt<'p>, SyntaxError> {
        let token = self.consume()?;

        let stmt = match token.kind {
            TokenType::Print => {
                let expr = self.parse_expr()?;
                Stmt::Print(expr)
            }
            TokenType::Let => todo!(),
            TokenType::Identifier => todo!(),

            TokenType::Error => todo!(),

            kind => Err(SyntaxError::ExpectedStatement(kind))?,
        };
        let semi = self.consume_if(TokenType::Semicolon)?;
        Ok(stmt)
    }

    fn parse_expr(&mut self) -> Result<Expr<'p>, SyntaxError> {
        let token = self.consume()?;

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
