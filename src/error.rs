use std::fmt::Display;

use crate::instr::Instruction;

pub type UmpResult<T> = Result<T, UmpError>;

#[derive(Debug)]
pub enum UmpError {
    UnexpectedToken(char),
    UnexpectedEof,
    InvalidInstruction(u8),
    WrongNumberBytes(usize, usize, Instruction),
    WrongNumberArguments(usize, usize, String),
    MissingValue(usize, u8),
}

impl Display for UmpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            Self::UnexpectedToken(c) => format!("unexpected token `{}`", c),
            Self::UnexpectedEof => "unexpected end of file".to_string(),
            Self::InvalidInstruction(byte) => format!("invalid Instruction `{}`", byte),
            Self::WrongNumberBytes(exp, got, instr) => format!(
                "wrong number of byte arguments for instruction {}, expected {} but got {}",
                instr, exp, got
            ),
            Self::WrongNumberArguments(exp, got, call) => format!(
                "wrong number of arguments for {}, expected {} but got {}",
                call, exp, got
            ),
            Self::MissingValue(pos, addr) => format!("missing value in chunk {} @ {}", pos, addr),
        };
        write!(f, "{desc}")
    }
}

pub fn report(e: UmpError) {
    eprintln!("ERR: {e}");
}

pub fn report_line(e: UmpError, line: usize, col: usize) {
    eprintln!("ERR: {} on line {}:{}", e, line, col);
}
