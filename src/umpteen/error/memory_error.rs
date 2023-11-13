use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum MemoryError {
    NoSuchVariable(String),
    UninitializedVariableAccess(String),
    OutOfBoundsMemoryAccess,
}

impl Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            MemoryError::NoSuchVariable(name) => format!("no such variable `{}`", name),
            MemoryError::UninitializedVariableAccess(name) => 
                format!("tried to access uninitialized variable `{}`", name),
            
            MemoryError::OutOfBoundsMemoryAccess => "out of bounds memory access".to_string(),
        };
        write!(f, "{}", desc)
    }
}

impl Error for MemoryError {}
