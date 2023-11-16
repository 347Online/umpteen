// TODO: Migrate Fnc-related definitions to fnc.rs to improve readability

use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
    rc::Rc,
};

use super::{
    fnc::{Fnc, NativeFnc, UserFnc},
    value::Value,
};

#[derive(Debug, Clone, PartialEq)]
pub struct List(Vec<Value>);

impl List {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Deref for List {
    type Target = Vec<Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for List {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    List(List),
    Fnc(Fnc),
}

impl Object {
    pub fn is_empty(&self) -> bool {
        match self {
            Object::List(values) => values.is_empty(),
            Object::Fnc(_) => false,
        }
    }

    pub fn list(values: Vec<Value>) -> RefCell<Self> {
        Self::create(Object::List(List(values)))
    }

    fn fnc(f: Fnc) -> RefCell<Self> {
        Self::create(Object::Fnc(f))
    }

    fn create(obj: Object) -> RefCell<Self> {
        RefCell::new(obj)
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Object::List(values) => {
                let mut buffer = String::from('[');
                let mut first = true;

                for value in values.iter() {
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

impl From<Vec<Value>> for List {
    fn from(values: Vec<Value>) -> Self {
        List(values)
    }
}
