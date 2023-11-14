use crate::{repr::value::Value, boxed};

use super::ops::{Binary, Unary};

pub type SubExpr<'t> = Box<Expr<'t>>;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'t> {
    Literal(Value),
    List(Vec<Expr<'t>>),
    Binding {
        name: &'t str,
        index: Option<SubExpr<'t>>,
    },
    Grouping {
        expr: SubExpr<'t>,
    },
    UnOp {
        expr: SubExpr<'t>,
        op: Unary,
    },
    BinOp {
        left: SubExpr<'t>,
        right: SubExpr<'t>,
        op: Binary,
    },
    Assign {
        name: &'t str,
        index: Option<SubExpr<'t>>,
        expr: SubExpr<'t>,
    },
    Call {
        callee: SubExpr<'t>,
        args: Vec<Expr<'t>>,
    },
}

impl<'t> Expr<'t> {
    pub fn unary(expr: Expr<'t>, op: Unary) -> Expr<'t> {
        let expr = boxed!(expr);
        Expr::UnOp { expr, op }
    }
    pub fn binary(left: Expr<'t>, right: Expr<'t>, op: Binary) -> Expr<'t> {
        let (left, right) = (boxed!(left), boxed!(right));
        Expr::BinOp { left, right, op }
    }
}
