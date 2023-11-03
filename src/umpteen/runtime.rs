use crate::{Program, Result, Value};

pub type Stack = Vec<Value>;
pub type Program = Vec<Chunk>;

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
