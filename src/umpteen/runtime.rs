use crate::{
    bytecode::{Chunk, Instruction},
    value::Value,
    Result,
};

pub type Stack = Vec<Value>;
pub type Program = Vec<Chunk>;

pub struct Runtime {
    stack: Stack,
    program: Program,
}

impl Runtime {
    pub fn new(program: Program) -> Self {
        Runtime {
            stack: vec![],
            program,
        }
    }

    pub fn exec(&mut self, chunk: Chunk) -> Result<Value> {
        let mut offset = 0;
        loop {
            let instr = chunk.read_instr(offset)?;
            match instr {
                Instruction::Constant => {
                    let addr = chunk.read_addr(offset);
                }
                Instruction::Print => todo!(),
                Instruction::Return => todo!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::Expr,
        bytecode::Compiler,
        value::{Object, Value},
    };

    #[test]
    fn some_fn() {
        let string = String::from("Hello world");
        let boxed_str = Box::new(string);
        let obj = Object::String(boxed_str);
        let val = Value::Object(obj);
        let ast = Expr::Value(val);

        let mut cp = Compiler::new();
        let chunk = cp.compile_expr(ast).unwrap();
        dbg!(chunk);
    }
}
