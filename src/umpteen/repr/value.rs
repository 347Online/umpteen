use std::{
    cell::RefCell,
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Not, Rem, Sub},
    process::{ExitCode, Termination},
};

use crate::{boxed, error::ParseError, umpteen::util::unescape};

use super::{
    ast::ops::{Binary, Unary},
    object::Object,
};

#[derive(Default, Debug, Clone, PartialEq)]
pub enum Value {
    #[default]
    Empty,
    Boolean(bool),
    Number(f64),
    String(Box<String>),
    Object(RefCell<Object>),
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

    pub fn truthy(&self) -> bool {
        match self {
            Value::Empty => false,
            Value::Boolean(x) => *x,
            Value::Number(x) => *x > 0.0,
            Value::String(string) => !string.is_empty(),

            Value::Object(x) => !x.borrow().is_empty(),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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

            Value::Object(x) => {
                // SAFETY
                // TODO: Determine enforcement of the invariants or find a safe alternative
                let repr = unsafe { x.as_ptr().as_ref().unwrap() };
                write!(f, "{}", repr)
            }
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
        Value::String(boxed!(unescape(value)))
    }
}

impl From<&String> for Value {
    fn from(value: &String) -> Self {
        Value::from(value as &str)
    }
}

impl From<char> for Value {
    fn from(value: char) -> Self {
        let mut tmp = [0_u8; 1];
        Value::from(&*value.encode_utf8(&mut tmp))
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Number(value)
    }
}

impl Not for Value {
    type Output = Self;

    fn not(self) -> Self::Output {
        Value::Boolean(!self.truthy())
    }
}

impl Neg for Value {
    type Output = Result<Self, ParseError>;

    fn neg(self) -> Self::Output {
        match self {
            Value::Number(x) => Ok(Value::Number(-x)),
            _ => Err(ParseError::IllegalUnaryOperation(
                self.to_string(),
                Unary::Negate,
            ))?,
        }
    }
}

impl Add for Value {
    type Output = Result<Self, ParseError>;

    fn add(self, rhs: Self) -> Self::Output {
        let lhs = self;
        let val = match (lhs.clone(), rhs.clone()) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            (Value::String(a), Value::String(b)) => Value::String(boxed!(*a + &b)),

            (a, b) => Err(ParseError::IllegalBinaryOperation(
                a.to_string(),
                b.to_string(),
                Binary::Add,
            ))?,
        };
        Ok(val)
    }
}

impl Sub for Value {
    type Output = Result<Self, ParseError>;

    fn sub(self, rhs: Self) -> Self::Output {
        let val = match (&self, &rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            _ => Err(ParseError::IllegalBinaryOperation(
                self.to_string(),
                rhs.to_string(),
                Binary::Subtract,
            ))?,
        };
        Ok(val)
    }
}

impl Mul for Value {
    type Output = Result<Self, ParseError>;

    fn mul(self, rhs: Self) -> Self::Output {
        let val = match (&self, &rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            _ => Err(ParseError::IllegalBinaryOperation(
                self.to_string(),
                rhs.to_string(),
                Binary::Multiply,
            ))?,
        };
        Ok(val)
    }
}

impl Div for Value {
    type Output = Result<Self, ParseError>;

    fn div(self, rhs: Self) -> Self::Output {
        let val = match (&self, &rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
            _ => Err(ParseError::IllegalBinaryOperation(
                self.to_string(),
                rhs.to_string(),
                Binary::Divide,
            ))?,
        };
        Ok(val)
    }
}

impl Rem for Value {
    type Output = Result<Self, ParseError>;

    fn rem(self, rhs: Self) -> Self::Output {
        let val = match (&self, &rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a % b),
            _ => Err(ParseError::IllegalBinaryOperation(
                self.to_string(),
                rhs.to_string(),
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
