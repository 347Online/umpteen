use std::{error::Error, fmt::Display, num::ParseFloatError};

use crate::repr::{
    ast::ops::{Binary, Unary},
    token::TokenType,
};

#[derive(Debug)]
pub enum ParseError {
    InvalidNumericLiteral(ParseFloatError),
    ExpectedStatement,
    ExpectedExpression,
    UnexpectedEof,
    IllegalBinaryOperation(String, String, Binary),
    IllegalUnaryOperation(String, Unary),
    UnexpectedToken(TokenType),
    ExpectedToken(TokenType),
    InvalidAssignmentTarget(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            ParseError::UnexpectedEof => "unexpected end of file".to_string(),
            ParseError::IllegalBinaryOperation(lhs, rhs, op) => {
                format!(
                    "cannot apply binary {} operation to {} and {}",
                    op, lhs, rhs
                )
            }
            ParseError::IllegalUnaryOperation(val, op) => {
                format!("cannot apply unary {} operation to {}", op, val)
            }
            ParseError::ExpectedExpression => "expected expression".to_string(),
            ParseError::ExpectedStatement => "expected statement".to_string(),
            ParseError::ExpectedToken(exp) => format!("expected {}", exp),
            ParseError::UnexpectedToken(kind) => format!("unexpected token: {}", kind),

            ParseError::InvalidNumericLiteral(e) => e.to_string(),
            ParseError::InvalidAssignmentTarget(target) => {
                format!("invalid assignment target `{}`", target)
            }
        };
        write!(f, "{}", desc)
    }
}

impl From<ParseFloatError> for ParseError {
    fn from(value: ParseFloatError) -> Self {
        ParseError::InvalidNumericLiteral(value)
    }
}

impl Error for ParseError {}
