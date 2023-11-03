use crate::{prelude::*, datatype::repr::AsBytes};

use super::instruction::Instruction;

#[derive(Debug, Default)]
pub struct Chunk {
    pub data: Box<Vec<Value>>,
    pub code: Box<Vec<Instruction>>,
    pub bytes: Box<Vec<u8>>,
    offset: usize,
    packed: bool,
}

impl Chunk {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write_value(&mut self, val: Value) -> usize {
        self.data.push(val);
        self.data.len() - 1
    }

    pub fn write_instr(&mut self, instr: Instruction) {
        self.code.push(instr)
    }

    pub fn write_arg<const N: usize, A: AsBytes<N>>(&mut self, arg: A) {
        self.store_bytes(arg.to_bytes());
    }

    fn store_bytes<const N: usize>(&mut self, bytes: [u8; N]) {
        for byte in bytes {
            self.bytes.push(byte);
        }
    }

    fn load_value(&self, addr: usize) -> Result<Value> {
        self.data.get(addr).cloned().ok_or(Error::CorruptedChunk)
    }

    fn load_bytes<const N: usize>(&mut self) -> Result<[u8; N]> {
        let mut bytes = [0; N];

        for b in bytes.iter_mut() {
            self.offset += 1;
            let byte = self
                .bytes
                .get(self.offset - 1)
                .ok_or(Error::CorruptedChunk)?;
            *b = *byte;
        }

        Ok(bytes)
    }

    fn load_byte(&mut self) -> Result<u8> {
        Ok(self.load_bytes::<1>()?[0])
    }

    pub fn exec(mut self, stack: &mut Vec<Value>) -> Result<Value> {
        let code = *std::mem::take(&mut self.code);

        for instr in code {
            match instr {
                Instruction::Constant => {
                    let addr = self.load_byte()? as usize;
                    let val = self.load_value(addr)?;
                    stack.push(val);
                }
                Instruction::Print => {
                    println!("{}", Value::from(stack.pop()));
                }
                Instruction::Return => {
                    let x = stack.pop();
                    if let Some(val) = x {
                        return Ok(val);
                    } else {
                        panic!("what?");
                    }
                }
            }
        }

        Ok(Value::Empty)
    }
}
