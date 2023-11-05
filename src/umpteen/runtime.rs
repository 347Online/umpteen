use std::ops::{Deref, DerefMut};

use crate::{
    ast::{Expr, Stmt},
    bytecode::{Chunk, Compiler, Instr, Program},
    value::{Object, Value},
    Error, Result, RuntimeError,
};

pub type Stack = Vec<Value>;

#[derive(Debug, Default)]
pub struct Memory(Vec<Value>);

impl Memory {
    pub fn get(&self, addr: usize) -> Option<Value> {
        self.0.get(addr).cloned()
    }

    pub fn store(&mut self, value: Value) -> usize {
        let addr = self.offset();
        self.0.push(value);
        addr
    }

    fn offset(&self) -> usize {
        self.0.len()
    }
}

impl Deref for Memory {
    type Target = Vec<Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Memory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default)]
pub struct Runtime {
    mem: Memory,
    stack: Stack,
    program: Program,
}

impl Runtime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn sample_program(&mut self) -> Result<()> {
        let string = String::from("Hello world");
        let boxed_str = Box::new(string);
        let obj = Object::String(boxed_str);
        let val = Value::Object(obj);
        let expr = Expr::Value(val);
        let stmt = Stmt::Print(expr);

        let cp = Compiler::new(&mut self.mem);
        let program = cp.compile(vec![stmt, Stmt::Return(None)])?;
        self.load_program(program);
        self.run()?;

        Ok(())
    }

    pub fn load_program(&mut self, mut prog: Program) {
        self.program.append(&mut prog)
    }

    pub fn run(&mut self) -> Result<Value> {
        let program = std::mem::take(&mut self.program);
        for chunk in program {
            self.exec(dbg!(chunk))?;
        }

        Ok(Value::Empty)
    }

    fn exec(&mut self, chunk: Chunk) -> Result<Value> {
        let mut offset = 0;

        macro_rules! read_addr {
            () => {{
                let (addr, off) = chunk.read_addr(offset)?.read();
                offset += off;
                addr
            }};
        }

        macro_rules! pop {
            (?) => {
                self.stack.pop()
            };

            (e) => {
                pop!(?).unwrap_or(Value::Empty)
            };

            () => {
                pop!(?).ok_or(RuntimeError::StackMissingValue)?
            };
        }

        macro_rules! mem_get {
            ($addr:expr) => {
                self.mem
                    .get($addr)
                    .ok_or(RuntimeError::OutOfBoundsMemoryAccess)?
            };
        }

        let return_value = loop {
            let instr = chunk.read_instr(offset)?;
            offset += 1;
            match instr {
                Instr::Constant => {
                    let addr = read_addr!();
                    let val = mem_get!(addr);
                    self.stack.push(val);
                }
                Instr::Print => {
                    println!("{}", pop!());
                }
                Instr::Return => {
                    break (if self.stack.is_empty() {
                        Value::Empty
                    } else {
                        pop!()
                    });
                }
            }
        };

        Ok(return_value)
    }
}
