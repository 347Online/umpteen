use std::convert::Infallible;

use super::{error::Error, instr::Instruction, value::Value, Result};

#[derive(Debug, Default)]
pub struct Chunk {
    pub data: Box<Vec<Value>>,
    pub code: Box<Vec<Instruction>>,
    pub bytes: Box<Vec<u8>>,
    offset: usize,
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

    pub fn write_bytes<I>(&mut self, bytes: I)
    where
        I: IntoIterator<Item = u8>,
    {
        for byte in bytes {
            self.write_byte(byte);
        }
    }

    fn read_val(&self, addr: usize) -> Result<Value> {
        self.data.get(addr).cloned().ok_or(Error::CorruptedChunk)
    }

    fn load_byte(&mut self) -> Result<u8> {
        let byte = self.bytes.get(self.offset).ok_or(Error::CorruptedChunk)?;
        self.offset += 1;
        Ok(*byte)
    }

    pub fn load_bytes(&mut self, size: usize) -> Result<Vec<u8>> {
        let mut bytes = vec![];

        let mut i = 0;
        while i < size {
            let byte = self.load_byte()?;
            bytes.push(byte);
            i += 1;
        }

        Ok(bytes)
    }

    pub fn exec(mut self, stack: &mut Vec<Value>) -> Result<Value> {
        let code = *std::mem::take(&mut self.code);

        macro_rules! pop {
            () => {
                Value::from(stack.pop())
            };
        }

        for instr in code {
            match instr {
                Instruction::Constant => {
                    let addr = self.load_byte()? as usize;
                    let val = self.read_val(addr)?;
                    stack.push(val);
                }
                Instruction::Print => {
                    println!("{}", pop!());
                }
                Instruction::Return => return Ok(pop!()),
            }
        }

        Ok(Value::Empty)
    }
}
