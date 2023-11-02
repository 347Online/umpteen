use crate::{exec::env::Environment, Result};

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
pub enum Expr<'t, 'e> {
    Value(Value),
    UnOp {
        expr: Box<Expr<'t, 'e>>,
        op: UnaryOp,
    },
    BinOp {
        left: Box<Expr<'t, 'e>>,
        right: Box<Expr<'t, 'e>>,
        op: BinaryOp,
    },
    Ident {
        name: &'t str,
        env: &'e mut Environment,
    },
}

impl<'t, 'e> Expr<'t, 'e> {
    pub fn unary(expr: Expr<'t, 'e>, op: UnaryOp) -> Expr<'t, 'e> {
        let expr = Box::new(expr);
        Expr::UnOp { expr, op }
    }
    pub fn binary(left: Expr<'t, 'e>, right: Expr<'t, 'e>, op: BinaryOp) -> Expr<'t, 'e> {
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
            Expr::Ident { name, env } => env.get(name),
        };
        Ok(v)
    }
}
