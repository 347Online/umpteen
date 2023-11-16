// TODO: Migrate Fnc-related definitions to fnc.rs to improve readability

use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use super::{value::Value, fnc::{Fnc, UserFnc, NativeFnc}};

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
