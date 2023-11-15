use super::expr::Expr;

pub type SubStmt<'t> = Box<Stmt<'t>>;

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt<'t> {
    Expr(Expr<'t>),
    Declare {
        name: &'t str,
        init: Option<Expr<'t>>,
    },
    Block(Vec<Stmt<'t>>),
    Condition {
        test: Expr<'t>,
        then_branch: Vec<Stmt<'t>>,
        else_branch: Option<Vec<Stmt<'t>>>,
    },
    Loop(Vec<Stmt<'t>>),
    Break,
    Continue,
    Return(Expr<'t>),
    Fnc {
        name: &'t str,
        params: Vec<(String, String)>, // TODO: Second tuple value should be Type
        body: Vec<Stmt<'t>>,
    },
    Exit,
}
