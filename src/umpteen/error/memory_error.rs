use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum MemoryError {
    InvalidReference(usize),
    OutOfBoundsMemoryAccess,
}

impl Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            MemoryError::InvalidReference(addr) => {
                format!("reference to invalid address {:#}", addr)
            }
            MemoryError::OutOfBoundsMemoryAccess => "out of bounds memory access".to_string(),
        };
        write!(f, "{}", desc)
    }
}

impl Error for MemoryError {}
