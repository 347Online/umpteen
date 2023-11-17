use crate::{boxed, repr::value::Value};

use super::ops::{Binary, Unary};

pub type SubExpr = Box<Expr>;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Value),
    List(Vec<Expr>),
    Binding {
        name: String,
        index: Option<SubExpr>,
    },
    Grouping {
        expr: SubExpr,
    },
    UnOp {
        expr: SubExpr,
        op: Unary,
    },
    BinOp {
        left: SubExpr,
        right: SubExpr,
        op: Binary,
    },
    Assign {
        name: String,
        index: Option<SubExpr>,
        expr: SubExpr,
    },
    Call {
        callee: SubExpr,
        args: Vec<Expr>,
    },
}

impl Expr {
    pub fn unary(expr: Expr, op: Unary) -> Expr {
        let expr = boxed!(expr);
        Expr::UnOp { expr, op }
    }
    pub fn binary(left: Expr, right: Expr, op: Binary) -> Expr {
        let (left, right) = (boxed!(left), boxed!(right));
        Expr::BinOp { left, right, op }
    }
}
