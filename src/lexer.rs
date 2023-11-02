pub struct Lexer {
    source: String,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer { source }
    }
}
