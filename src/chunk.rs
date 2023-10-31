use crate::{instr::Instruction, value::Value};

pub type Bytecode = (Vec<Value>, Vec<Instruction>, Vec<u8>);

#[derive(Debug, Default)]
pub struct Chunk {
    pub data: Vec<Value>,
    pub code: Vec<Instruction>,
    pub bytes: Vec<u8>,
}

impl Chunk {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write_val(&mut self, val: Value) -> usize {
        self.data.push(val);
        self.data.len() - 1
    }

    pub fn write_instr(&mut self, instr: Instruction) {
        self.code.push(instr)
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.bytes.push(byte)
    }

    pub fn write_bytes(&mut self, arg: &[u8]) {
        for byte in arg {
            self.write_byte(*byte);
        }
    }

    pub fn read_byte(&self, offset: usize) -> Option<u8> {
        self.bytes.get(offset).copied()
    }

    pub fn read(&self, offset: usize, size: usize) -> Option<&[u8]>{
        let x = [0..10].into_iter().enumerate().map(|(i, _)| self.read_byte(offset+i)?);

        todo!()
    }
}
