use crate::token::Token;

pub struct Parser<'p> {
    tokens: Vec<Token<'p>>,
}

impl<'p> Parser<'p> {
    pub fn new(tokens: Vec<Token<'p>>) -> Self {
        Parser { tokens }
    }

    
}
