use crate::{bytecode::Chunk, value::Value, Result};

pub type Program = Vec<Chunk>;
pub type Stack = Vec<Value>;

#[derive(Default)]
pub struct Runtime {
    stack: Stack,
    program: Program,
    index: usize,
}

impl Runtime {
    pub fn new(program: Program) -> Self {
        Runtime {
            stack: vec![],
            program,
            index: 0,
        }
    }

    pub fn exec(mut self) -> Result<Value> {
        let mut program = std::mem::take(&mut self.program).into_iter();

        let value = loop {
            let Some(chunk) = program.next() else {
                break Value::Empty;
            };
        };

        Ok(value)
    }
}
