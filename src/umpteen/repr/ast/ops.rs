use std::fmt::Display;

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Binary {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    And,
    Or,
}

impl Binary {
    pub fn logical(&self) -> bool {
        matches!(self, Binary::And | Binary::Or)
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
        };
        write!(f, "{}", name)
    }
}
