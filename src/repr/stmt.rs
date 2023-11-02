use crate::Result;

use super::expr::Expr;

#[derive(Debug)]
pub enum Stmt<'t, 'e> {
    Expr(Expr<'t, 'e>),
}

impl<'t, 'e> Stmt<'t, 'e> {
    pub fn eval(self) -> Result<()> {
        match self {
            Stmt::Expr(expr) => expr.eval()?,
        };

        Ok(())
    }
}
