use std::fmt::{Display, Write};

#[derive(Debug)]
pub enum TokenType {
    Let,
    Identifier,
    Semicolon,
    Newline,
    Equal,
    Number,
    Error(&'static str),
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Token {
    kind: TokenType,
    lexeme: String,
    line: u32,
    column: u32,
}

impl Token {
    pub fn new(kind: TokenType, lexeme: &str, line: u32) -> Self {
        Self {
            kind,
            lexeme: lexeme.to_string(),
            line,
            column: 0,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            TokenType::Error(s) => write!(f, "Error [{}:{}]: {s}", self.line, self.column),
            _ => write!(f, "{}({:?})", self.kind, self.lexeme),
        }
    }
}

pub fn print_tokens(tokens: Vec<Token>) -> std::fmt::Result {
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
