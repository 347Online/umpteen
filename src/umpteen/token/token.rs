use std::fmt::{Display, Write};

use super::TokenType;

#[derive(Debug)]
pub struct Token<'t> {
    pub kind: TokenType,
    pub lexeme: &'t str,
    pub line: usize,
}

impl<'t> Token<'t> {
    pub fn new(kind: TokenType, lexeme: &'t str, line: usize) -> Self {
        Self { kind, lexeme, line }
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenType as TT;
        match self.kind {
            TT::Number | TT::Identifier => write!(f, "{}({:?})", self.kind, self.lexeme),
            _ => write!(f, "{}", self.kind),
        }
    }
}

pub fn print_tokens(tokens: &Vec<Token>) -> std::fmt::Result {
    let mut buffer = String::new();
    write!(&mut buffer, "Tokens: [")?;
    for (i, tk) in tokens.iter().enumerate() {
        write!(&mut buffer, "{}", tk)?;
        if i < tokens.len() - 1 {
            write!(&mut buffer, ", ")?;
        }
    }
    write!(
        &mut buffer,
        "] — EOF line {}",
        tokens.last().map_or(0, |x| { x.line })
    )?;
    println!("{}", buffer);

    Ok(())
}