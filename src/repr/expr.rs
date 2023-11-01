use super::{value::Value, Result};

pub enum UnaryOp {
    Not,
    Negate,
}

pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
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
                _ => todo!()
                // BinaryOp::Subtract => todo!(),
                // BinaryOp::Multiply => todo!(),
                // BinaryOp::Divide => todo!(),
            },
            // _ => todo!(),
        };
        Ok(v)
    }
    // pub fn truthy(self) -> bool {

    // }
}
