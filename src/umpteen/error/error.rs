use std::{error::Error, fmt::Display};

use super::{CompilerError, ParseError, RuntimeError, SyntaxError};

#[derive(Debug)]
pub enum UmpteenError {
    SyntaxError(SyntaxError),
    ParseError(ParseError),
    CompilerError(CompilerError),
    RuntimeError(RuntimeError),
}

impl UmpteenError {
    pub fn report_line<E: std::error::Error>(error: E, line: Line) {
        eprintln!("ERR: {} on line {}", error, line);
    }
    pub fn report<E: std::error::Error>(error: E) {
        eprintln!("ERR: {}", error);
    }
}

impl Display for UmpteenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UmpteenError::SyntaxError(e) => write!(f, "{}", e),
            UmpteenError::ParseError(_) => todo!(),
            UmpteenError::CompilerError(e) => write!(f, "{}", e),
            UmpteenError::RuntimeError(e) => write!(f, "{}", e),
        }
    }
}

impl Error for UmpteenError {}

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

#[derive(Debug, Clone, Copy, Default)]
pub struct Line(usize, usize);

impl Line {
    pub fn new(ln: usize) -> Self {
        Self(ln, 0)
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
