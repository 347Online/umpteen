use std::{
    array::from_fn,
    ops::{Deref, DerefMut},
};

use crate::{
    ast::{Expr, Unary},
    bytecode::{chunk, Args, Bytes},
    value::Value,
    Result,
};

use super::{Chunk, Instruction};

#[derive(Debug, Default)]
pub struct CompilerMemory(Vec<MemoryPage>);

pub type Memory<const N: usize> = [MemoryPage; N];

const BYTE: usize = u8::MAX as usize + 1;
const WORD: usize = u16::MAX as usize + 1;
const LONG: usize = u32::MAX as usize + 1;

#[derive(Debug, Clone, Copy)]
pub enum AddrSize {
    Byte,
    Word,
    Long,
}

impl AddrSize {
    const fn size(&self) -> &usize {
        match self {
            AddrSize::Byte => &BYTE,
            AddrSize::Word => &WORD,
            AddrSize::Long => &LONG,
        }
    }
}

impl Deref for AddrSize {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        self.size()
    }
}

pub struct MemAddr(AddrSize, usize);

#[derive(Debug)]
enum MemData {
    Small(Box<[Option<Value>; BYTE]>),
    Medium(Box<[Option<Value>; WORD]>),
    Large(Box<[Option<Value>; LONG]>),
}

#[derive(Debug)]
pub struct MemoryPage {
    offset: usize,
    data: MemData,
}

impl MemoryPage {
    pub fn new(size: AddrSize) -> Self {
        let f = |_| None;

        MemoryPage {
            offset: 0,
            data: match size {
                AddrSize::Byte => MemData::Small(Box::new(from_fn(f))),
                AddrSize::Word => MemData::Medium(Box::new(from_fn(f))),
                AddrSize::Long => MemData::Large(Box::new(from_fn(f))),
            },
        }
    }

    pub const fn size(&self) -> AddrSize {
        match self.data {
            MemData::Small(_) => AddrSize::Byte,
            MemData::Medium(_) => AddrSize::Word,
            MemData::Large(_) => AddrSize::Long,
        }
    }

    pub fn read_value(&self, addr: usize) -> Option<&Value> {
        todo!()
    }

    pub fn write_value(&mut self, value: Value) -> usize {
        todo!()
    }
}

#[derive(Debug)]
pub struct Compiler<'c> {
    mem: MemoryPage,
    values: Vec<Value>,
    code: Vec<Chunk<'c>>,
}

impl<'c> Compiler<'c> {
    fn new() -> Self {
        Compiler {
            mem: MemoryPage::new(AddrSize::Byte),
            values: vec![],
            code: vec![],
        }
    }

    fn store(&mut self, value: Value) -> usize {
        self.values.push(value);
        self.values.len() - 1
    }

    pub fn compile_expr(&mut self, expr: Expr<'_>) -> Result<MemoryPage> {
        let mut largest_addr = 0;
        let mut bytes = Bytes::new();
        let mut code = vec![];

        match expr {
            Expr::Value(val) => {
                let addr = self.store(val);
                largest_addr = std::cmp::max(addr, largest_addr);
                code.push((Instruction::Load, Some(Args::Constant(addr))));

                let addr_size = match largest_addr {
                    x if x < BYTE => AddrSize::Byte,
                    x if x < WORD => AddrSize::Word,
                    x if x < LONG => AddrSize::Long,
                    _ => panic!("out of memory"),
                };

                let mut mem = MemoryPage::new(addr_size);
                for val in std::mem::take(&mut self.values) {
                    mem.write_value(val);
                }

                // Memory allocated

                for (instr, args) in code.into_iter() {
                    match instr {
                        Instruction::Load => {
                            let Args::Constant(addr) = args.unwrap();
                        }

                        Instruction::Print => todo!(),
                        Instruction::Return => todo!(),
                    }
                }

                todo!()
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

        todo!()
    }

    // pub fn compile<const N: usize>(mut self, ast: Expr<'c>) -> Result<Program<N>> {
    //     let memory = self
    //         .memory
    //         .0
    //         .try_into()
    //         .unwrap_or_else(|v: Vec<MemoryPage>| panic!("Ack!"));

    //     let code = (self.code)
    //         .try_into()
    //         .unwrap_or_else(|v: Vec<Chunk<'c>>| panic!("Ack!"));

    //     Ok(Program { memory, code })
    // }
}

pub struct Program<'p, const N: usize> {
    memory: Memory<N>,
    code: [Chunk<'p>; N],
}
