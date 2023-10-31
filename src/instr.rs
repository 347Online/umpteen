use std::{fmt::Display, mem::size_of};

use crate::error::Error;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    U8,
    U16,
    U32,
    Print,
    Return,
}

impl Instruction {
    pub fn offset(&self) -> usize {
        match self {
            Instruction::U8 => size_of::<u8>(),
            Instruction::U16 => size_of::<u16>(),
            Instruction::U32 => size_of::<u32>(),
            _ => 0,
        }
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

impl From<Instruction> for u8 {
    fn from(value: Instruction) -> Self {
        value as u8
    }
}
