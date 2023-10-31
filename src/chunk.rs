use crate::{instr::Instruction, value::Value, Result};

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

    pub fn exec(self) -> Result<Value> {
        for instr in self {
            
        }

        Ok(Value::Empty)
    }
}

pub struct ChunkIntoIterator {
    chunk: Chunk,
    index: usize,
    offset: usize,
}

impl IntoIterator for Chunk {
    type Item = Instruction;
    type IntoIter = ChunkIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        ChunkIntoIterator {
            chunk: self,
            index: 0,
            offset: 0,
        }
    }
}

impl Iterator for ChunkIntoIterator {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        let instr = self.chunk.code.get(self.index)?;
        self.index += 1;
        self.offset += instr.offset();

        Some(*instr)
    }
}
