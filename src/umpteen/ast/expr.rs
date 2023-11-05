use crate::{value::Value, Result};

use super::{Binary, Unary};

pub type SubExpr<'t> = Box<Expr<'t>>;

#[derive(Debug)]
pub enum Expr<'t> {
    Value(Value),
    UnOp {
        expr: SubExpr<'t>,
        op: Unary,
    },
    BinOp {
        left: SubExpr<'t>,
        right: SubExpr<'t>,
        op: Binary,
    },
    Ident {
        name: &'t str,
    },
    Assign {
        name: &'t str,
        expr: SubExpr<'t>,
    },
}

impl<'t> Expr<'t> {
    pub fn unary(expr: Expr<'t>, op: Unary) -> Expr<'t> {
        let expr = Box::new(expr);
        Expr::UnOp { expr, op }
    }
    pub fn binary(left: Expr<'t>, right: Expr<'t>, op: Binary) -> Expr<'t> {
        let (left, right) = (Box::new(left), Box::new(right));
        Expr::BinOp { left, right, op }
    }
    pub fn eval(self) -> Result<Value> {
        let v = match self {
            Expr::Value(v) => v,
            Expr::UnOp { expr, op } => match op {
                Unary::Not => {
                    let x = expr.eval()?;
                    !x
                }
                Unary::Negate => {
                    let x = expr.eval()?;
                    (-x)?
                }
            },
            Expr::BinOp { left, right, op } => match op {
                Binary::Add => {
                    let a = left.eval()?;
                    let b = right.eval()?;
                    (a + b)?
                }
                Binary::Subtract => {
                    let a = left.eval()?;
                    let b = right.eval()?;
                    (a - b)?
                }
                Binary::Multiply => {
                    let a = left.eval()?;
                    let b = right.eval()?;
                    (a * b)?
                }
                Binary::Divide => {
                    let a = left.eval()?;
                    let b = right.eval()?;
                    (a / b)?
                }
                Binary::Modulo => {
                    let a = left.eval()?;
                    let b = right.eval()?;
                    (a % b)?
                }

                Binary::And => {
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
