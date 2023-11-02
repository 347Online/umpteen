use crate::repr::{
    expr::Expr,
    token::{Token, TokenType},
    value::Value,
};

pub struct Parser<'t> {
    tokens: &'t Vec<Token<'t>>,
}

impl<'t> Parser<'t> {
    pub fn new(tokens: &'t Vec<Token<'t>>) -> Self {
        Parser { tokens }
    }

    pub fn parse(&self) -> Expr {
        // for tk in self.tokens {
        //     Self::parse_token(tk);
        // }
        // Expr::Value(Value::Empty)
        Self::parse_token(&self.tokens[0])
    }

    fn parse_token(tk: &Token) -> Expr {
        use TokenType as TT;
        match tk.kind {
            TT::Number => Expr::Value(Value::Number(tk.lexeme.parse().unwrap())),
            TT::String => Expr::Value(tk.lexeme.into()),
            TT::Let => todo!(),
            TT::Print => todo!(),
            TT::Identifier => todo!(),
            TT::Equal => todo!(),
            TT::Error => todo!(),
            TT::Semicolon => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::exec::lexer::Lexer;

    use super::Parser;

    #[test]
    fn parse_let_x_equal_10() {
        let tokens = Lexer::new("\"Hello world 23423421 \"").scan();
        let parser = Parser::new(&tokens);
        let result = parser.parse().eval().unwrap();
        println!("Result: {} ({:?})", result, result);
    }
}
