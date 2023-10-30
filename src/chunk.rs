use crate::{bytecode::{Instruction, Argument}, value::Value};

#[derive(Debug)]
pub struct Chunk<'c> {
    instructions: Vec<Instruction>,
    data: Vec<Argument>,
    constants: Vec<Value<'c>>,
}

impl<'c> Chunk<'c> {
    pub fn new() -> Self {
        Self {
            instructions: vec![],
            data: vec![],
            constants: vec![],
        }
    }

    pub fn instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }

    pub fn data(&self) -> &Vec<Argument> {
        &self.data
    }

    pub fn constants(&self) -> &Vec<Value> {
        &self.constants
    }

    pub fn add_constant(&mut self, constant: Value<'c>) -> u8 {
        let addr = self.constants.len() as u8;
        self.constants.push(constant);
        addr
    }

    pub fn write_inst(&mut self, inst: Instruction) {
        self.instructions.push(inst)
    }
    
    pub fn write_arg(&mut self, arg: Argument) {
        self.data.push(arg)
    }
}

impl<'c> Default for Chunk<'c> {
    fn default() -> Self {
        Self::new()
    }
}

