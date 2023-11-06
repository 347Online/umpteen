use crate::{
    ast::{Expr, Stmt},
    error::{CompilerError, UmpteenError},
    value::Value,
};

use super::{AddrMode, Chunk, Instr};

pub type Program = Vec<Chunk>;

pub enum Address {
    Byte(u8),
    Word(u16),
    Long(u32),
}

impl Address {
    pub fn read(&self) -> (usize, usize) {
        match self {
            Address::Byte(b) => (*b as usize, 1),
            Address::Word(w) => (*w as usize, 2),
            Address::Long(l) => (*l as usize, 4),
        }
    }
}

#[derive(Debug)]
pub struct Compiler<'m> {
    pub mem: &'m mut Memory<'m>,

    instr_buf: Vec<Instr>,
    arg_buf: Vec<usize>, // TODO: This type should not be this specific
}

impl<'c> Compiler<'c> {
    pub fn new(mem: &'c mut Memory<'c>) -> Self {
        Compiler {
            mem,
            instr_buf: vec![],
            arg_buf: vec![],
        }
    }

    pub fn compile(&mut self, ast: Vec<Stmt<'c>>) -> Result<Program, CompilerError> {
        for stmt in ast {
            self.compile_stmt(stmt);
        }
        let chunk = self.flush()?;
        let program = vec![chunk];
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
