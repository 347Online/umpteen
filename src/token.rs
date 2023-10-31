use std::fmt::{Display, Write};

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Semicolon,
    Newline,
    Equal,

    Let,
    Print,

    Number,
    String,
    Identifier,

    Error,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    kind: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(kind: TokenType, lexeme: String, line: usize) -> Self {
        Self { kind, lexeme, line }
    }

    pub fn kind(&self) -> TokenType {
        self.kind
    }

    pub fn lexeme(&self) -> String {
        self.lexeme.clone()
    }
}

impl Display for Token {
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
