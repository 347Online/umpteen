pub enum UmpErrorType {
    Generic
}

pub struct UmpError {
    message: String,
    kind: UmpErrorType
}

impl UmpError {
    pub fn new(message: &str, kind: UmpErrorType) -> Self {
        Self {
            message: message.to_string(),
            kind,
        }
    }

    pub fn generic() -> Self {
        Self::new("An unknown error occurred", UmpErrorType::Generic)
    }
}

pub enum TokenType {
    Let,
    Identifier,
    Semicolon,
    Newline,
    Equals,
    Number,
}

pub struct Token {
    lexeme: String,
    kind: TokenType,
    line: u32,
    // column: u32,
}

pub fn lex(source: &str) -> Result<Vec<Token>, UmpError> {
    

    Err(UmpError::generic())
}