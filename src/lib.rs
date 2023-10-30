pub mod bytecode;
pub mod chunk;
pub mod error;
pub mod token;
pub mod value;

use bytecode::{Argument, Instruction};
use chunk::Chunk;
use error::{UmpResult, UmpError};
use token::*;

fn lex(source: &str) -> Vec<Token> {
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
            '\n' => {
                token!(Newline);
                line += 1;
                col = 1;
            }

            c if c.is_ascii_whitespace() => (),

            ';' => token!(Semicolon),
            '=' => token!(Equal),

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

    #[cfg(debug_assertions)]
    if let Err(e) = print_tokens(&tokens) {
        eprintln!("{e}")
    }

    tokens
}

pub fn run(program: Vec<Chunk>) -> UmpResult<()> {
    for chunk in program {
        let mut val = None;
        let code = chunk.instructions().iter().peekable();
        let mut args = chunk.data().iter().peekable();
        let constants = chunk.constants();

        for inst in code {
            match inst {
                Instruction::Constant => {
                    let Some(Argument(addr)) = args.next() else {
                        return Err(UmpError::wrong_num_args(1, 0));
                    };
                    let Some(value) = constants.get(*addr as usize) else {
                        return Err(UmpError::missing_value(*addr))
                    };
                    val = Some(value);
                },
                Instruction::Return => {
                    if let Some(value) = val {
                        println!("{value}");
                    }
                },
            }
        }
    }

    Ok(())
}

pub fn exec(source: &str) {
    let tokens = lex(source);
}
