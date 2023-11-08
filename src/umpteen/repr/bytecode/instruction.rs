use std::fmt::Display;

use crate::error::RuntimeError;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Instr {
    Let, // LET  | Create a new variable with ($name)
    Push,    // PUSH | Push 1 $(val) onto the stack
    Pop,     // POP  | Pop 1 value from the stack, discarding it
    Set,     // SET  | Set variable ($name) to ($value)
    Get,     // GET  | Load a value at ($addr)
    Print,   // PRNT | Pop 1 Print to stdout
    Exit,    // EXIT | Halt the program
}

impl Instr {
    const MAX: u8 = Instr::Exit as u8;

    pub fn arg_count(&self) -> usize {
        // Returns the number of byte arguments for the instruction

        match self {
            Instr::Push => 1,

            Instr::Set => 2,

            _ => 0,
        }
    }
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<u8> for Instr {
    type Error = RuntimeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (0..=Instr::MAX).contains(&value) {
            // SAFETY:
            // MAX_OPCODE is derived from Instr::Exit, the final variant
            // Since Instr is defined as repr(u8), the variants form a contiguous range
            // any u8 value less than or equal to Instr::Exit as u8 is a valid instruction
            let instruction = unsafe { std::mem::transmute(value) };
            Ok(instruction)
        } else {
            Err(RuntimeError::InvalidInstruction(value))
        }
    }
}
