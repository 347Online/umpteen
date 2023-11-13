use std::collections::HashSet;

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
pub struct Compiler<'c> {
    program: Vec<Chunk>,
    index: usize,

    names: HashSet<&'c str>,
    code_buf: Vec<u8>,
}

impl Compiler<'_> {
    pub fn new() -> Self {
        Compiler {
            program: vec![Chunk::new()],
            index: 0,

            names: HashSet::new(),
            code_buf: vec![],
        }
    }

    pub fn compile(mut self, ast: Ast) -> Result<Vec<Chunk>, CompilerError> {
        for stmt in ast {
            self.statement(stmt)?;
        }

        Ok(self.program)
    }

    fn statement(&mut self, stmt: Stmt) -> Result<(), CompilerError> {
        match stmt {
            Stmt::Expr(expr) => {
                self.expression(expr)?;
            }
            Stmt::Print(expr) => {
                self.expression(expr)?;
                self.push_instr(Instr::Print);
            }
            Stmt::Declare(name, maybe_expr) => self.declaration(name, maybe_expr)?,
            Stmt::Return(_) => todo!(),
            Stmt::Exit => self.push_instr(Instr::Exit),
        }

        Ok(())
    }

    fn expression(&mut self, expr: Expr) -> Result<(), CompilerError> {
        match expr {
            Expr::Value(value) => {
                self.push_instr(Instr::Push);
                // self.write_value(value); // Is this right? I don't think it is
            }
            Expr::Assign { target, expr } => {
                self.push_instr(Instr::Set); // TODO: self.assign
                self.expression(*target)?;
                self.expression(*expr)?;
            }
            Expr::Ident { name } => {}
            Expr::UnOp { expr, op } => todo!(),
            Expr::BinOp { left, right, op } => todo!(),
        }

        Ok(())
    }

    fn declaration(&mut self, name: &str, maybe_expr: Option<Expr>) -> Result<(), CompilerError> {
        self.push_instr(Instr::Let);

        if let Some(expr) = maybe_expr {
            self.push_instr(Instr::Set);

            self.expression(expr)?;
        }

        Ok(())
    }

    fn put_name(&mut self, name: &str) {
        self.names.insert(name);
    }

    fn push_instr(&mut self, instr: Instr) {
        self.push(instr as u8);
    }

    fn push(&mut self, byte: u8) {
        self.code_buf.push(byte);
    }

    // fn write_name(&mut self, name: &str) {
    //     let chunk = self.current_chunk();
    //     // // TODO: should make a names lookup table instead of writing the bytes everytime
    //     // let bytes = name.as_bytes();
    //     // chunk.write_byte(bytes.len() as u8);
    //     // chunk.write_bytes(bytes);
    // }

    // fn write_value(&mut self, value: Value) {
    //     let chunk = self.current_chunk();

    //     chunk.write_byte(value.designation());

    //     match value {
    //         Value::Empty => chunk.write_byte(0),
    //         Value::Boolean(x) => chunk.write_byte(x as u8),
    //         Value::Number(x) => chunk.write_bytes(&x.to_be_bytes()),
    //         Value::String(x) => {
    //             let str_bytes = &x.into_bytes();
    //             chunk.write_bytes(&str_bytes.len().to_be_bytes());
    //             chunk.write_bytes(str_bytes);
    //         }

    //         Value::Object(_) => todo!(),
    //     }
    // }

    fn current_chunk(&mut self) -> &mut Chunk {
        self.program
            .get_mut(self.index)
            .expect("Compiler should always have access to a chunk")
    }

    fn flush(&mut self) {
        let chunk = self.current_chunk();


    }
}
