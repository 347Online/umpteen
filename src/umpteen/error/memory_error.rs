use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum MemoryError {
    NoSuchVariable(String),
    UninitializedVariable(String),
    OutOfBoundsMemoryAccess,
    CannotIndex(String),
    CannotIndexWith(String),
}

impl Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            MemoryError::NoSuchVariable(x) => format!("no such variable `{}`", x),
            MemoryError::OutOfBoundsMemoryAccess => "out of bounds memory access".to_string(),
            MemoryError::CannotIndex(x) => format!("cannot index into {}", x),
            MemoryError::CannotIndexWith(x) => format!("invalid index type for variable `{}`", x),
            MemoryError::UninitializedVariable(x) => format!("variable `{}` is not initialized", x),
        };
        write!(f, "{}", desc)
    }
}

impl Error for MemoryError {}
