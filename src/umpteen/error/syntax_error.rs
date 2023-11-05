use std::{error::Error, fmt::Display};

use crate::{token::TokenType, value::Value, ast::{Binary, Unary}};

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

impl Error for SyntaxError {}
