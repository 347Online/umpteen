use crate::repr::{
    expr::Expr,
    token::{Token, TokenType},
    value::Value,
};

use super::env::Environment;

pub struct Parser<'t, 'e> {
    tokens: &'t Vec<Token<'t>>,
    globals: &'e mut Environment,
}

impl<'t, 'e> Parser<'t, 'e> {
    pub fn new(tokens: &'t Vec<Token<'t>>, globals: &'e mut Environment) -> Self {
        Parser { tokens, globals }
    }

    pub fn parse(&'e mut self) -> Expr {
        // for tk in self.tokens {
        //     Self::parse_token(tk);
        // }
        // Expr::Value(Value::Empty)
        self.parse_token(&self.tokens[0])
    }

    fn parse_token(&'e mut self, tk: &Token<'t>) -> Expr<'t, 'e> {
        use TokenType as TT;
        match tk.kind {
            TT::Number => Expr::Value(Value::Number(tk.lexeme.parse().unwrap())),
            TT::String => Expr::Value(tk.lexeme.into()),
            TT::Identifier => Expr::Ident {
                name: tk.lexeme,
                env: self.globals,
            },
            // TT::Identifier => Expr::Ident {
            //     name: tk.lexeme,
            //     env: self.globals,
            // },
            TT::Let => {
                todo!()
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
        let mut globals = Environment::default();
        globals.assign("x", 10.0.into());
        dbg!(globals.get("x"), globals.get("y"));
        let tokens = Lexer::new("x = 10;").scan();
        let mut parser = Parser::new(&tokens, &mut globals);
        let result = parser.parse().eval().unwrap();
        dbg!(globals);
        println!("Result: {} ({:?})", result, result);
    }
}
