use crate::{
    error::CompilerError,
    repr::{
        ast::{expr::Expr, stmt::Stmt},
        bytecode::{chunk::Chunk, instruction::Instr},
        value::Value,
    },
};

use super::parse::Ast;

#[derive(Debug, Default)]
pub struct Program {
    chunks: Vec<Chunk>,
    v_table: Vec<Value>,
}

impl Program {
    pub fn write_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }

    pub fn write_value(&mut self, value: Value) {
        self.v_table.push(value);
    }

    pub fn v_table(&self) -> usize {
        self.v_table.len()
    }
}

#[derive(Debug, Default)]
pub struct Compiler<'c> {
    program: Program,

    instr_buf: Vec<Instr>,
    name_buf: Vec<&'c str>,
}

impl<'c> Compiler<'c> {
    pub fn new() -> Self {
        Compiler {
            program: Program::default(),

            instr_buf: vec![],
            name_buf: vec![],
        }
    }

    pub fn compile(mut self, ast: Ast<'c>) -> Result<Program, CompilerError> {
        for stmt in ast {
            self.statement(stmt)?;
        }

        self.flush();

        dbg!(&self);

        Ok(self.program)
    }

    fn statement(&mut self, stmt: Stmt<'c>) -> Result<(), CompilerError> {
        match stmt {
            Stmt::Expr(expr) => self.expression(expr)?,
            Stmt::Print(expr) => {
                self.expression(expr)?;
                self.push_instr(Instr::Print);
            }
            Stmt::Exit => self.push_instr(Instr::Exit),
            Stmt::Declare(name, maybe_expr) => {
                self.push_instr(Instr::Let);
                self.push_name(name);

                if let Some(expr) = maybe_expr {
                    self.push_instr(Instr::Set);
                    self.push_name(name);
                    self.expression(expr)?;
                }
            }
        }

        Ok(())
    }

    fn expression(&mut self, expr: Expr<'c>) -> Result<(), CompilerError> {
        match expr {
            Expr::Value(value) => {
                self.push_instr(Instr::Push);
                self.push_value(value);
            }
            Expr::Assign { target, expr } => {
                self.push_instr(Instr::Set);
                self.expression(*target)?;
                self.expression(*expr)?;
            }
            Expr::Ident { name } => {
                self.push_instr(Instr::Get);
                self.push_name(name);
            }
            Expr::UnOp { expr, op } => todo!(),
            Expr::BinOp { left, right, op } => todo!(),
        }

        Ok(())
    }

    fn push_instr(&mut self, instr: Instr) {
        self.instr_buf.push(instr);
    }

    fn push_name(&mut self, name: &'c str) {
        self.name_buf.push(name);
    }

    fn push_value(&mut self, value: Value) {
        self.program.v_table.push(value);
    }

    fn flush(&mut self) {
        let instr_buf = std::mem::take(&mut self.instr_buf);
        let name_buf = std::mem::take(&mut self.name_buf);

        let mut name_pos = 0;
        let mut value_pos = 0;

        macro_rules! name {
            () => {{
                let name = name_buf[name_pos];
                name_pos += 1;
                name
            }};
        }

        let mut chunk = Chunk::new();

        for instr in instr_buf {
            chunk.write_instr(instr);

            match instr {
                Instr::Let => {
                    let name = name_buf[name_pos];
                    name_pos += 1;

                    chunk.write_bytes(name.as_bytes());
                }
                Instr::Push => {
                    chunk.write_byte(value_pos);
                    value_pos += 1;
                }
                Instr::Set => {
                    name_pos += 1;
                    value_pos += 1;
                }
                Instr::Get => {
                    let name = name_buf[name_pos];
                    name_pos += 1;

                    chunk.write_bytes(name.as_bytes());
                }

                _ => (),
            }
        }

        self.program.write_chunk(chunk);
    }
}
