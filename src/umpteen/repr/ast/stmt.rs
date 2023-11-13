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
    Block(Vec<Stmt<'t>>),
    Condition {
        test: Expr<'t>,
        then_branch: SubStmt<'t>,
        else_branch: Option<SubStmt<'t>>,
    },
    Empty,
    Exit,
}
