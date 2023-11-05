use std::ops::{Deref, DerefMut};

use crate::{
    ast::{Expr, Stmt},
    value::Value,
    Result,
};

use super::{AddrMode, Chunk, Instr};

pub type Bytecode<const N: usize> = [Chunk; N];

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

#[derive(Debug, Default)]
pub struct Memory(Vec<Value>);

impl Memory {
    pub fn get(&self, addr: usize) -> Option<Value> {
        self.0.get(addr).cloned()
    }

    fn offset(&self) -> usize {
        self.0.len()
    }

    fn store(&mut self, value: Value) -> usize {
        let addr = self.offset();
        self.0.push(value);
        addr
    }
}

impl Deref for Memory {
    type Target = Vec<Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Memory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct Compiler<'m> {
    pub mem: &'m mut Memory,

    instr_buf: Vec<Instr>,
    arg_buf: Vec<usize>, // TODO: This type should not be this specific
}

impl<'m> Compiler<'m> {
    pub fn new(mem: &'m mut Memory) -> Self {
        Compiler {
            mem,
            instr_buf: vec![],
            arg_buf: vec![],
        }
    }

    pub fn compile(mut self) {
        self.push_instr(Instr::Return);
    }

    pub fn compile_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Expr(expr) => self.compile_expr(expr),
        }
    }

    pub fn compile_expr(&mut self, expr: Expr) {
        match expr {
            Expr::Value(value) => {
                if value != Value::Empty {
                    let addr = self.mem.store(value);

                    self.instr_buf.push(Instr::Constant);
                    self.arg_buf.push(addr);
                }
            }
            Expr::UnOp { expr, op } => todo!(),
            Expr::BinOp { left, right, op } => todo!(),
            Expr::Ident { name } => todo!(),
            Expr::Assign { name, expr } => todo!(),
        }
    }

    fn push_instr(&mut self, instr: Instr) {
        self.instr_buf.push(instr);
    }

    fn flush(&mut self) -> Result<Chunk> {
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
                _ => panic!(),
            }
        }

        Ok(chunk)
    }
}
