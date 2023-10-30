use crate::error::UmpError;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Constant,
    Return,
}

impl TryFrom<u8> for Instruction {
    type Error = UmpError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (0..=Instruction::Return as u8).contains(&value) {
            // SAFETY
            // Since Instruction is repr(u8), its variants are guaranteed to be contiguous
            // Any u8 value <= Instruction::Return as u8 is valid as an instruction
            Ok(unsafe { std::mem::transmute(value) })
        } else {
            Err(UmpError::invalid_instruction(value))
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Argument(pub u8);

impl From<u8> for Argument {
    fn from(value: u8) -> Self {
        Argument(value)
    }
}
