use crate::exec::interpreter::Divergence;

use super::{InterpretError, MemoryError, ParseError};
use std::{
    error::Error,
    fmt::{Debug, Display},
};

use rustyline::error::ReadlineError;

#[derive(Debug)]
pub enum UmpteenError {
    ParseError(ParseError),
    InterpretError(InterpretError),
    MemoryError(MemoryError),
    ReplError(ReadlineError),
    Divergence(Divergence),
}

impl Display for UmpteenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UmpteenError::ParseError(e) => write!(f, "{}", e),
            UmpteenError::InterpretError(e) => write!(f, "{}", e),
            UmpteenError::MemoryError(e) => write!(f, "{}", e),
            UmpteenError::ReplError(e) => write!(f, "{}", e),
            UmpteenError::Divergence(e) => write!(f, "{}", e),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Line(usize, usize);

impl Line {
    pub fn new(ln: usize) -> Self {
        Self(ln, 0)
    }

    pub fn advance(&mut self) -> Self {
        self.1 += 1;
        *self
    }

    pub fn newline(&mut self) -> Self {
        self.0 += 1;
        self.1 = 1;
        *self
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.1 == 0 {
            write!(f, "{}", self.0)
        } else {
            write!(f, "{}:{}", self.0, self.1)
        }
    }
}

impl From<ParseError> for UmpteenError {
    fn from(value: ParseError) -> Self {
        UmpteenError::ParseError(value)
    }
}

impl From<InterpretError> for UmpteenError {
    fn from(value: InterpretError) -> Self {
        UmpteenError::InterpretError(value)
    }
}

impl From<MemoryError> for UmpteenError {
    fn from(value: MemoryError) -> Self {
        UmpteenError::MemoryError(value)
    }
}

impl From<ReadlineError> for UmpteenError {
    fn from(value: ReadlineError) -> Self {
        UmpteenError::ReplError(value)
    }
}

impl From<Divergence> for UmpteenError {
    fn from(value: Divergence) -> Self {
        UmpteenError::Divergence(value)
    }
}

impl Error for UmpteenError {}
