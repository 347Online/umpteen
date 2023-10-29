#[derive(Debug)]
pub enum TokenType {
    Let,
    Identifier,
    Semicolon,
    Newline,
    Equal,
    Number,
}

#[derive(Debug)]
pub struct Token {
    kind: TokenType,
    lexeme: String,
    line: u32,
    // column: u32,
}

impl Token {
    pub fn new(kind: TokenType, lexeme: &str, line: u32) -> Self {
        Self {
            kind,
            lexeme: lexeme.to_string(),
            line,
        }
    }
}
