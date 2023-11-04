use std::{
    array::from_fn,
    ops::{Deref, DerefMut},
};

use crate::{
    ast::{Expr, Unary},
    value::Value,
    Result,
};

use super::Chunk;

#[derive(Debug, Default)]
pub struct CompilerMemory(Vec<MemoryPage>);

impl Deref for CompilerMemory {
    type Target = Vec<MemoryPage>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CompilerMemory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub type Memory<const N: usize> = [MemoryPage; N];

const SM: usize = u8::MAX as usize + 1;
const MD: usize = u16::MAX as usize + 1;
const LG: usize = u32::MAX as usize + 1;

#[derive(Debug)]
enum AddrSize {
    Byte,
    Word,
    Long,
}

#[derive(Debug)]
enum MemData {
    Small([Option<Value>; SM]),
    Medium([Option<Value>; MD]),
    Large([Option<Value>; LG]),
}

#[derive(Debug)]
pub struct MemoryPage {
    pub size: AddrSize,
    offset: usize,
    data: MemData,
}

impl MemoryPage {
    pub fn new(size: AddrSize) -> Self {
        let f = |_| None;

        MemoryPage {
            size,
            offset: 0,
            data: match size {
                AddrSize::Byte => MemData::Small(from_fn(f)),
                AddrSize::Word => MemData::Medium(from_fn(f)),
                AddrSize::Long => MemData::Large(from_fn(f)),
            },
        }
    }

    pub fn get(&self, addr: usize) -> Option<&Value> {
        todo!()
    }

    pub fn set(&mut self, value: Value) -> usize {
        todo!()
    }
}

#[derive(Debug, Default)]
pub struct Compiler<'c> {
    memory: CompilerMemory,
    values: Vec<Value>,
    code: Vec<Chunk<'c>>,
}

impl<'c> Compiler<'c> {
    fn new() -> Self {
        Self::default()
    }

    fn store(&mut self, value: Value) -> usize {
        self.values.push(value);
        self.values.len() - 1
    }

    pub fn compile<'cc, const N: usize>(mut self, ast: Expr) -> Result<Program<N>> {
        match ast {
            Expr::Value(val) => {
                let addr = self.store(val);
            }
            Expr::UnOp { expr, op } => {
                match op {
                    Unary::Not => !expr.eval()?,
                    Unary::Negate => {
                        let val = expr.eval()?;
                        (-val)?
                    }
                };
            }
            Expr::BinOp { left, right, op } => todo!(),
            Expr::Ident { name } => todo!(),
            Expr::Assign { name, expr } => todo!(),
        };

        let memory = (*self.memory)
            .try_into()
            .unwrap_or_else(|v: Vec<MemoryPage>| panic!("Ack!"));

        let code = (self.code)
            .try_into()
            .unwrap_or_else(|v: Vec<Chunk<'c>>| panic!("Ack!"));

        Ok(Program { memory, code })
    }
}

pub struct Program<'p, const N: usize> {
    memory: Memory<N>,
    code: [Chunk<'p>; N],
}
