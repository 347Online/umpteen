use crate::{
    bytecode::{Arg, Instruction},
    value::Value,
};

#[derive(Debug)]
pub struct Chunk {
    data: Vec<Value>,
    code: Vec<Instruction>,
    args: Vec<Arg>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            data: vec![],
            code: vec![],
            args: vec![],
        }
    }

    pub fn write_val(&mut self, constant: Value) -> u8 {
        let addr = self.data.len() as u8;
        self.data.push(constant);
        addr
    }

    pub fn write_inst(&mut self, inst: Instruction) {
        self.code.push(inst)
    }

    pub fn write_arg(&mut self, arg: Arg) {
        self.args.push(arg)
    }

    pub fn write_args(&mut self, args: &[Arg]) {
        for arg in args {
            self.write_arg(*arg);
        }
    }

    pub fn consume(self) -> (Vec<Value>, Vec<Instruction>, Vec<Arg>) {
        (self.data, self.code, self.args)
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}
