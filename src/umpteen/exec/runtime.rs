use crate::{
    error::{MemoryError, UmpteenError},
    repr::{
        bytecode::{chunk::Chunk, instruction::Instr},
        token::Token,
        value::Value,
    }, exec::env::StackItem,
};

use super::{
    compiler::Compiler,
    env::{Memory, Stack},
    lexer::Lexer,
    parse::{Ast, Parser},
};

#[derive(Default)]
pub struct Runtime {
    mem: Memory,
    stack: Stack,
}

impl Runtime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self, src: &str) -> Result<Value, UmpteenError> {
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

    fn compile(ast: Ast<'_>) -> Result<Vec<Chunk>, UmpteenError> {
        let compiler = Compiler::new();
        let program = compiler.compile(ast)?;

        #[cfg(debug_assertions)]
        dbg!(&program);

        Ok(program)
    }

    fn exec(&mut self, chunk: Chunk) -> Result<Value, UmpteenError> {
        let mut offset = 0;

        macro_rules! read_byte {
            () => {{
                let byte = chunk.read_byte(offset)?;
                offset += 1;
                *byte
            }};
        }

        // macro_rules! read_bytes {
        //     ($count:expr) => {{
        //         let bytes = chunk.read_bytes(offset, $count)?;
        //         offset += $count;
        //         bytes
        //     }};
        // }

        macro_rules! read_name {
            () => {{
                let len = read_byte!() as usize;
                let utf8_bytes = chunk.read_bytes(offset, len)?;
                let name = std::str::from_utf8(utf8_bytes).expect("Name was not valid utf8"); // TODO: Create an error variant for this
                name
            }};
        }

        macro_rules! pop {
            () => {
                self.stack.pop().expect("Popped when stack was empty"); // TODO: Create an error variant for this
            };
        }

        macro_rules! push {
            ($e:expr) => {
                self.stack.push($e)
            };
        }

        let return_value = loop {
            let byte = read_byte!();
            let instr = Instr::try_from(byte)?;

            match instr {
                Instr::Address => {
                    let name = read_name!();
                    let addr = self.mem.retrieve(name)?;
                    push!(StackItem::Name(name.to_string()));
                },
                Instr::Pop => {}
                Instr::Let => {
                    let name = read_name!();
                    self.mem.declare(name)?;
                }
                Instr::Set => {
                    let name = read_name!();
                    let StackItem::Value(value) = pop!() else {
                        panic!("Failed to pop a value")
                    };
                    // let StackItem::Address(addr) = pop!() else {
                    //     panic!("Failed to pop an address")
                    // };
                    let StackItem::Name(name) = pop!() else {
                        panic!("Failed to pop a name")
                    };
                    self.mem.assign(&name, value)?;
                }
                Instr::Get => {
                    let name = read_name!();
                    let addr = self.mem.retrieve(name)?;
                    let value = self.mem.get(addr)?;
                    push!(StackItem::Value(value));
                },
                Instr::Push => todo!(),
                Instr::Print => todo!(),
                Instr::Exit => break Value::Empty,
            }

            // todo!();

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
        };

        Ok(return_value)
    }

    // fn mem_get(&self, addr: usize) -> Result<Value, MemoryError> {
    //     let mem = self
    //         .mem
    //         .as_ref()
    //         .ok_or(MemoryError::OutOfBoundsMemoryAccess)?;
    //     mem.get(addr)
    // }
}
