use std::fmt::Display;

use crate::exec::interpreter::Interpreter;

use super::{ast::stmt::Stmt, value::Value};

pub trait Call {
    fn call(&mut self, interpreter: &mut Interpreter, args: Vec<Value>) -> Value;
    fn arity(&self) -> usize;
    fn name(&self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
pub enum NativeFnc {
    Time,
    Print,
    Str,
}
impl Call for NativeFnc {
    fn call(&mut self, interpreter: &mut Interpreter, args: Vec<Value>) -> Value {
        match self {
            NativeFnc::Time => return interpreter.start().elapsed().as_secs_f64().into(),
            NativeFnc::Print => println!("{}", args[0]),
            NativeFnc::Str => return args[0].to_string().into(),
        }

        Value::Empty
    }

    fn arity(&self) -> usize {
        match self {
            NativeFnc::Time => 0,
            NativeFnc::Print => 1,
            NativeFnc::Str => 1,
        }
    }

    fn name(&self) -> String {
        format!("{:?}", self).to_ascii_lowercase()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserFnc {
    name: String,
    arity: usize,
    params: Vec<(String, String)>,
    body: Vec<Stmt>,
}

impl UserFnc {
    pub fn new(name: String, params: Vec<(String, String)>, body: Vec<Stmt>) -> Self {
        UserFnc {
            name,
            arity: params.len(),
            params,
            body,
        }
    }
}

impl Call for UserFnc {
    fn call(&mut self, interpreter: &mut Interpreter, args: Vec<Value>) -> Value {
        let (mem_key, mem) = interpreter.new_context();
        for (i, (param, _)) in self.params.iter().enumerate() {
            mem.declare(param).unwrap();
            mem.assign(param, None, args[i].clone()).unwrap();
        }

        interpreter.exec_block(&self.body, Some(mem_key)).unwrap();

        Value::Empty
    }

    fn arity(&self) -> usize {
        self.arity
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Fnc {
    Native(NativeFnc),
    User(UserFnc),
}

impl Call for Fnc {
    fn call(&mut self, interpreter: &mut Interpreter, args: Vec<Value>) -> Value {
        match self {
            Fnc::Native(f) => f.call(interpreter, args),
            Fnc::User(f) => f.call(interpreter, args),
        }
    }

    fn arity(&self) -> usize {
        match self {
            Fnc::Native(n) => n.arity(),
            Fnc::User(u) => u.arity(),
        }
    }

    fn name(&self) -> String {
        match self {
            Fnc::Native(n) => n.name(),
            Fnc::User(u) => u.name(),
        }
    }
}

impl Display for Fnc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fnc::Native(native) => write!(f, "<native fnc {}()>", native.name()),
            Fnc::User(_) => write!(f, "<fnc todo()>"),
        }
    }
}
