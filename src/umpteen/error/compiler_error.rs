use std::{error::Error, fmt::Display};

use super::MemoryError;

#[derive(Debug)]
pub enum CompilerError {
    CorruptedChunk,
    InvalidInstruction(u8),
    WrongNumberArguments(usize, usize, String),
    IllegalDeclare,
    MemoryError(MemoryError),
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tmp: String;
        let desc = match self {
            CompilerError::CorruptedChunk => "encountered corrupted chunk",
            CompilerError::IllegalDeclare => "illegal declaration",

            CompilerError::InvalidInstruction(byte) => {
                tmp = format!("invalid Instruction `{}`", byte);
                &tmp
            }

            CompilerError::WrongNumberArguments(exp, got, call) => {
                tmp = format!(
                    "wrong number of arguments for {}, expected {} but got {}",
                    call, exp, got
                );
                &tmp
            }
            CompilerError::MemoryError(e) => {
                tmp = e.to_string();
                &tmp
            }
        };
        write!(f, "{}", desc)
    }
}

impl From<MemoryError> for CompilerError {
    fn from(value: MemoryError) -> Self {
        CompilerError::MemoryError(value)
    }
}

impl Error for CompilerError {}
