use crate::{error::Error, value::Value, Result};

use super::{AddrSize, AsBytes, Instruction, MemoryPage};

pub struct Bytes(Vec<u8>);
impl Bytes {
    pub fn new() -> Self {
        Bytes(vec![])
    }

    pub fn write<const N: usize, A: AsBytes<N>>(&mut self, data: A) -> usize {
        let offset = self.0.len();
        let bytes = data.to_bytes();

        for byte in bytes {
            self.0.push(byte)
        }

        offset
    }
}

#[derive(Debug)]
pub struct Chunk<'m> {
    //     mem: &'m mut MemoryPage,
    //     // code: Vec<Instruction>,
    //     bytes: Vec<u8>,
    //     offset: usize,
    x: &'m str,
}

// impl<'m> Chunk<'m> {
//     pub fn new(mem: &'m mut MemoryPage) -> Self {
//         Chunk {
//             mem,
//             // code: vec![],
//             bytes: vec![],
//             offset: 0,
//         }
//     }

//     // pub fn write_value(&mut self, val: Value) -> usize {
//     //     self.mem.write_value(val)
//     // }

//     pub fn write_instr(&mut self, instr: Instruction) {
//         // self.code.push(instr);
//     }

//     pub fn write<const N: usize, A: AsBytes<N>>(&mut self, arg: A) {
//         self.store_bytes(arg.to_bytes());
//     }

//     fn store_bytes<const N: usize>(&mut self, bytes: [u8; N]) {
//         for byte in bytes {
//             self.bytes.push(byte);
//         }
//     }

//     fn load_value(&self, addr: usize) -> Result<&Value> {
//         self.mem.read_value(addr).ok_or(Error::CorruptedChunk)
//     }

//     fn load_bytes<const N: usize>(&mut self) -> Result<[u8; N]> {
//         let mut bytes = [0; N];

//         for b in bytes.iter_mut() {
//             self.offset += 1;
//             let byte = self
//                 .bytes
//                 .get(self.offset - 1)
//                 .ok_or(Error::CorruptedChunk)?;
//             *b = *byte;
//         }

//         Ok(bytes)
//     }

//     fn load_byte(&mut self) -> Result<u8> {
//         Ok(self.load_bytes::<1>()?[0])
//     }

//     pub fn exec(mut self, stack: &mut Vec<Value>) -> Result<Value> {
//         let code = std::mem::take(&mut self.code);

//         for instr in code {
//             match instr {
//                 Instruction::Load => {
//                     let addr = self.load_byte()? as usize;
//                     let val = self.load_value(addr).cloned()?;
//                     stack.push(val);
//                 }
//                 Instruction::Print => {
//                     let value = stack.pop().unwrap_or(Value::Empty);
//                     println!("{}", value);
//                 }
//                 Instruction::Return => {
//                     let x = stack.pop();
//                     if let Some(val) = x {
//                         return Ok(val);
//                     } else {
//                         panic!("what?");
//                     }
//                 }
//             }
//         }

//         Ok(Value::Empty)
//     }
// }

// fn write<const N: usize, T>(values: [T; N], store: &mut Vec<T>) -> Option<usize> {
//     if store.len() + values.len() >= u8::MAX as usize {
//         None
//     } else {
//         for value in values {
//             store.push(value);
//         }
//         Some(store.len() - 1)
//     }
// }
