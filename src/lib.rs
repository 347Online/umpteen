pub mod error;
pub mod token;

use error::*;
use token::*;

pub fn lex(source: &str) -> Result<Vec<Token>, UmpError> {
    let source: Vec<char> = source.to_string().chars().collect();
    let len = source.len();
    let mut pos = 0;
    let mut line = 1;
    let mut tokens = vec![];

    while pos < len {
        let c = source.get(pos);
        pos += 1;

        match c {
            Some(c) => {
                macro_rules! token {
                    ($k:tt) => {
                        tokens.push(Token::new(TokenType::$k, &String::from(*c), line))
                    };
                }

                match c {
                    ';' => token!(Semicolon),
                    '=' => token!(Equal),

                    '0'..='9' => (),

                    '\n' => {
                        line += 1;
                    }
                    _ => todo!(),
                }
            }
            None => {
                return Err(UmpError::new(
                    "Unexpected end of file",
                    UmpErrorType::UnexpectedEof,
                    line,
                ));
            }
        }
    }

    Ok(tokens)
}
