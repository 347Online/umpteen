use std::{error::Error, fmt::Display};

use crate::repr::value::Value;

use super::MemoryError;

#[derive(Debug)]
pub enum RuntimeError {
    StackMissingValue,
    MemoryError(MemoryError),
    InvalidInstruction(u8),
    ChunkReadError,

    Break,
    Continue,
    Return(Value),
    Exit,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            RuntimeError::StackMissingValue => "popped when stack was empty".to_string(),
            RuntimeError::ChunkReadError => "chunk read error".to_string(),
            RuntimeError::MemoryError(e) => e.to_string(),

            RuntimeError::InvalidInstruction(byte) => {
                format!("invalid Instruction `{:#04x}`", byte)
            }
            RuntimeError::Break => "break not allowed outside loop".to_string(),
            RuntimeError::Continue => "continue not allowed outside loop".to_string(),
            RuntimeError::Return(_) => "return not allowed outside function".to_string(),
            RuntimeError::Exit => "explicit exit".to_string(),
        };

        write!(f, "{}", desc)
    }
}

impl From<MemoryError> for RuntimeError {
    fn from(value: MemoryError) -> Self {
        RuntimeError::MemoryError(value)
    }
}

impl Error for RuntimeError {}
