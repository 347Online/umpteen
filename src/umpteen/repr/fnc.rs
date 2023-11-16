use std::fmt::Display;

use crate::{error::UmpteenError, exec::interpreter::Interpreter};

use super::{ast::stmt::Stmt, value::Value};

pub trait Call {
    fn call(&mut self, vm: &mut Interpreter, args: Vec<Value>) -> Result<Value, UmpteenError>;
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
    fn call(&mut self, vm: &mut Interpreter, args: Vec<Value>) -> Result<Value, UmpteenError> {
        let return_value = match self {
            NativeFnc::Time => {
                let now = vm.start().elapsed().as_secs_f64();
                Value::from(now)
            }
            NativeFnc::Print => {
                let value = &args[0];
                println!("{}", value);
                Value::Empty
            }
            NativeFnc::Str => {
                let value = &args[0];
                Value::from(value.to_string())
            }
        };

        Ok(return_value)
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
    fn call(&mut self, vm: &mut Interpreter, args: Vec<Value>) -> Result<Value, UmpteenError> {
        let (mem_key, mem) = vm.new_context();
        for (i, (param, _)) in self.params.iter().enumerate() {
            mem.declare(param).unwrap();
            mem.assign(param, None, args[i].clone())?;
        }

        vm.exec_block(&self.body, Some(mem_key))?;

        Ok(Value::Empty)
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
    fn call(&mut self, vm: &mut Interpreter, args: Vec<Value>) -> Result<Value, UmpteenError> {
        match self {
            Fnc::Native(f) => f.call(vm, args),
            Fnc::User(f) => f.call(vm, args),
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
