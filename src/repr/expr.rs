use crate::Result;

use super::value::Value;

#[derive(Debug)]
pub enum UnaryOp {
    Not,
    Negate,
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    And,
}

#[derive(Debug)]
pub enum Expr<'t> {
    Value(Value),
    UnOp {
        expr: Box<Expr<'t>>,
        op: UnaryOp,
    },
    BinOp {
        left: Box<Expr<'t>>,
        right: Box<Expr<'t>>,
        op: BinaryOp,
    },
    Ident {
        name: &'t str,
    },
    Assign {
        name: &'t str,
        expr: Box<Expr<'t>>,
    },
}

impl<'t> Expr<'t> {
    pub fn unary(expr: Expr<'t>, op: UnaryOp) -> Expr<'t> {
        let expr = Box::new(expr);
        Expr::UnOp { expr, op }
    }
    pub fn binary(left: Expr<'t>, right: Expr<'t>, op: BinaryOp) -> Expr<'t> {
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
            Expr::Ident { name } => todo!(),
            Expr::Assign { name, expr } => todo!(),
        };
        Ok(v)
    }
}
