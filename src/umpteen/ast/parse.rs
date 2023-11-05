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

        ast.push(Stmt::Return(None));

        #[cfg(debug_assertions)]
        dbg!(&ast);

        Ok(ast)
    }

    fn consume(&mut self) -> Result<Token<'p>, SyntaxError> {
        self.consume_or(SyntaxError::UnexpectedEof)
    }

    fn consume_or(&mut self, err: SyntaxError) -> Result<Token<'p>, SyntaxError> {
        let tk = self.tokens.get(self.index).ok_or(err)?;
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
        let token = self.consume_or(SyntaxError::ExpectedStatement)?;

        let stmt = match token.kind {
            TokenType::Print => {
                let expr = self.parse_expr()?;
                Stmt::Print(expr)
            }
            TokenType::Let => {
                let ident = self.consume_if(TokenType::Identifier)?;
                self.consume_if(TokenType::Equal)?;
                let expr = Box::new(self.parse_expr()?);
                let assign = Expr::Assign {
                    name: ident.lexeme,
                    expr,
                };
                Stmt::Expr(assign)
            }
            TokenType::Identifier => todo!(),

            TokenType::Error => todo!(),

            kind => Err(SyntaxError::UnexpectedToken(kind))?,
        };
        self.consume_if(TokenType::Semicolon)?;
        Ok(stmt)
    }

    fn parse_expr(&mut self) -> Result<Expr<'p>, SyntaxError> {
        let token = self.consume_or(SyntaxError::ExpectedExpression)?;

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

            kind => Err(SyntaxError::UnexpectedToken(kind)),
        }
    }
}
