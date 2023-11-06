use std::{error::Error, fmt::Display};

use super::MemoryError;

#[derive(Debug)]
pub enum RuntimeError {
    StackMissingValue,
    MemoryError(MemoryError),
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            RuntimeError::StackMissingValue => "popped when stack was empty".to_string(),
            RuntimeError::MemoryError(e) => e.to_string(),
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
