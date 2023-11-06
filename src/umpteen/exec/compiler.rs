use crate::{
    error::{CompilerError, UmpteenError},
    repr::{
        ast::{expr::Expr, stmt::Stmt},
        bytecode::{
            chunk::{AddrMode, Chunk},
            instruction::Instr,
        },
        value::Value,
    },
};

use super::{
    env::Memory,
    parse::{Ast, AstNode},
};

#[derive(Debug)]
pub struct Program<'p> {
    pub chunks: Vec<Chunk>,
    pub mem: Memory<'p>,
}

#[derive(Debug, Default)]
pub struct Compiler<'a> {
    mem: Memory<'a>,
    chunks: Vec<Chunk>,

    instr_buf: Vec<Instr>,
    arg_buf: Vec<usize>, // TODO: This type should not be this specific
}

impl<'c> Compiler<'c> {
    pub fn new(mem: Memory<'c>) -> Self {
        Compiler {
            mem,
            chunks: vec![],

            instr_buf: vec![],
            arg_buf: vec![],
        }
    }

    pub fn compile(mut self, ast: Ast<'c>) -> Program<'c> {
        for stmt in ast {
            self.statment(stmt);
        }
        self.flush();
        let chunks = self.chunks;
        let program = Program {
            chunks,
            mem: self.mem,
        };

        program
    }

    fn statment(&mut self, stmt: Stmt<'c>) -> Result<(), CompilerError> {
        match stmt {
            Stmt::Expr(expr) => {
                self.expression(expr);
            }
            Stmt::Print(expr) => {
                self.expression(expr);
                self.push_instr(Instr::Print);
            }
            Stmt::Exit(x) => {
                if let Some(expr) = x {
                    self.expression(expr);
                };
                self.push_instr(Instr::Exit);
            }
            Stmt::Declare(ident, _) => {}
        }

        Ok(())
    }

    fn expression(&mut self, expr: Expr<'c>) {
        match expr {
            Expr::Constant(value) => {
                if value != Value::Empty {
                    self.push_instr(Instr::Constant);
                    self.constant(value);
                }
            }
            Expr::UnOp { expr, op } => todo!(),
            Expr::BinOp { left, right, op } => todo!(),
            Expr::Ident { name } => todo!(),
        }
    }

    fn declaration(&mut self, name: &'c str) -> Result<usize, CompilerError> {
        let addr = self.mem.declare(name)?;
        self.arg_buf.push(addr);
        Ok(addr)
    }

    fn constant(&mut self, value: Value) {
        let addr = self.mem.declare_constant(value);
        self.arg_buf.push(addr);
    }

    fn push_instr(&mut self, instr: Instr) {
        self.instr_buf.push(instr);
    }

    fn flush(&mut self) {
        let addr_mode = match self.arg_buf.len() {
            x if x < AddrMode::BYTE => AddrMode::Byte,
            x if x < AddrMode::WORD => AddrMode::Word,
            x if x < AddrMode::LONG => AddrMode::Long,
            _ => panic!("out of memory somehow, incredible"),
        };

        let mut chunk = Chunk::new(addr_mode);

        let mut arg_pos = 0;

        let instr_buf = std::mem::take(&mut self.instr_buf);

        for instr in instr_buf {
            chunk.write_instr(instr);
            match instr {
                Instr::Constant => {
                    let addr = self.arg_buf[arg_pos];
                    arg_pos += 1;
                    chunk.write_addr(addr);
                }
                _ => (),
            }
        }

        self.chunks.push(chunk);
    }
}
