use crate::{bytecode::Chunk, value::Value, Result};

pub type Program = Vec<Chunk>;
pub type Stack = Vec<Value>;

#[derive(Default)]
pub struct Runtime {
    stack: Stack,
    program: Program,
    program_counter: usize,
}

impl Runtime {
    pub fn new(program: Program) -> Self {
        Runtime {
            stack: vec![],
            program,
            program_counter: 0,
        }
    }

    pub fn exec(mut self) -> Result<Value> {
        for chunk in self.program {
            chunk.exec(&mut self.stack);
        }

        Ok(Value::Empty)
    }
}
