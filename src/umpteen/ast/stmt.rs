use crate::Result;

use super::Expr;

#[derive(Debug)]
pub enum Stmt<'t> {
    Expr(Expr<'t>),
}

impl<'t> Stmt<'t> {
    pub fn eval(self) -> Result<()> {
        match self {
            Stmt::Expr(expr) => expr.eval()?,
        };

        Ok(())
    }
}
