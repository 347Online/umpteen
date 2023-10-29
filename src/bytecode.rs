use crate::error::{UmpError, UmpResult};

#[derive(Clone, Copy)]
pub enum Instruction {
    Constant,
    Return,
}

impl TryFrom<u8> for Instruction {
    type Error = UmpError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (0..=Instruction::Return as u8).contains(&value) {
            // SAFETY:
            // Since Instruction is defined as repr(u8), the variants form a contiguous range
            // any u8 value less than or equal to Opcode::Return as u8 is valid as an instruction
            Ok(unsafe { std::mem::transmute(value) })
        } else {
            Err(UmpError::invalid_instruction(value))
        }
    }
}

#[derive(Clone, Copy)]
pub struct Argument(u8);

impl From<u8> for Argument {
    fn from(value: u8) -> Self {
        Argument(value)
    }
}

#[derive(Clone, Copy)]
pub union Bytecode {
    inst: Instruction,
    arg: Argument,
}

impl Bytecode {
    pub fn code(self) -> UmpResult<Instruction> {
        // SAFETY:
        // Failed conversion safely returns an error
        (unsafe { self.inst } as u8).try_into()
    }

    pub fn data(self) -> Argument {
        // SAFETY:
        // This operation is infallible
        unsafe { self.arg }
    }
}

impl From<Instruction> for Bytecode {
    fn from(value: Instruction) -> Self {
        Bytecode { inst: value }
    }
}

impl From<Argument> for Bytecode {
    fn from(value: Argument) -> Self {
        Bytecode { arg: value }
    }
}
