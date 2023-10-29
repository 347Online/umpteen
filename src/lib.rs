pub mod error;
pub mod token;

use error::*;
use token::*;

pub fn lex(source: &str) -> UmpResult<Vec<Token>> {
    let mut source = source.chars().peekable();
    let mut line = 1;
    let mut tokens = vec![];

    while let Some(c) = source.next() {
        macro_rules! token {
            ($($k:tt)+) => {
                tokens.push(Token::new(TokenType::$($k)+, &String::from(c), line))
            };
        }
        source.peek(); // TODO: remove this
        match c {
            ';' => token!(Semicolon),
            '=' => token!(Equal),

            '\n' => {
                token!(Newline);
                line += 1;
            }
            _ => token!(Error("Unexpected token")),
        }
    }

    Ok(tokens)
}
