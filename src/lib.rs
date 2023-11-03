use exec::{lexer::Lexer, vm::Vm};
use repr::{error::Error, value::Value};

pub mod exec;
pub mod repr;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Default)]
pub struct Runtime {
    compiler: Compiler,
    vm: Vm,
}

impl Runtime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self, src: &str) -> Result<Value> {
        let tokens = Lexer::new(src).scan();
        // todo!
        let ast = Parser::new(tokens).parse()?;
        let bytecode = self.compiler.compile(ast);
        vm.exec(bytecode)
    }
}
