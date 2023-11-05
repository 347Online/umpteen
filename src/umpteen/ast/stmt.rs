use super::Expr;

pub type SubStmt<'t> = Box<Stmt<'t>>;

#[derive(Debug)]
pub enum Stmt<'t> {
    Expr(Expr<'t>),
    Print(Expr<'t>),
    Return(Option<Expr<'t>>),
}
