use std::collections::VecDeque;

use crate::{
    repr::{chunk::Chunk, value::Value},
    Result,
};

use super::env::Environment;

#[derive(Default)]
pub struct Vm {
    env: Environment,
    stack: Vec<Value>, // env: todo!()
    program: VecDeque<Chunk>,
}

impl Vm {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write_chunk(&mut self, chunk: Chunk) {
        self.program.push_back(chunk);
    }

    fn load_chunk(&mut self) -> Option<Chunk> {
        self.program.pop_front()
    }

    pub fn exec(&mut self) -> Result<Value> {
        while let Some(chunk) = self.load_chunk() {
            chunk.exec(&mut self.stack)?;
        }

        Ok(Value::Empty)
    }
}
