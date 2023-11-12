use std::{
    error::Error,
    fmt::{Debug, Display},
};

use rustyline::error::ReadlineError;

#[derive(Debug)]
pub enum UmpteenError {
    ParseError(ParseError),
    CompilerError(CompilerError),
    RuntimeError(RuntimeError),
    MemoryError(MemoryError),
    ReplError(ReadlineError),
}

impl Display for UmpteenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UmpteenError::ParseError(e) => write!(f, "{}", e),
            UmpteenError::CompilerError(e) => write!(f, "{}", e),
            UmpteenError::RuntimeError(e) => write!(f, "{}", e),
            UmpteenError::MemoryError(e) => write!(f, "{}", e),
            UmpteenError::ReplError(e) => write!(f, "{}", e),
        }
    }
}

use super::{CompilerError, MemoryError, ParseError, RuntimeError};

#[derive(Debug, Clone, Copy)]
pub struct Line(usize, usize);

impl Line {
    pub fn new(ln: usize) -> Self {
        Self(ln, 0)
    }

    pub fn column(&mut self, col: usize) -> Self {
        self.1 = col;
        *self
    }

    pub fn newline(&mut self) -> Self {
        self.0 += 1;
        self.1 = 1;
        *self
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl From<CompilerError> for UmpteenError {
    fn from(value: CompilerError) -> Self {
        UmpteenError::CompilerError(value)
    }
}

impl From<RuntimeError> for UmpteenError {
    fn from(value: RuntimeError) -> Self {
        UmpteenError::RuntimeError(value)
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

impl Error for UmpteenError {}
