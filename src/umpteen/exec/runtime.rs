use crate::{
    ast::Parser,
    bytecode::{Chunk, Compiler, Instr, Program},
    error::{RuntimeError, UmpteenError},
    token::Lexer,
    value::Value,
};

use super::memory::Environment;

#[derive(Default)]
pub struct Runtime<'r> {
    mem: Environment<'r>,
    stack: Stack,
    program: Program,
}

impl<'r> Runtime<'r> {
    pub fn new(mem: Environment<'r>) -> Self {
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
