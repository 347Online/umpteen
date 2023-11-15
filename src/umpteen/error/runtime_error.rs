// TODO: Rename this type to InterpreterError, reserving RuntimeError for the future compiled variant's more minimal runtime

use std::{error::Error, fmt::Display};

use crate::exec::interpreter::Divergence;

use super::MemoryError;

#[derive(Debug)]
pub enum RuntimeError {
    StackMissingValue,
    MemoryError(MemoryError),
    InvalidInstruction(u8),
    ChunkReadError,
    IllegalDivergence(String),
    TriedToCallNonFunction(String),
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            RuntimeError::StackMissingValue => "popped when stack was empty".to_string(),
            RuntimeError::ChunkReadError => "chunk read error".to_string(),
            RuntimeError::MemoryError(e) => e.to_string(),
            RuntimeError::IllegalDivergence(x) => format!("illegal divergence: {}", x),
            RuntimeError::TriedToCallNonFunction(x) => format!("`{}` is not a function", x),
            RuntimeError::InvalidInstruction(x) => format!("invalid Instruction `{:#04x}`", x),
        };

        write!(f, "{}", desc)
    }
}

impl From<MemoryError> for RuntimeError {
    fn from(value: MemoryError) -> Self {
        RuntimeError::MemoryError(value)
    }
}

impl From<Divergence> for RuntimeError {
    fn from(value: Divergence) -> Self {
        RuntimeError::IllegalDivergence(value.to_string())
    }
}

impl Error for RuntimeError {}
