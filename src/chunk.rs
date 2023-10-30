use crate::{
    bytecode::{Bytecode, Instruction},
    value::Value,
};

pub struct Chunk<'c> {
    code: Vec<Bytecode>,
    constants: Vec<Value<'c>>,
}

impl<'c> Chunk<'c> {
    pub fn new() -> Self {
        Self {
            code: vec![],
            constants: vec![],
        }
    }

    pub fn write(&mut self, inst: Instruction) {}
}

impl<'c> Default for Chunk<'c> {
    fn default() -> Self {
        Self::new()
    }
}
