use crate::repr::{
    expr::Expr,
    token::{Token, TokenType},
    value::Value,
};

use super::env::Environment;

pub struct Parser<'t> {
    tokens: &'t [Token<'t>],
    index: usize,
    len: usize,
}

impl<'t> Parser<'t> {
    pub fn new(tokens: &'t [Token<'t>]) -> Self {
        Parser {
            tokens,
            index: 0,
            len: tokens.len(),
        }
    }

    pub fn parse(mut self, globals: &Environment) -> Expr<'t, '_> {
        let len = self.tokens.len();
        while self.index < len {
            let token = &self.tokens[self.index];
            self.parse_token(token, globals);
        }

        todo!()
    }

    fn parse_token<'e>(&'e mut self, tk: &Token<'t>, env: &Environment) -> Box<Expr<'t, '_>> {
        use TokenType as TT;
        match tk.kind {
            TT::Number => Expr::Value(Value::Number(tk.lexeme.parse().unwrap())),
            TT::String => Expr::Value(tk.lexeme.into()),
            TT::Identifier => Expr::Ident {
                name: tk.lexeme,
                env: &mut env,
            },
            TT::Let => {
                let expr = self.parse_token(tk, &env);
                Expr::Assign {
                    name: tk.lexeme,
                    env: &mut env,
                    expr,
                }
            }
            TT::Print => todo!(),
            TT::Equal => todo!(),
            TT::Error => todo!(),
            TT::Semicolon => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::exec::{env::Environment, lexer::Lexer};

    use super::Parser;

    #[test]
    fn parse_let_x_equal_10() {
        let globals = &mut Environment::default();
        let tokens = dbg!(Lexer::new("x = 10").scan());
        let mut parser = Parser::new(&tokens);
        let result = parser.parse(&globals);
        dbg!(globals);
        println!("Result: ({:?})", result);
    }
}
