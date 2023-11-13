use super::expr::Expr;

pub type SubStmt<'t> = Box<Stmt<'t>>;

#[derive(Debug)]
pub enum Stmt<'t> {
    Expr(Expr<'t>),
    Print(Expr<'t>),
    Return(Expr<'t>),
    Declare {
        name: &'t str,
        init: Option<Expr<'t>>,
    },
    Empty,
    Exit,
}
