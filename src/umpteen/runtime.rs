use crate::{bytecode::Chunk, value::Value, Result};

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

    pub fn exec(mut self, stack: &mut Stack) -> Result<Value> {
        for chunk in self.program {
            
        }

        todo!();
        Ok(value)
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
