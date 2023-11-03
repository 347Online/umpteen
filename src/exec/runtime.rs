use crate::{
    repr::{chunk::Chunk, value::Value},
    Result,
};

use super::{Program, Stack};

#[derive(Default)]
pub struct Runtime {
    program: Program,
    stack: Stack,
    index: usize,
}

impl Runtime {
    pub fn new(program: Program) -> Self {
        Runtime {
            program,
            stack: vec![],
            index: 0,
        }
    }

    pub fn exec(mut self) -> Result<Value> {
        for chunk in self.program {
            chunk.exec(&mut self.stack);
        }

        Ok(Value::Empty)
    }
}
