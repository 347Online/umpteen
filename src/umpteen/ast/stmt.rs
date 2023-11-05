use crate::Result;

use super::Expr;

#[derive(Debug)]
pub enum Stmt<'t> {
    Expr(Expr<'t>),
    Print(Expr<'t>),
}

// impl<'t> Stmt<'t> {
//     pub fn eval(self) -> Result<()> {
//         match self {
//             Stmt::Expr(expr) => expr.eval()?,
//             Stmt::Print(_) => todo!(),
//         };

//         Ok(())
//     }
// }
