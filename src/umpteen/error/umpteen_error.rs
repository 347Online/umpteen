#[derive(Debug)]
pub enum UmpteenError {
    SyntaxError(SyntaxError),
    ParseError(ParseError),
    CompilerError(CompilerError),
    RuntimeError(RuntimeError),
    MemoryError(MemoryError),
}

impl UmpteenError {
    pub fn report_line<E: Display>(error: E, line: Line) {
        eprintln!("ERR: {} on line {}", error, line);
    }
    pub fn report<E: Display>(error: E) {
        eprintln!("ERR: {}", error);
    }
}

impl Display for UmpteenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UmpteenError::SyntaxError(e) => write!(f, "{}", e),
            UmpteenError::ParseError(e) => write!(f, "{}", e),
            UmpteenError::CompilerError(e) => write!(f, "{}", e),
            UmpteenError::RuntimeError(e) => write!(f, "{}", e),
            UmpteenError::MemoryError(e) => write!(f, "{}", e),
        }
    }
}

use std::{
    error::Error,
    fmt::{Debug, Display},
};

use super::{CompilerError, MemoryError, ParseError, RuntimeError, SyntaxError};

#[derive(Debug, Clone, Copy)]
pub struct Line(usize, usize);

impl Line {
    pub fn new(ln: usize) -> Self {
        Self(ln, 0)
    }

    pub fn advance(&mut self) {
        self.1 += 1;
    }

    pub fn newline(&mut self) {
        self.0 += 1;
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

impl From<SyntaxError> for UmpteenError {
    fn from(value: SyntaxError) -> Self {
        UmpteenError::SyntaxError(value)
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

impl Error for UmpteenError {}
