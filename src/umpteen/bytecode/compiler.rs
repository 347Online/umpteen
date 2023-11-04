use crate::{ast::Expr, Program};

use super::Chunk;

#[derive(Debug, Default)]
pub struct Compiler {
    chunk: Chunk,
}

impl Compiler {
    pub fn new(chunk: Chunk) -> Self {
        Compiler { chunk }
    }

    pub fn compile(mut self, ast: Expr) -> Program {
        match ast {
            Expr::Value(val) => todo!(),
            Expr::UnOp { expr, op } => todo!(),
            Expr::BinOp { left, right, op } => todo!(),
            Expr::Ident { name } => todo!(),
            Expr::Assign { name, expr } => todo!(),
        }
    }
}
