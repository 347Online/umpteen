use crate::{bytecode::Bytecode, value::Value};

pub struct Chunk {
  code: Vec<Bytecode>,
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
