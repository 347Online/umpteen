use std::{error::Error, f32::consts::E, fmt::Display};

use crate::{
    ast::{Binary, Unary},
    token::TokenType,
    value::Value,
};

#[derive(Debug)]
pub enum UmpteenError {
    SyntaxError(SyntaxError),
    CompilerError(CompilerError),
    RuntimeError(RuntimeError),
    Other(Box<dyn std::error::Error>),
}

impl Display for UmpteenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UmpteenError::RuntimeError(e) => write!(f, "{}", e),
            UmpteenError::SyntaxError(e) => write!(f, "{}", e),
            UmpteenError::CompilerError(e) => write!(f, "{}", e),
            UmpteenError::Other(e) => write!(f, "{}", e),
        }
    }
}

#[derive(Debug)]
pub enum SyntaxError {
    UnexpectedSymbol(char),
    UnexpectedToken(TokenType),
    ExpectedExpression,
    ExpectedStatement,
    UnexpectedEof,
    IllegalBinaryOperation(Value, Value, Binary),
    IllegalUnaryOperation(Value, Unary),
    ExpectedToken(TokenType),
    Other(Box<dyn Error>),
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tmp: String;
        let desc = match self {
            SyntaxError::UnexpectedSymbol(c) => {
                tmp = format!("unexpected symbol `{}`", c);
                &tmp
            }
            SyntaxError::UnexpectedEof => "unexpected end of file",
            SyntaxError::IllegalBinaryOperation(lhs, rhs, op) => {
                tmp = format!(
                    "cannot apply binary {} operation to {} and {}",
                    op, lhs, rhs
                );
                &tmp
            }
            SyntaxError::IllegalUnaryOperation(val, op) => {
                tmp = format!("cannot apply unary {} operation to {}", op, val);
                &tmp
            }
            // SyntaxError::Lexeme(lexeme) => {
            //     tmp = format!("parse error near {}", lexeme);
            //     &tmp
            // }
            SyntaxError::Other(e) => {
                tmp = format!("{}", e);
                &tmp
            }
            SyntaxError::ExpectedExpression => "expected expression",
            SyntaxError::ExpectedStatement => "expected statement",
            SyntaxError::ExpectedToken(exp) => {
                tmp = format!("expected {}", exp);
                &tmp
            }
            SyntaxError::UnexpectedToken(kind) => {
                tmp = format!("unexpected token {}", kind);
                &tmp
            }
        };
        write!(f, "{}", desc)
    }
}

#[derive(Debug)]
pub enum CompilerError {
    CorruptedChunk,
    InvalidInstruction(u8),
    WrongNumberArguments(usize, usize, String),
    IllegalDeclare,
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

impl From<SyntaxError> for UmpteenError {
    fn from(value: SyntaxError) -> Self {
        UmpteenError::SyntaxError(value)
    }
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tmp: String;
        let desc = match self {
            CompilerError::CorruptedChunk => "encountered corrupted chunk",
            CompilerError::InvalidInstruction(byte) => {
                tmp = format!("invalid Instruction `{}`", byte);
                &tmp
            }
            CompilerError::IllegalDeclare => "illegal declaration",

            CompilerError::WrongNumberArguments(exp, got, call) => {
                tmp = format!(
                    "wrong number of arguments for {}, expected {} but got {}",
                    call, exp, got
                );
                &tmp
            }
        };
        write!(f, "{}", desc)
    }
}

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

pub fn report_line<E: std::error::Error>(error: E, line: Line) {
    eprintln!("ERR: {} on line {}", error, line);
}
pub fn report<E: std::error::Error>(error: E) {
    eprintln!("ERR: {}", error);
}
