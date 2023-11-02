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

    pub fn write_bytes<const N: usize>(&mut self, bytes: [u8; N]) {
        for byte in bytes {
            self.write_byte(byte);
        }
    }

    fn read_val(&self, addr: usize) -> Result<Value> {
        self.data.get(addr).cloned().ok_or(Error::CorruptedChunk)
    }

    pub fn load_bytes<const N: usize>(&mut self) -> Result<[u8; N]> {
        let mut bytes = [0; N];

        for byte in bytes.iter_mut() {
            let b = self.bytes.get(self.offset).ok_or(Error::CorruptedChunk)?;
            self.offset += 1;
            *byte = *b;
        }

        Ok(bytes)
    }

    pub fn load_byte(&mut self) -> Result<u8> {
        Ok(self.load_bytes::<1>()?[0])
    }

    pub fn exec(mut self, stack: &mut Vec<Value>) -> Result<Value> {
        let code = *std::mem::take(&mut self.code);

        macro_rules! pop {
            () => {
                Ok(Value::from(stack.pop()))
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
                    println!("{}", pop!()?);
                }
                Instruction::Return => return pop!(),
            }
        }

        Ok(Value::Empty)
    }
}
