pub mod chunk;
pub mod error;
pub mod instr;
pub mod token;
pub mod value;

use chunk::Chunk;
use error::Error;
use instr::Instruction;
use token::*;
use value::Value;

pub type Result<T> = std::result::Result<T, Error>;

fn lex(source: &str) -> Result<Vec<Token>> {
    let mut source = source.chars().peekable();
    let mut line = 1;
    let mut col = 0;
    let mut tokens = vec![];

    while let Some(c) = source.next() {
        macro_rules! token {
            ($k:tt, $s:expr) => {
                tokens.push(Token::new(TokenType::$k, $s, line))
            };
            ($k:tt) => {
                tokens.push(Token::new(TokenType::$k, String::from(c), line))
            };
        }
        match c {
            ';' => token!(Semicolon),
            '=' => token!(Equal),

            '\n' => {
                token!(Newline);
                line += 1;
                col = 1;
            }

            c if c.is_ascii_whitespace() => (),

            c if c.is_ascii_digit() => {
                let mut num_str = String::from(c);
                while let Some(c) = source.next_if(|x| x.is_ascii_digit()) {
                    num_str.push(c);
                }
                token!(Number, num_str)
            }

            c if c.is_ascii_alphabetic() || c == '_' => {
                let mut ident_str = String::from(c);
                while let Some(c) = source.next_if(|x| x.is_ascii_alphanumeric() || *x == '_') {
                    ident_str.push(c);
                }

                match ident_str.as_str() {
                    "let" => token!(Let, ident_str),
                    "print" => token!(Print, ident_str),
                    _ => token!(Identifier, ident_str),
                }
            }

            c => return Err(Error::UnexpectedToken(c)),
        }
        col += 1;
    }

    #[cfg(debug_assertions)]
    if let Err(e) = print_tokens(&tokens) {
        eprintln!("{e}")
    }

    Ok(tokens)
}

pub fn run(program: Vec<Chunk>) -> Result<()> {
    let mut stack: Vec<Value> = vec![];

    for (chunk_index, chunk) in program.into_iter().enumerate() {
        let (data, code, args) = chunk.consume();
        let mut args = args.iter();

        for instr in code {
            match instr {
                Instruction::Constant => {
                    let Some(addr) = args.next() else {
                        return Err(Error::WrongNumberBytes(1, 0, instr));
                    };
                    let Some(val) = data.get(*addr as usize).cloned() else {
                        return Err(Error::MissingValue(chunk_index, *addr));
                    };
                    stack.push(val);
                }
                Instruction::Print => {
                    let Some(val) = stack.pop() else {
                        return Err(Error::WrongNumberArguments(1, 0, instr.to_string()));
                    };
                    println!("{val}");
                }
                Instruction::Return => {
                    if let Some(val) = stack.pop() {
                        println!("Result: {val}");
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn exec(source: &str) -> Result<()> {
    lex(source)?;

    Ok(())
}
