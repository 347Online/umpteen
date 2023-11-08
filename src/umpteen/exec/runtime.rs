use crate::{
    error::UmpteenError,
    repr::{bytecode::chunk::Chunk, token::Token, value::Value},
};

use super::{
    compiler::Compiler,
    env::{Memory, Stack},
    lexer::Lexer,
    parse::{Ast, Parser},
};

#[derive(Default)]
pub struct Runtime<'r> {
    mem: Memory<'r>,
    stack: Stack,
}

impl<'r> Runtime<'r> {
    pub fn new() -> Self {
        Runtime {
            mem: Memory::default(),
            stack: vec![],
        }
    }

    pub fn run<'m>(&'m mut self, src: &'r str) -> Result<Value, UmpteenError> {
        let tokens = Self::scan(src);
        let ast = Self::parse(tokens)?;
        let program = Self::compile(ast)?;

        for chunk in program {
            self.exec(chunk)?;
        }

        Ok(Value::Empty)
    }

    fn scan(src: &str) -> Vec<Token> {
        let lexer = Lexer::new(src);
        let tokens = lexer.scan();

        #[cfg(debug_assertions)]
        dbg!(&tokens);

        tokens
    }

    fn parse(tokens: Vec<Token>) -> Result<Ast, UmpteenError> {
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;

        #[cfg(debug_assertions)]
        dbg!(&ast);

        Ok(ast)
    }

    fn compile(ast: Ast<'r>) -> Result<Vec<Chunk>, UmpteenError> {
        let compiler = Compiler::new();
        let program = compiler.compile(ast)?;

        #[cfg(debug_assertions)]
        dbg!(&program);

        Ok(program)
    }

    fn exec(&mut self, chunk: Chunk) -> Result<Value, UmpteenError> {
        let mut offset = 0;

        // let return_value = loop {
        //     let instr = chunk.read_instr(offset)?;
        //     offset += 1;
        //     match instr {
        //         Instr::Constant => {
        //             let addr = read_addr!();
        //             let val = self.mem_get(addr)?;
        //             self.stack.push(StackItem::Value(val));
        //         }
        //         Instr::Print => {
        //             println!("{}", pop!());
        //         }
        //         Instr::Push => {
        //             let addr = read_addr!();
        //             self.stack.push(StackItem::Address(addr))
        //         }
        //         Instr::Pop => todo!(),
        //         Instr::Assign => todo!(),

        //         Instr::Return => todo!(),
        //         Instr::Exit => break Value::Empty,
        //     }
        // };

        Ok(Value::Empty)
    }

    // fn mem_get(&self, addr: usize) -> Result<Value, MemoryError> {
    //     let mem = self
    //         .mem
    //         .as_ref()
    //         .ok_or(MemoryError::OutOfBoundsMemoryAccess)?;
    //     mem.get(addr)
    // }
}
