use crate::{
    error::{RuntimeError, UmpteenError},
    repr::{
        ast::stmt::Stmt,
        bytecode::{chunk::Chunk, instruction::Instr},
        token::Token,
        value::Value,
    },
};

use super::{
    compiler::{Compiler, Program},
    env::{Memory, Stack},
    lexer::Lexer,
    parse::{Ast, Parser},
};

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

    pub fn run(&mut self, src: &str) -> Result<Value, UmpteenError> {
        let tokens = Self::scan(src);
        let ast = Self::parse(tokens)?;
        let program = Self::compile(ast, &mut self.mem)?;

        for chunk in program {
            #[cfg(debug_assertions)]
            dbg!(&chunk);

            self.exec(chunk)?;
        }

        Ok(Value::Empty)
    }

    fn scan(src: &str) -> Vec<Token> {
        let lexer = Lexer::new(src);
        lexer.scan()
    }

    fn parse(tokens: Vec<Token>) -> Result<Ast, UmpteenError> {
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;
        Ok(ast)
    }

    fn compile(ast: Ast, mem: &mut Memory) -> Result<Program, UmpteenError> {
        let mut compiler = Compiler::new(mem);
        let program = compiler.compile(ast)?;
        Ok(program)
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
