use super::{Expr, Stmt};

pub enum AstNode<'n> {
    Stmt(Stmt<'n>),
    Expr(Expr<'n>),
}

pub struct Ast<'n> {
    value: AstNode<'n>,
    left: Box<Ast<'n>>,
    right: Box<Ast<'n>>,
}
