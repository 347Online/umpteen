use super::expr::Expr;

pub type SubStmt = Box<Stmt>;

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Declare {
        name: String,
        init: Option<Expr>,
    },
    Block(Vec<Stmt>),
    Condition {
        test: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
    Loop(Vec<Stmt>),
    Break,
    Continue,
    Return(Expr),
    Fnc {
        name: String,
        params: Vec<(String, String)>, // TODO: Second tuple value should be Type
        body: Vec<Stmt>,
    },
    Exit,
}
