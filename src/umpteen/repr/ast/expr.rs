use crate::repr::value::Value;

use super::ops::{Binary, Unary};

pub type SubExpr<'t> = Box<Expr<'t>>;

#[derive(Debug)]
pub enum Expr<'t> {
    Literal(Value),
    UnOp {
        expr: SubExpr<'t>,
        op: Unary,
    },
    BinOp {
        left: SubExpr<'t>,
        right: SubExpr<'t>,
        op: Binary,
    },
    Binding {
        name: &'t str,
    },
    Assign {
        name: &'t str,
        expr: SubExpr<'t>,
    },
    Grouping {
        expr: SubExpr<'t>,
    },
}

impl<'t> Expr<'t> {
    pub fn unary(expr: Expr<'t>, op: Unary) -> Expr<'t> {
        // Helper method for initializing unary op expressions
        let expr = Box::new(expr);
        Expr::UnOp { expr, op }
    }
    pub fn binary(left: Expr<'t>, right: Expr<'t>, op: Binary) -> Expr<'t> {
        // Helper method for initializing binary op expressions
        let (left, right) = (Box::new(left), Box::new(right));
        Expr::BinOp { left, right, op }
    }
}
