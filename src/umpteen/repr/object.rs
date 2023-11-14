use std::fmt::Display;

use super::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    List(Vec<Value>),
    SomethingElse
    // Fnc(Fnc),
}

impl Object {
    pub fn is_empty(&self) -> bool {
        match self {
            Object::List(values) => values.is_empty(),
            Object::SomethingElse => todo!(),
            // Object::Fnc(_) => false,
        }
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
            Object::SomethingElse => todo!(), // Object::Fnc(fnc) => write!(f, "{:#?}", fnc),
        }
    }
}
