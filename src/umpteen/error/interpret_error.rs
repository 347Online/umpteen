use std::{error::Error, fmt::Display};

use crate::exec::interpreter::Divergence;

use super::MemoryError;

#[derive(Debug)]
pub enum InterpretError {
    MemoryError(MemoryError),
    IllegalDivergence(String),
    TriedToCallNonFunction(String),
}

impl Display for InterpretError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            InterpretError::MemoryError(e) => e.to_string(),
            InterpretError::IllegalDivergence(x) => format!("illegal divergence: {}", x),
            InterpretError::TriedToCallNonFunction(x) => format!("`{}` is not a function", x),
        };

        write!(f, "{}", desc)
    }
}

impl From<MemoryError> for InterpretError {
    fn from(value: MemoryError) -> Self {
        InterpretError::MemoryError(value)
    }
}

impl From<Divergence> for InterpretError {
    fn from(value: Divergence) -> Self {
        InterpretError::IllegalDivergence(value.to_string())
    }
}

impl Error for InterpretError {}
