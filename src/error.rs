use std::fmt::Display;

use crate::instr::Instruction;

pub type UmpResult<T> = Result<T, UmpError>;

#[derive(Debug)]
pub enum UmpError {
    UnexpectedToken(char),
    UnexpectedEof,
    InvalidInstruction(u8),
    WrongNumberBytes(usize, usize, Instruction),
    MissingValue(usize, u8)
}

impl Display for UmpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedToken(c) => write!(f, ""),
            Self::UnexpectedEof => write!(f, "unexpected end of file"),
            Self::InvalidInstruction(byte) => write!(f, "invalid Instruction `{byte}`"),
            Self::WrongNumberBytes(exp, got, instr) => 
                write!(f, "wrong number of byte arguments for instruction {instr}, expected {exp} but got {got}"),
            Self::MissingValue(pos, addr) => 
                write!(f, "missing value in chunk {pos} @ {addr}")
            
        }
    }
}

pub fn report(e: UmpError) {
    eprintln!("ERR: {e}");
}

pub fn report_line(e: UmpError, line: usize) {
    eprintln!("ERR: {e} on line {line}");
}
