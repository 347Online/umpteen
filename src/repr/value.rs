use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Not, Rem, Sub},
    process::{ExitCode, Termination},
};

use crate::Result;

use super::error::{Error, RuntimeError};

#[derive(Default, Debug, Clone, PartialEq)]
pub enum Value {
    #[default]
    Empty,
    Boolean(bool),
    Number(f64),
    String(Box<String>),
}

impl Value {
    pub fn truthy(&self) -> bool {
        match self {
            Value::Empty => false,
            Value::Boolean(x) => *x,
            Value::Number(x) => *x > 0.0,
            Value::String(x) => !x.is_empty(),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tmp;

        let repr = match self {
            Value::Empty => "<Empty>",
            Value::Boolean(x) => {
                tmp = x.to_string();
                &tmp
            }
            Value::Number(x) => {
                tmp = x.to_string();
                &tmp
            }
            Value::String(x) => x,
        };

        write!(f, "{repr}")
    }
}

impl From<Option<Value>> for Value {
    fn from(value: Option<Value>) -> Self {
        value.unwrap_or(Value::Empty)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}

impl From<Value> for bool {
    fn from(value: Value) -> Self {
        match value {
            Value::Empty => false,
            Value::Boolean(x) => x,
            Value::Number(x) => x > 0.0,
            Value::String(x) => !x.is_empty(),
        }
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Number(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(Box::new(value))
    }
}

impl Not for Value {
    type Output = Value;

    fn not(self) -> Self::Output {
        Value::Boolean(!self.truthy())
    }
}

impl Neg for Value {
    type Output = Result<Value>;

    fn neg(self) -> Self::Output {
        match self {
            Value::Number(x) => Ok(Value::Number(-x)),
            _ => Err(Error::Runtime(RuntimeError::Illegal)),
        }
    }
}

impl Add for Value {
    type Output = Result<Value>;

    fn add(self, rhs: Self) -> Self::Output {
        let val = match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            (Value::String(mut a), Value::String(ref b)) => {
                a.push_str(b);
                Value::from(*a)
            }
            _ => Err(Error::Runtime(RuntimeError::Illegal))?,
        };
        Ok(val)
    }
}

impl Sub for Value {
    type Output = Result<Value>;

    fn sub(self, rhs: Self) -> Self::Output {
        let val = match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            _ => Err(Error::Runtime(RuntimeError::Illegal))?,
        };
        Ok(val)
    }
}

impl Mul for Value {
    type Output = Result<Value>;

    fn mul(self, rhs: Self) -> Self::Output {
        let val = match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            _ => Err(Error::Runtime(RuntimeError::Illegal))?,
        };
        Ok(val)
    }
}

impl Div for Value {
    type Output = Result<Value>;

    fn div(self, rhs: Self) -> Self::Output {
        let val = match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
            _ => Err(Error::Runtime(RuntimeError::Illegal))?,
        };
        Ok(val)
    }
}

impl Rem for Value {
    type Output = Result<Value>;

    fn rem(self, rhs: Self) -> Self::Output {
        let val = match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a % b),
            _ => Err(Error::Runtime(RuntimeError::Illegal))?,
        };
        Ok(val)
    }
}

impl Termination for Value {
    fn report(self) -> ExitCode {
        ExitCode::SUCCESS
    }
}
