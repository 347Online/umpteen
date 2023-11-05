use std::{fmt::Display, error::Error};

#[derive(Debug)]
pub enum RuntimeError {
    OutOfBoundsMemoryAccess,
    StackMissingValue,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            RuntimeError::OutOfBoundsMemoryAccess => "out of bounds memory access",
            RuntimeError::StackMissingValue => "popped when stack was empty",
        };

        write!(f, "{}", desc)
    }
}

impl Error for RuntimeError {}
