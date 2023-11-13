use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Not, Rem, Sub},
    process::{ExitCode, Termination},
};

use crate::error::ParseError;

use super::ast::ops::{Binary, Unary};

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    List(Vec<Value>),
}

impl Object {
    pub fn is_empty(&self) -> bool {
        match self {
            Object::List(values) => values.is_empty(),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"[")?;
        match self {
            Object::List(values) => {
                let mut first = true;

                for value in values {
                    if first {first = false;} else {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", value)?;
                }
            },
        }
        write!(f, "]")
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum Value {
    #[default]
    Empty, // 000
    Boolean(bool),       // 001
    Number(f64),         // 002
    String(Box<String>), // 020
    Object(Box<Object>), // 030
}

impl Value {
    pub fn designation(&self) -> u8 {
        match self {
            Value::Empty => 0,
            Value::Boolean(_) => 1,
            Value::Number(_) => 2,
            Value::String(_) => 20,
            Value::Object(_) => 30,
        }
    }
}

impl Value {
    pub fn truthy(&self) -> bool {
        match self {
            Value::Empty => false,
            Value::Boolean(x) => *x,
            Value::Number(x) => *x > 0.0,
            Value::String(string) => !string.is_empty(),

            Value::Object(x) => !x.is_empty(),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        macro_rules! write_val {
            ($x: expr) => {
                write!(f, "{}", $x)
            };
        }

        match self {
            Value::Empty => write_val!("<Empty>"),
            Value::Boolean(x) => write_val!(x),
            Value::Number(x) => write_val!(x),
            Value::String(string) => write_val!(string),

            Value::Object(x) => write_val!(x),
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(Box::new(value.to_string()))
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(Box::new(value))
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Number(value)
    }
}

impl Not for Value {
    type Output = Value;

    fn not(self) -> Self::Output {
        Value::Boolean(!self.truthy())
    }
}

impl Neg for Value {
    type Output = Result<Value, ParseError>;

    fn neg(self) -> Self::Output {
        match self {
            Value::Number(x) => Ok(Value::Number(-x)),
            _ => Err(ParseError::IllegalUnaryOperation(self, Unary::Negate))?,
        }
    }
}

impl Add for Value {
    type Output = Result<Value, ParseError>;

    fn add(self, rhs: Self) -> Self::Output {
        let lhs = self;
        let val = match (lhs.clone(), rhs.clone()) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),

            (a, b) => Err(ParseError::IllegalBinaryOperation(a, b, Binary::Add))?,
        };
        Ok(val)
    }
}

impl Sub for Value {
    type Output = Result<Value, ParseError>;

    fn sub(self, rhs: Self) -> Self::Output {
        let val = match (&self, &rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            _ => Err(ParseError::IllegalBinaryOperation(
                self,
                rhs,
                Binary::Subtract,
            ))?,
        };
        Ok(val)
    }
}

impl Mul for Value {
    type Output = Result<Value, ParseError>;

    fn mul(self, rhs: Self) -> Self::Output {
        let val = match (&self, &rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            _ => Err(ParseError::IllegalBinaryOperation(
                self,
                rhs,
                Binary::Multiply,
            ))?,
        };
        Ok(val)
    }
}

impl Div for Value {
    type Output = Result<Value, ParseError>;

    fn div(self, rhs: Self) -> Self::Output {
        let val = match (&self, &rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
            _ => Err(ParseError::IllegalBinaryOperation(
                self,
                rhs,
                Binary::Divide,
            ))?,
        };
        Ok(val)
    }
}

impl Rem for Value {
    type Output = Result<Value, ParseError>;

    fn rem(self, rhs: Self) -> Self::Output {
        let val = match (&self, &rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a % b),
            _ => Err(ParseError::IllegalBinaryOperation(
                self,
                rhs,
                Binary::Modulo,
            ))?,
        };
        Ok(val)
    }
}

impl Termination for Value {
    fn report(self) -> ExitCode {
        ExitCode::SUCCESS
    }
}
