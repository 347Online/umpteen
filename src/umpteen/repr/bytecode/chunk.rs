use crate::error::RuntimeError;

use super::instruction::Instr;

#[derive(Debug)]
pub struct Chunk {
    bytecode: Vec<u8>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk { bytecode: vec![] }
    }

    pub fn write_instr(&mut self, instr: Instr) {
        self.write_byte(instr as u8);
    }

    fn write_byte(&mut self, byte: u8) {
        self.bytecode.push(byte);
    }

    fn write_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.write_byte(*byte);
        }
    }

    pub fn read_byte(&self, offset: usize) -> Result<&u8, RuntimeError> {
        self.bytecode
            .get(offset)
            .ok_or(RuntimeError::ChunkReadError)
    }

    pub fn read_bytes(&self, offset: usize, count: usize) -> Result<&[u8], RuntimeError> {
        let end = offset + count;
        self.bytecode
            .get(offset..count)
            .ok_or(RuntimeError::ChunkReadError)
    }
}
