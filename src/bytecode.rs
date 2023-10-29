use crate::error::UmpError;

#[repr(u8)]
pub enum Opcode {
    Constant,
    Return,
}

const MAX_OPCODE: u8 = Opcode::Return as u8;

impl TryFrom<u8> for Opcode {
    type Error = UmpError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (0..=MAX_OPCODE).contains(&value) {
            // SAFETY:
            // MAX_OPCODE is derived from Opcode::Return, the final variant
            // Since Opcode is defined as repr(u8), the variants form a contiguous range
            // any u8 value less than or equal to Opcode::Return as u8 is a valid instruction
            Ok(unsafe { std::mem::transmute(value) })
        } else {
            Err(UmpError::invalid_opcode(value))
        }
    }
}

pub enum Value {
    Empty,
    Boolean(bool),
    Number(f64),
    String(Box<String>),
}

pub struct Chunk {
    code: Vec<Opcode>,
    constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: vec![],
            constants: vec![],
        }
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}
