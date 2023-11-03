use crate::repr::{chunk::Chunk, value::Value};

pub mod compiler;
pub mod lexer;
pub mod parser;
pub mod runtime;

pub type Stack = Vec<Value>;
pub type Program = Vec<Chunk>;
