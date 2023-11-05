use crate::{
    error::ParseError,
    token::{Token, TokenType},
    value::{Value, self},
};

use super::{Expr, Stmt};

pub enum AstNode<'a> {
    Syntax(TokenType),
    Constant(Value),
    Stmt(Stmt<'a>),
    Expr(Expr<'a>),
}

pub struct Parser<'p> {
    tokens: Vec<Token<'p>>,
}

impl<'p> Parser<'p> {
    pub fn new(tokens: Vec<Token<'p>>) -> Self {
        Parser { tokens }
    }

    pub fn parse(mut self) -> Result<Vec<Stmt<'p>>, ParseError> {
        let tokens = std::mem::take(&mut self.tokens);
        for token in tokens {
            self.parse_token(token);
        }

        todo!()
    }

    fn parse_token(&mut self, token: Token) -> Result<AstNode, ParseError> {
        let kind = token.kind;
        let node = match token.kind {
            TokenType::Semicolon => AstNode::Syntax(kind),
            TokenType::Print => AstNode::Syntax(TokenType::Print),
            TokenType::String => AstNode::Constant(Value::from(token.lexeme)),

            TokenType::Identifier => todo!(),
            TokenType::Equal => todo!(),
            TokenType::Let => todo!(),

            TokenType::Number => {
                let num: f64 = match token.lexeme.parse() {
                    Ok(x) => x,
                    Err(e) => {
                        let lexeme = token.lexeme.to_string();
                        Err(ParseError::Other(Box::new(e)))?
                    }
                };
                AstNode::Expr(Expr::Value(Value::Number(num)))
            }

            TokenType::Error => {
                let lexeme = token.lexeme.to_string();
                Err(ParseError::Lexeme(lexeme))?
            }
        };
        Ok(node)
    }
}
