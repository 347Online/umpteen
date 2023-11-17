use std::fmt::Display;

use crate::{error::UmpteenError, exec::interpreter::Interpreter};

use super::{ast::stmt::Stmt, object::Object, value::Value};

pub trait Call {
    fn call(&mut self, vm: &mut Interpreter, args: &[Value]) -> Result<Value, UmpteenError>;
    fn arity(&self) -> usize;
    fn name(&self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
pub enum NativeFnc {
    Time,   // Returns a numeric representation of the current time
    Print,  // Print a line to stdout
    Printx, // Similar to print without a trailing newline and limited support for escape sequences
    Str,    // Returns a string representation of an Umpteen Value
    Len,    // Returns the "length" of a Value (List: Entries, String: Bytes, Empty: 0, Other: 1)
    Chr,    // Converts a number from 0 - 255 to its ASCII representation
    Ord,    // Converts one ASCII character to its numeric representation
}

impl Call for NativeFnc {
    fn call(&mut self, vm: &mut Interpreter, args: &[Value]) -> Result<Value, UmpteenError> {
        let return_value = match self {
            NativeFnc::Time => {
                let now = vm.start().elapsed().as_secs_f64();
                Value::from(now)
            }
            NativeFnc::Print => {
                println!("{}", args[0]);
                Value::Empty
            }
            NativeFnc::Printx => {
                print!("{}", Value::from(&args[0].to_string()));
                Value::Empty
            }
            NativeFnc::Str => {
                let string = &args[0].to_string();
                Value::from(string)
            }
            NativeFnc::Len => match &args[0] {
                Value::Empty => Value::from(0.0),
                Value::Boolean(_) => Value::from(1.0),
                Value::Number(_) => Value::from(1.0),
                Value::String(s) => Value::from(s.len() as f64),
                Value::Object(ref obj) => match *obj.borrow() {
                    Object::List(ref list) => Value::from(list.len() as f64),
                    Object::Fnc(_) => Value::from(1.0),
                },
            },
            NativeFnc::Chr => match &args[0] {
                Value::Number(x) if (0.0..=255.0).contains(x) => {
                    Value::from(x.trunc() as u8 as char)
                }
                _ => {
                    eprintln!("Value must be a number from 0 - 255");
                    Value::Empty
                }
            },
            NativeFnc::Ord => match &args[0] {
                Value::String(c) => Value::from(c.as_bytes()[0] as f64),

                _ => {
                    eprintln!("Value must be a number from 0 - 255");
                    Value::Empty
                }
            },
        };

        Ok(return_value)
    }

    fn arity(&self) -> usize {
        match self {
            NativeFnc::Time => 0,

            _ => 1,
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
    fn call(&mut self, vm: &mut Interpreter, args: &[Value]) -> Result<Value, UmpteenError> {
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
    fn call(&mut self, vm: &mut Interpreter, args: &[Value]) -> Result<Value, UmpteenError> {
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
            Fnc::Native(nf) => write!(f, "<native fnc {}()>", nf.name()),
            Fnc::User(uf) => write!(f, "<fnc {}()>", uf.name()),
        }
    }
}
