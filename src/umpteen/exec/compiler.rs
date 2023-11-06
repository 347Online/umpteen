use crate::{
    error::CompilerError,
    repr::{
        ast::{expr::Expr, stmt::Stmt},
        bytecode::{
            argument::Arg,
            chunk::{AddrMode, Chunk},
            instruction::Instr,
        },
        value::Value,
    },
};

use super::{env::Memory, parse::Ast};

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
    arg_buf: Vec<Arg>,
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
                self.expression(expr)?;
            }
            Stmt::Print(expr) => {
                self.expression(expr)?;
                self.push_instr(Instr::Print);
            }
            Stmt::Exit(x) => {
                if let Some(expr) = x {
                    self.expression(expr)?;
                };
                self.push_instr(Instr::Exit);
            }
            Stmt::Declare(ident, x) => {
                if let Expr::Ident { name } = ident {
                    let addr = self.declaration(name)?;
                    if let Some(expr) = x {
                        self.expression(expr)?;
                        self.assignment(addr);
                    }
                } else {
                    Err(CompilerError::InvalidIdentifier)?
                }
            }
        }

        Ok(())
    }

    fn variable_ref(&mut self, expr: Expr<'c>) -> Result<(), CompilerError> {
        if let Expr::Ident { name } = expr {
            let addr = self.mem.retrieve(name)?;
            self.arg_buf.push(Arg::Address(addr));
        } else {
            Err(CompilerError::InvalidIdentifier)?
        }

        Ok(())
    }

    fn expression(&mut self, expr: Expr<'c>) -> Result<(), CompilerError> {
        match expr {
            Expr::Constant(value) => {
                if value != Value::Empty {
                    self.constant(value);
                }
            }
            Expr::UnOp { expr, op } => todo!(),
            Expr::BinOp { left, right, op } => todo!(),
            Expr::Ident { name } => {
                self.push_instr(Instr::Push);
                let addr = self.mem.retrieve(name)?;
                self.push_arg(Arg::Address(addr));
            }
            Expr::Assign { name, expr } => {
                self.push_instr(Instr::Push);
                let addr = self.mem.retrieve(name)?;
                self.expression(*expr)?;
                self.assignment(addr)?;
            }
        }

        Ok(())
    }

    fn declaration(&mut self, name: &'c str) -> Result<usize, CompilerError> {
        let addr = self.mem.declare(name)?;
        self.push_arg(Arg::Address(addr));
        Ok(addr)
    }

    fn assignment(&mut self, addr: usize) -> Result<(), CompilerError> {
        self.push_arg(Arg::Address(addr));

        Ok(())
    }

    fn constant(&mut self, value: Value) {
        self.push_instr(Instr::Constant);
        let addr = self.mem.declare_constant(value);
        self.push_arg(Arg::Address(addr));
    }

    fn push_instr(&mut self, instr: Instr) {
        self.instr_buf.push(instr);
    }

    fn push_arg(&mut self, arg: Arg) {
        self.arg_buf.push(arg);
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
        let arg_buf = std::mem::take(&mut self.arg_buf);

        for instr in instr_buf {
            chunk.write_instr(instr);
            let count = instr.arg_count();

            for _ in 0..count {
                match arg_buf[arg_pos] {
                    Arg::Address(addr) => chunk.write_addr(addr),
                }
                arg_pos += 1;
            }

            // match instr {
            //     Instr::Constant => {
            //         let addr = self.arg_buf[arg_pos];
            //         arg_pos += 1;
            //         chunk.write_addr(addr);
            //     }
            //     _ => (),
            // }
        }

        self.chunks.push(chunk);
    }
}
