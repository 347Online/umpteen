use crate::{
    error::ParseError,
    repr::{
        ast::{expr::Expr, stmt::Stmt},
        token::{Token, TokenType},
        value::Value,
    },
};

pub enum AstNode<'a> {
    Stmt(Stmt<'a>),
    Expr(Expr<'a>),
}

pub type Ast<'a> = Vec<Stmt<'a>>;

pub struct Parser<'p> {
    tokens: Vec<Token<'p>>,
    index: usize,
}

impl<'p> Parser<'p> {
    pub fn new(tokens: Vec<Token<'p>>) -> Self {
        Parser { tokens, index: 0 }
    }

    pub fn parse(&mut self) -> Result<Ast<'p>, ParseError> {
        let mut ast = vec![];
        loop {
            let stmt = self.parse_stmt()?;
            ast.push(stmt);
            if self.index == self.tokens.len() {
                break;
            }
        }

        ast.push(Stmt::Exit(None));

        Ok(ast)
    }

    fn consume(&mut self) -> Result<Token<'p>, ParseError> {
        if let Some(tk) = self.tokens.get(self.index) {
            self.index += 1;
            Ok(*tk)
        } else {
            Err(ParseError::UnexpectedEof)
        }
    }

    fn consume_or(&mut self, err: ParseError) -> Result<Token<'p>, ParseError> {
        self.consume().map_err(|_| err)
    }

    fn consume_if(&mut self, kind: TokenType) -> Result<Token<'p>, ParseError> {
        if self.peek().is_ok_and(|x| x == kind) {
            self.consume()
        } else {
            Err(ParseError::ExpectedToken(kind))
        }
    }

    fn peek(&self) -> Result<TokenType, ParseError> {
        let index = self.index;
        self.tokens
            .get(index)
            .copied()
            .map(|tk| tk.kind)
            .ok_or(ParseError::UnexpectedEof)
    }

    fn check(&self, kind: TokenType) -> bool {
        self.peek().is_ok_and(|x| x == kind)
    }

    fn parse_stmt(&mut self) -> Result<Stmt<'p>, ParseError> {
        let token = self.consume_or(ParseError::ExpectedStatement)?;

        let stmt = match token.kind {
            TokenType::Print => {
                let expr = self.parse_expr()?;
                Stmt::Print(expr)
            }
            TokenType::Let => {
                let name = self.consume_if(TokenType::Identifier)?.lexeme;

                if self.consume_if(TokenType::Equal).is_ok() {
                    let expr = Box::new(self.parse_expr()?);
                    Stmt::Declare(Expr::Ident { name }, Some(*expr))
                } else if self.check(TokenType::Semicolon) {
                    Stmt::Declare(Expr::Ident { name }, None)
                } else {
                    Err(ParseError::ExpectedToken(TokenType::Semicolon))?
                }
            }

            TokenType::Error => todo!(),

            kind => Err(ParseError::UnexpectedToken(kind))?,
        };
        self.consume_if(TokenType::Semicolon)?;
        Ok(stmt)
    }

    fn parse_expr(&mut self) -> Result<Expr<'p>, ParseError> {
        let token = self.consume_or(ParseError::ExpectedExpression)?;

        match token.kind {
            TokenType::Number => {
                let num: f64 = token.lexeme.parse()?;
                // .map_err(|e| ParseError::Other(Box::new(e)))?;
                Ok(Expr::Constant(Value::Number(num)))
            }
            TokenType::String => Ok(Expr::Constant(Value::from(token.lexeme))),
            TokenType::Identifier => todo!(),

            kind => Err(ParseError::UnexpectedToken(kind)),
        }
    }
}
