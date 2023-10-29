pub mod error;
pub mod token;

use error::*;
use token::*;

pub fn lex(source: &str) -> UmpResult<Vec<Token>> {
    let mut source = source.chars().peekable();
    let mut line = 1;
    let mut col = 1;
    let mut tokens = vec![];

    while let Some(c) = source.next() {
        macro_rules! token {
            ($k:tt, $s:expr) => {
                tokens.push(Token::new(TokenType::$k, $s, line))
            };

            ($($k:tt)+) => {
                tokens.push(Token::new(TokenType::$($k)+, &String::from(c), line))
            };
        }
        match c {
            c if c.is_ascii_whitespace() => (),

            ';' => token!(Semicolon),
            '=' => token!(Equal),

            '\n' => {
                token!(Newline);
                line += 1;
                col = 1;
            }

            c if c.is_ascii_digit() => {
                let mut num_str = String::from(c);
                while let Some(c) = source.next_if(|x| x.is_ascii_digit()) {
                    num_str.push(c);
                }
                token!(Number, &num_str)
            }

            c if c.is_ascii_alphabetic() || c == '_' => {
                let mut ident_str = String::from(c);
                while let Some(c) = source.next_if(|x| x.is_ascii_alphanumeric() || *x == '_') {
                    ident_str.push(c);
                }

                match ident_str.as_str() {
                    "let" => token!(Let, &ident_str),
                    _ => token!(Identifier, &ident_str),
                }
            }

            _ => token!(Error("Unexpected token", col)),
        }
        col += 1;
    }

    Ok(tokens)
}
