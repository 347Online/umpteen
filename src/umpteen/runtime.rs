use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::{
    ast::{Expr, Parser, Stmt},
    bytecode::{Chunk, Compiler, Instr, Program},
    error::{CompilerError, RuntimeError, UmpteenError},
    token::Lexer,
    value::{Object, Value},
};

pub type Stack = Vec<Value>;

#[derive(Debug, Default)]
pub struct Memory<'m> {
    values: Vec<Option<Value>>,
    names: HashMap<&'m str, usize>,
}

impl<'m> Memory<'m> {
    pub fn declare_constant(&mut self, value: Value) -> usize {
        let addr = self.offset();
        self.values.push(Some(value));
        addr
    }

    pub fn declare(&mut self, name: &'m str) -> Result<usize, RuntimeError> {
        if self.names.contains_key(name) {
            panic!("variable already declared") // TODO: Create an error variant instead of panic
        } else {
            let addr = self.offset();
            self.values.push(None);
            self.names.insert(name, addr);
            Ok(addr)
        }
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), UmpteenError> {
        let addr = self.retrieve(name)?;
        self.values[addr] = Some(value);

        Ok(())
    }

    pub fn get(&self, addr: usize) -> Result<Value, UmpteenError> {
        let value = self
            .values
            .get(addr)
            .cloned()
            .flatten()
            .expect(&format!("invalid reference {:#}", addr)); // TODO: Create an error variant instead of expect

        Ok(value)
    }

    fn offset(&self) -> usize {
        self.values.len()
    }

    fn retrieve(&self, name: &str) -> Result<usize, UmpteenError> {
        let addr = *self
            .names
            .get(name)
            .expect(&format!("unknown identifier {}", name));

        Ok(addr)
    }
}

impl<'m> Deref for Memory<'m> {
    type Target = Vec<Option<Value>>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<'m> DerefMut for Memory<'m> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}

#[derive(Default)]
pub struct Runtime<'r> {
    mem: Memory<'r>,
    stack: Stack,
    program: Program,
}

impl<'r> Runtime<'r> {
    pub fn new(mem: Memory<'r>) -> Self {
        Runtime {
            mem,
            stack: vec![],
            program: vec![],
        }
    }

    pub fn compile_source<'c>(&mut self, src: &'c str) -> Result<Program, UmpteenError> {
        let lexer = Lexer::new(src);
        let tokens = lexer.scan();

        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;
        let mut compiler = Compiler::new(&mut self.mem);

        let program = compiler.compile(ast)?;

        Ok(program)
    }

    pub fn run<'rr, 'x: 'rr>(&mut self, src: &str) -> Result<Value, UmpteenError> {
        // let program = std::mem::take(&mut self.program);
        let program = self.compile_source(src)?;

        for chunk in program {
            #[cfg(debug_assertions)]
            dbg!(&chunk);

            self.exec(chunk)?;
        }

        Ok(Value::Empty)
    }

    fn load_program(&'r mut self, mut prog: Program) {
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

        let return_value = loop {
            let instr = chunk.read_instr(offset)?;
            offset += 1;
            match instr {
                Instr::Constant => {
                    let addr = read_addr!();
                    let val = self.mem.get(addr)?;
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
