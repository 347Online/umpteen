use std::ops::{Deref, DerefMut};

use crate::{
    ast::{Expr, Parser, Stmt},
    bytecode::{Chunk, Compiler, Instr, Program},
    error::{RuntimeError, UmpteenError},
    token::Lexer,
    value::{Object, Value},
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
    pub fn new(mem: Memory) -> Self {
        Runtime {
            mem,
            stack: vec![],
            program: vec![],
        }
    }

    pub fn load_source(&mut self, src: &str) -> Result<(), UmpteenError> {
        let lexer = Lexer::new(src);
        let tokens = lexer.scan();

        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;

        let mut compiler = Compiler::new(&mut self.mem);
        let program = compiler.compile(ast)?;

        self.load_program(program);

        Ok(())
    }

    pub fn run(&mut self) -> Result<Value, UmpteenError> {
        let program = std::mem::take(&mut self.program);
        for chunk in program {
            #[cfg(debug_assertions)]
            dbg!(&chunk);

            self.exec(chunk)?;
        }

        Ok(Value::Empty)
    }

    fn load_program(&mut self, mut prog: Program) {
        self.program.append(&mut prog)
    }

    fn exec(&mut self, chunk: Chunk) -> Result<Value, UmpteenError> {
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
