use crate::repr::bytecode::{chunk::Chunk, instruction::Instr};

use super::{env::Memory, parse::Ast};

pub struct Compiler<'c> {
    mem: Option<&'c Memory<'c>>,
    program: Vec<Chunk>,

    instr_buf: Vec<Instr>,
    data_buf: Vec<u8>,
}

impl<'c> Compiler<'c> {
    pub fn new() -> Self {
        Compiler {
            mem: None,
            program: vec![],

            instr_buf: vec![],
            data_buf: vec![],
        }
    }

    pub fn compile(mut self, ast: Ast, mem: &'c mut Memory) -> Vec<Chunk> {
        self.mem_return(mem);

        for stmt in ast {
            // self.statement(stmt);
        }

        // self.flush();

        self.take_program()
    }

    fn mem_take(&mut self) -> &'c mut Memory {
        self.mem.as_mut().take().expect("Compiler memory missing")
    }

    fn mem_return(&mut self, mem: &'c mut Memory) {
        self.mem = Some(mem);
    }

    fn take_program(&mut self) -> Vec<Chunk> {
        std::mem::take(&mut self.program)
    }
}
