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
pub struct Compiler {
    program: Vec<Chunk>,
    index: usize,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            program: vec![Chunk::new()],
            index: 0,
        }
    }

    pub fn compile(mut self, ast: Ast) -> Result<Vec<Chunk>, CompilerError> {
        for stmt in ast {
            self.statement(stmt)?;
        }

        dbg!(&self);

        Ok(self.program)
    }

    fn statement(&mut self, stmt: Stmt) -> Result<(), CompilerError> {
        match stmt {
            Stmt::Expr(expr) => self.expression(expr)?,
            Stmt::Print(expr) => {
                self.expression(expr)?;
                self.write_instr(Instr::Print);
            }
            Stmt::Declare(name, maybe_expr) => self.declaration(name, maybe_expr)?,
            Stmt::Return(_) => todo!(),
            Stmt::Exit => self.write_instr(Instr::Exit),
        }

        Ok(())
    }

    fn expression(&mut self, expr: Expr) -> Result<(), CompilerError> {
        match expr {
            Expr::Value(value) => {
                self.write_instr(Instr::Push);
                self.write_value(value);
            }
            Expr::Assign { target, expr } => {
                self.write_instr(Instr::Set);
                self.expression(*target)?;
                self.expression(*expr)?;
            }
            Expr::Ident { name } => {
                self.write_instr(Instr::Get);
                self.write_name(name);
            }
            Expr::UnOp { expr, op } => todo!(),
            Expr::BinOp { left, right, op } => todo!(),
        }

        Ok(())
    }

    fn declaration(&mut self, name: &str, maybe_expr: Option<Expr>) -> Result<(), CompilerError> {
        self.write_instr(Instr::Let);
        self.write_name(name);

        if let Some(expr) = maybe_expr {
            self.write_instr(Instr::Set);
            self.write_name(name);
            self.expression(expr)?;
        }

        Ok(())
    }

    fn write_instr(&mut self, instr: Instr) {
        let chunk = self.current_chunk();

        chunk.write_byte(instr as u8);
    }

    fn write_name(&mut self, name: &str) {
        let chunk = self.current_chunk();

        chunk.write_bytes(name.as_bytes());
    }

    fn write_value(&mut self, value: Value) {
        let chunk = self.current_chunk();
        
        chunk.write_byte(value.designation());

        match value {
            Value::Empty => chunk.write_byte(0),
            Value::Boolean(x) => chunk.write_byte(x as u8),
            Value::Number(x) => chunk.write_bytes(&x.to_be_bytes()),
            Value::String(x) => {
                let str_bytes = &x.into_bytes();
                chunk.write_bytes(&str_bytes.len().to_be_bytes());
                chunk.write_bytes(str_bytes);
            }

            Value::Object(_) => todo!(),
        }
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        self.program
            .get_mut(self.index)
            .expect("Compiler should always have access to a chunk")
    }
}
