use crate::{Error, Result};

use super::{AsBytes, Instruction};

#[derive(Debug, Clone, Copy)]
pub enum AddrMode {
    Byte,
    Word,
    Long,
}

impl AddrMode {
    pub const BYTE: usize = u8::MAX as usize + 1;
    pub const WORD: usize = u16::MAX as usize + 1;
    pub const LONG: usize = u32::MAX as usize + 1;

    pub const fn size(&self) -> usize {
        match self {
            AddrMode::Byte => Self::BYTE,
            AddrMode::Word => Self::WORD,
            AddrMode::Long => Self::LONG,
        }
    }
}

pub type Bytecode = Vec<u8>;

#[derive(Debug)]
pub struct Chunk {
    addr_mode: AddrMode,
    bytes: Bytecode,
}

impl Chunk {
    pub fn new(addr_mode: AddrMode) -> Self {
        Chunk {
            addr_mode,
            bytes: vec![],
        }
    }

    pub fn write_instr(&mut self, instr: Instruction) {
        self.write_arg(instr)
    }

    pub fn write_arg<const N: usize, T: AsBytes<N>>(&mut self, arg: T) {
        let bytes = arg.to_bytes();

        for byte in bytes {
            self.bytes.push(byte);
        }
    }

    pub fn write_addr(&mut self, addr: usize) {
        // Special helper function for writing addresses which have to conform to the address mode

        match self.addr_mode {
            AddrMode::Byte => self.write_arg(addr as u8),
            AddrMode::Word => self.write_arg(addr as u16),
            AddrMode::Long => self.write_arg(addr as u32),
        };
    }

    pub fn consume(self) -> (AddrMode, Bytecode) {
        let Chunk { addr_mode, bytes } = self;
        (addr_mode, bytes)
    }

    pub fn read_instr(&self, index: usize) -> Result<Instruction> {
        // Attempts to read one bytecode instruction
        let byte = self.bytes.get(index).ok_or(Error::CorruptedChunk)?;

        Instruction::try_from_bytes([*byte])
    }

    pub fn read_arg<const N: usize, T: AsBytes<N>>(&self, index: usize) -> Result<T> {
        let bytes = self.read_bytes::<N>(index)?;
        T::try_from_bytes(bytes).map_err(|e| {
            eprintln!("{}", e);
            Error::CorruptedChunk
        })
    }

    fn read_bytes<const N: usize>(&self, start: usize) -> Result<[u8; N]> {
        let mut bytes = [0; N];
        let mut i = start;
        for byte in bytes.iter_mut() {
            *byte = *self.read_byte(i).ok_or(Error::CorruptedChunk)?;
            i += 1;
        }

        Ok(bytes)
    }

    fn read_byte(&self, index: usize) -> Option<&u8> {
        self.bytes.get(index)
    }

    // fn read_word(&self, index: usize) -> Result<[u8; 2]> {
    //     self.read_bytes::<2>(index)
    // }

    // fn read_long(&self, index: usize) -> Result<[u8; 4]> {
    //     self.read_bytes::<4>(index)
    // }
}

// pub struct ChunkIntoIterator {
//     chunk: Chunk,
//     offset: usize,
// }

// impl Iterator for ChunkIntoIterator {
//     type Item = Instruction;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.chunk.read_byte()
//     }
// }

// impl IntoIterator for Chunk {
//     type Item = Instruction;

//     type IntoIter = ChunkIntoIterator;

//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }
