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

#[derive(Debug)]
pub struct Chunk {
    addr_mode: AddrMode,
    bytes: Vec<u8>,
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

    pub fn write_arg<const N: usize, Arg: AsBytes<N>>(&mut self, arg: Arg) {
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
}
