use std::fmt::{Display, Write};

use crate::error::Line;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Colon,
    Comma,

    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Plus,
    PlusEqual,
    Minus,
    MinusEqual,
    Star,
    StarEqual,
    Slash,
    SlashEqual,
    Percent,
    PercentEqual,

    And,
    Or,

    ThinArrow,
    FatArrow,

    Var,
    Let,
    If,
    Else,
    Loop,
    Break,
    Continue,
    Fnc,
    Return,
    Print,

    True,
    False,
    Empty,
    Number,
    String,
    Identifier,

    Eof,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Token<'t> {
    pub kind: TokenType,
    pub lexeme: &'t str,
    pub line: Line,
}

impl<'t> Token<'t> {
    pub fn new(kind: TokenType, lexeme: &'t str, line: Line) -> Self {
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
        tokens.last().map_or(Line::new(9999), |x| { x.line })
    )?;
    println!("{}", buffer);

    Ok(())
}
