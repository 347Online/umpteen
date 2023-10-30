use crate::{bytecode::{Instruction, Argument}, value::Value};

#[derive(Debug)]
pub struct Chunk {
    instructions: Vec<Instruction>,
    data: Vec<Argument>,
    constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            instructions: vec![],
            data: vec![],
            constants: vec![],
        }
    }

    pub fn add_constant(&mut self, constant: Value) -> u8 {
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

    pub fn consume(self) -> (Vec<Instruction>, Vec<Argument>, Vec<Value>) {
        (self.instructions, self.data, self.constants)
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}

