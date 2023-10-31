use crate::{instr::Instruction, value::Value};

#[derive(Debug)]
pub struct Chunk {
    values: Vec<Value>,
    code: Vec<Instruction>,
    args: Vec<u8>,
}

pub type Bytecode = (Vec<Value>, Vec<Instruction>, Vec<u8>);

impl Chunk {
    pub fn new() -> Self {
        Self {
            values: vec![],
            code: vec![],
            args: vec![],
        }
    }

    pub fn write_val(&mut self, constant: Value) -> u8 {
        let addr = self.values.len() as u8;
        self.values.push(constant);
        addr
    }

    pub fn write_instr(&mut self, instr: Instruction) {
        self.code.push(instr)
    }

    pub fn write_byte(&mut self, arg: u8) {
        self.args.push(arg)
    }

    pub fn consume(self) -> Bytecode {
        (self.values, self.code, self.args)
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}
