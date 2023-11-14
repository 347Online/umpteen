use std::{error::Error, fmt::Display};

use crate::repr::value::Value;

#[derive(Debug)]
pub enum MemoryError {
    NoSuchVariable(String),
    UninitializedVariableAccess(String),
    OutOfBoundsMemoryAccess,
    CannotIndex(String),
    CannotIndexWith(String),
}

impl Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            MemoryError::NoSuchVariable(name) => format!("no such variable `{}`", name),
            MemoryError::UninitializedVariableAccess(name) => {
                format!("tried to access uninitialized variable `{}`", name)
            }

            MemoryError::OutOfBoundsMemoryAccess => "out of bounds memory access".to_string(),
            MemoryError::CannotIndex(x) => format!("cannot index into {}", x),
            MemoryError::CannotIndexWith(x) => format!("invalid index type for variable `{}`", x),
        };
        write!(f, "{}", desc)
    }
}

impl Error for MemoryError {}
