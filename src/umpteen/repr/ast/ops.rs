use std::fmt::Display;

use crate::{error::ParseError, repr::token::TokenType};

#[derive(Debug, Clone, Copy)]
pub enum Unary {
    Not,
    Negate,
}

impl Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Unary::Not => "negate",
            Unary::Negate => "logical NOT",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Binary {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    And,
    Or,
    Equality,
    Inequality,
    GreaterThan,
    GreaterOrEqual,
    LessThan,
    LessOrEqual,
}

impl Binary {
    pub fn logical(&self) -> bool {
        matches!(self, Binary::And | Binary::Or)
    }
}

impl TryFrom<TokenType> for Binary {
    type Error = ParseError;

    fn try_from(value: TokenType) -> Result<Self, Self::Error> {
        let op = match value {
            TokenType::EqualEqual => Self::Equality,
            TokenType::BangEqual => Self::Inequality,
            TokenType::Greater => Self::GreaterThan,
            TokenType::GreaterEqual => Self::GreaterOrEqual,
            TokenType::Less => Self::LessThan,
            TokenType::LessEqual => Self::LessOrEqual,
            TokenType::Plus | TokenType::PlusEqual => Self::Add,
            TokenType::Minus | TokenType::MinusEqual => Self::Subtract,
            TokenType::Star | TokenType::StarEqual => Self::Multiply,
            TokenType::Slash | TokenType::SlashEqual => Self::Divide,
            TokenType::Percent | TokenType::PercentEqual => Self::Modulo,

            _ => Err(ParseError::UnexpectedToken(value))?,
        };

        Ok(op)
    }
}

impl TryFrom<TokenType> for Unary {
    type Error = ParseError;

    fn try_from(value: TokenType) -> Result<Self, Self::Error> {
        let op = match value {
            TokenType::Minus => Self::Negate,
            TokenType::Bang => Self::Not,

            _ => Err(ParseError::UnexpectedToken(value))?,
        };

        Ok(op)
    }
}

impl Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Binary::Add => "addition",
            Binary::Subtract => "subtraction",
            Binary::Multiply => "multiplication",
            Binary::Divide => "division",
            Binary::Modulo => "remainder",
            Binary::And => "logical AND",
            Binary::Or => "logical OR",
            Binary::Equality => "equality",
            Binary::Inequality => "inequality",
            Binary::GreaterThan => "greater than",
            Binary::GreaterOrEqual => "greater than or equal to",
            Binary::LessThan => "less than",
            Binary::LessOrEqual => "less than or equal to",
        };
        write!(f, "{}", name)
    }
}
