use super::expr::Expr;

pub type SubStmt<'t> = Box<Stmt<'t>>;

#[derive(Debug)]
pub enum Stmt<'t> {
    Declare(Expr<'t>, Option<Expr<'t>>),
    Expr(Expr<'t>),
    Print(Expr<'t>),
    Exit(Option<Expr<'t>>),
}
