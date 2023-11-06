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

use super::{env::Memory, parse::Ast};

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

    pub fn compile(mut self, ast: Ast<'c>) -> Result<Program<'c>, CompilerError> {
        for stmt in ast {
            self.compile_stmt(stmt);
        }
        let chunk = self.flush()?;
        let chunks = vec![chunk];
        let program = Program {
            chunks,
            mem: self.mem,
        };
        Ok(program)
    }

    fn compile_stmt(&mut self, stmt: Stmt<'c>) {
        match stmt {
            Stmt::Expr(expr) => self.compile_expr(expr),
            Stmt::Print(expr) => {
                self.compile_expr(expr);
                self.compile_instr(Instr::Print);
            }
            Stmt::Return(x) => {
                if let Some(expr) = x {
                    self.compile_expr(expr);
                };
                self.compile_instr(Instr::Return);
            }
        }
    }

    fn compile_expr(&mut self, expr: Expr<'c>) {
        match expr {
            Expr::Constant(value) => {
                if value != Value::Empty {
                    self.compile_instr(Instr::Constant);
                    self.compile_constant(value);
                }
            }
            Expr::UnOp { expr, op } => todo!(),
            Expr::BinOp { left, right, op } => todo!(),
            Expr::Ident { name } => todo!(),
            Expr::Assign { name, expr } => {
                self.compile_declare(name);
                self.compile_expr(*expr)
            }
        }
    }

    fn compile_instr(&mut self, instr: Instr) {
        self.instr_buf.push(instr);
    }

    fn compile_declare(&mut self, name: &'c str) -> Result<(), UmpteenError> {
        let addr = self.mem.declare(name)?;
        self.arg_buf.push(addr);
        Ok(())
    }

    fn compile_constant(&mut self, value: Value) {
        let addr = self.mem.declare_constant(value);
        self.arg_buf.push(addr);
    }

    fn flush(&mut self) -> Result<Chunk, CompilerError> {
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

        Ok(chunk)
    }
}
