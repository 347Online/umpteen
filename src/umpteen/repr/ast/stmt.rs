use super::expr::Expr;

pub type SubStmt<'t> = Box<Stmt<'t>>;

#[derive(Debug)]
pub enum Stmt<'t> {
    Declare(&'t str, Option<Expr<'t>>),
    Expr(Expr<'t>),
    Print(Expr<'t>),
    Return(Expr<'t>),
    Exit,
}
