use std::ops::{Deref, DerefMut};

use crate::{ast::Expr, value::Value, Result};

use super::{AddrMode, Chunk, Instruction};

pub type Bytecode<const N: usize> = [Chunk; N];

pub type Address = usize;

#[derive(Debug, Default)]
pub struct Memory(Vec<Value>);

impl Memory {
    pub fn offset(&self) -> usize {
        self.0.len()
    }

    pub fn store(&mut self, value: Value) -> usize {
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
pub struct Compiler {
    mem: Memory,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            mem: Memory::default(),
        }
    }

    pub fn compile_expr(&mut self, expr: Expr) -> Result<Chunk> {
        let mut instr_buf = vec![];
        let mut arg_buf = vec![];

        match expr {
            Expr::Value(value) => {
                if value != Value::Empty {
                    let addr = self.mem.store(value);

                    instr_buf.push(Instruction::Constant);
                    arg_buf.push(addr);
                }
            }
            Expr::UnOp { expr, op } => todo!(),
            Expr::BinOp { left, right, op } => todo!(),
            Expr::Ident { name } => todo!(),
            Expr::Assign { name, expr } => todo!(),
        }

        let addr_mode = match arg_buf.len() {
            x if x < AddrMode::BYTE => AddrMode::Byte,
            x if x < AddrMode::WORD => AddrMode::Word,
            x if x < AddrMode::LONG => AddrMode::Long,
            _ => panic!("out of memory somehow, incredible"),
        };

        instr_buf.push(Instruction::Return);
        let mut chunk = Chunk::new(addr_mode);

        let mut arg_pos = 0;

        let mut read_arg = || {
            let arg = arg_buf[arg_pos];
            arg_pos += 1;
            arg
        };

        for instr in instr_buf {
            chunk.write_instr(instr);
            match instr {
                Instruction::Constant => {
                    let addr = read_arg();
                    chunk.write_addr(addr);
                }
                _ => (),
            }
        }

        Ok(chunk)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::Expr,
        value::{Object, Value},
    };

    use super::Compiler;

    #[test]
    fn some_fn() {
        let string = String::from("Hello world");
        let boxed_str = Box::new(string);
        let obj = Object::String(boxed_str);
        let val = Value::Object(obj);
        let ast = Expr::Value(val);

        let mut cp = Compiler::new();
        let chunk = cp.compile_expr(ast).unwrap();
        dbg!(chunk);
    }
}
