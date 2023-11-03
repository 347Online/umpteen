use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    UnexpectedEof,
    CorruptedChunk,
    UnexpectedToken(char),
    InvalidInstruction(u8),
    WrongNumberArguments(usize, usize, String),
    IllegalDeclare,
    Runtime(RuntimeError),
}

#[derive(Debug, Clone, Copy)]
pub struct Line(usize, usize);

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.1 == 0 {
            write!(f, "{}", self.0)
        } else {
            write!(f, "{}:{}", self.0, self.1)
        }
    }
}

#[derive(Debug)]
pub enum RuntimeError {
    Illegal,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            Error::CorruptedChunk => "encountered corrupted chunk".to_string(),
            Error::UnexpectedEof => "unexpected end of file".to_string(),
            Error::UnexpectedToken(c) => format!("unexpected token `{}`", c),
            Error::InvalidInstruction(byte) => format!("invalid Instruction `{}`", byte),
            Error::WrongNumberArguments(exp, got, call) => format!(
                "wrong number of arguments for {}, expected {} but got {}",
                call, exp, got
            ),
            Error::Runtime(r) => match r {
                RuntimeError::Illegal => "illegal operation".to_string(),
            },
            Error::IllegalDeclare => "illegal declaration".to_string(),
        };
        write!(f, "{desc}")
    }
}

pub fn report(e: Error) {
    eprintln!("ERR: {e}");
}

pub fn report_line(e: Error, line: usize, col: usize) {
    eprintln!("ERR: {} on line {}:{}", e, line, col);
}
