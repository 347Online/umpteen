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

        ast.push(Stmt::Return(None));

        #[cfg(debug_assertions)]
        dbg!(&ast);

        Ok(ast)
    }

    fn consume(&mut self) -> Result<Token<'p>, ParseError> {
        let tk = self.peek()?;
        self.index += 1;
        Ok(tk)
    }

    fn consume_or(&mut self, err: ParseError) -> Result<Token<'p>, ParseError> {
        let tk = self.peek().map_err(|_| err)?;
        self.index += 1;
        Ok(tk)
    }

    fn consume_if(&mut self, kind: TokenType) -> Result<Token<'p>, ParseError> {
        let next = self.peek()?;
        if next.kind == kind {
            self.consume()
        } else {
            Err(ParseError::ExpectedToken(kind))?
        }
    }

    fn peek(&self) -> Result<Token<'p>, ParseError> {
        let index = self.index;
        self.tokens
            .get(index)
            .copied()
            .ok_or(ParseError::UnexpectedEof)
    }

    fn parse_stmt(&mut self) -> Result<Stmt<'p>, ParseError> {
        let token = self.consume_or(ParseError::ExpectedStatement)?;

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
