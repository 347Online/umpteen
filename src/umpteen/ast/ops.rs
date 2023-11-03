#[derive(Debug)]
pub enum Binary {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    And,
}

#[derive(Debug)]
pub enum Unary {
    Not,
    Negate,
}
