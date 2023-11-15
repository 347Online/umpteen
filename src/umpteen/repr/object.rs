use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::exec::interpreter::Interpreter;

use super::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    List(Vec<Value>),
    Fnc(Fnc),
}

impl Object {
    pub fn is_empty(&self) -> bool {
        match self {
            Object::List(values) => values.is_empty(),
            Object::Fnc(_) => false,
        }
    }

    pub fn list(values: Vec<Value>) -> Rc<RefCell<Self>> {
        Self::create(Object::List(values))
    }

    pub fn fnc(f: Fnc) -> Rc<RefCell<Self>> {
        Self::create(Object::Fnc(f))
    }

    fn create(obj: Object) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(obj))
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Object::List(values) => {
                let mut buffer = String::from('[');
                let mut first = true;

                for value in values {
                    if first {
                        first = false;
                    } else {
                        buffer.push_str(", ");
                    }
                    buffer.push_str(&format!("{}", value));
                }
                buffer.push(']');
                write!(f, "{}", buffer)
            }
            Object::Fnc(fnc) => write!(f, "{}", fnc),
        }
    }
}

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

impl From<UserFnc> for Value {
    fn from(value: UserFnc) -> Self {
        Value::Object(Object::fnc(Fnc::User(value)))
    }
}
impl From<NativeFnc> for Value {
    fn from(value: NativeFnc) -> Self {
        Value::Object(Object::fnc(Fnc::Native(value)))
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Value::Object(Object::list(value))
    }
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
pub struct UserFnc {}

impl Call for UserFnc {
    fn call(&mut self, interpreter: &mut Interpreter, args: Vec<Value>) -> Value {
        todo!()
    }

    fn arity(&self) -> usize {
        todo!()
    }

    fn name(&self) -> String {
        todo!()
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
