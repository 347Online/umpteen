use std::fmt::Display;

use crate::Error;

use super::{Address, AsBytes};

pub enum Arg {
    Address(Address),
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Instr {
    Constant,     // LOAD($addr); PUSH1
    Print,        // POP 1; Print to stdout
    Return = 255, // POP 1: TBD — Currently halts the program, return the last value on the stack
}

impl Instr {
    pub fn arg_count(&self) -> usize {
        // Returns the number of arguments the specified instruction requires
        // Note that this is the number of distinct arguments to read, NOT the number of bytes
        
        match self {
            Instr::Constant => 1,
            Instr::Print => 0,
            Instr::Return => 0,
        }
    }
}

impl AsBytes<1> for Instr {
    type Error = Error;

    fn to_bytes(self) -> [u8; 1] {
        [self as u8]
    }

    fn try_from_bytes(bytes: [u8; 1]) -> Result<Self, Self::Error> {
        let [byte] = bytes;
        let instr = match byte {
            0 => Instr::Constant,
            1 => Instr::Print,
            255 => Instr::Return,

            x => return Err(Error::InvalidInstruction(x)),
        };

        Ok(instr)
    }
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
