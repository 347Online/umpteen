use std::{convert::Infallible, fmt::Display};

use crate::Error;

use super::AsBytes;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Instruction {
    Constant,     // LOAD($addr); PUSH1
    Print,        // POP 1; Print to stdout
    Return = 255, // POP 1: TBD
}

impl Instruction {
    const MAX_INSTR: u8 = Instruction::Return as u8;

    pub fn arg_count(&self) -> usize {
        // Returns the number of arguments the specified instruction requires
        // Note that this is the number of distinct arguments to read, NOT the number of bytes
        use Instruction as I;
        match self {
            I::Constant => 1,
            I::Print => 0,
            I::Return => 0,
        }
    }
}

impl AsBytes<1> for Instruction {
    type Error = Error;

    fn to_bytes(self) -> [u8; 1] {
        [self as u8]
    }

    fn try_from_bytes(bytes: [u8; 1]) -> Result<Self, Self::Error> {
        let [byte] = bytes;
        let instr = match byte {
            0 => Instruction::Constant,
            1 => Instruction::Print,
            255 => Instruction::Return,

            x => return Err(Error::InvalidInstruction(x)),
        };

        Ok(instr)
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<u8> for Instruction {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (0..=Instruction::Return as u8).contains(&value) {
            // SAFETY
            // Since Instruction is repr(u8), its variants are guaranteed to be contiguous
            // Any u8 value <= Instruction::Return as u8 is valid as an instruction
            Ok(unsafe { std::mem::transmute(value) })
        } else {
            Err(Error::InvalidInstruction(value))
        }
    }
}
