use crate::Result;

use super::value::Value;

pub enum UnaryOp {
    Not,
    Negate,
}

pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    And,
}

pub enum Expr {
    Value(Value),
    UnOp {
        expr: Box<Expr>,
        op: UnaryOp,
    },
    BinOp {
        left: Box<Expr>,
        right: Box<Expr>,
        op: BinaryOp,
    },
}

impl Expr {
    pub fn unary(expr: Expr, op: UnaryOp) -> Expr {
        let expr = Box::new(expr);
        Expr::UnOp { expr, op }
    }
    pub fn binary(left: Expr, right: Expr, op: BinaryOp) -> Expr {
        let (left, right) = (Box::new(left), Box::new(right));
        Expr::BinOp { left, right, op }
    }
    pub fn eval(self) -> Result<Value> {
        let v = match self {
            Expr::Value(v) => v,
            Expr::UnOp { expr, op } => match op {
                UnaryOp::Not => {
                    let x = expr.eval()?;
                    !x
                }
                UnaryOp::Negate => {
                    let x = expr.eval()?;
                    (-x)?
                }
            },
            Expr::BinOp { left, right, op } => match op {
                BinaryOp::Add => {
                    let a = left.eval()?;
                    let b = right.eval()?;
                    (a + b)?
                }
                BinaryOp::Subtract => {
                    let a = left.eval()?;
                    let b = right.eval()?;
                    (a - b)?
                }
                BinaryOp::Multiply => {
                    let a = left.eval()?;
                    let b = right.eval()?;
                    (a * b)?
                }
                BinaryOp::Divide => {
                    let a = left.eval()?;
                    let b = right.eval()?;
                    (a / b)?
                }
                BinaryOp::Modulo => {
                    let a = left.eval()?;
                    let b = right.eval()?;
                    (a % b)?
                }

                BinaryOp::And => {
                    let a = left.eval()?;
                    if !a.truthy() {
                        a
                    } else {
                        right.eval()?
                    }
                }
            },
        };
        Ok(v)
    }
}
