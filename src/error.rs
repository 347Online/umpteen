use std::fmt::Display;

use crate::instr::Instruction;

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
pub enum Error {
    CorruptChunk(usize),
    UnexpectedToken(char),
    UnexpectedEof,
    InvalidInstruction(u8),
    WrongNumberArguments(usize, usize, String),
    MissingValue(usize, u8),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            Error::CorruptChunk(index) => format!("chunk at index {} is corrupt", index),
            Error::UnexpectedToken(c) => format!("unexpected token `{}`", c),
            Error::UnexpectedEof => "unexpected end of file".to_string(),
            Error::InvalidInstruction(byte) => format!("invalid Instruction `{}`", byte),
            Error::WrongNumberArguments(exp, got, call) => format!(
                "wrong number of arguments for {}, expected {} but got {}",
                call, exp, got
            ),
            Error::MissingValue(pos, addr) => format!("missing value in chunk {} @ {}", pos, addr),
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
