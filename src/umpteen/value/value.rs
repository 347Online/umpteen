use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Not, Rem, Sub},
    process::{ExitCode, Termination},
};

use crate::{
    ast::{Binary, Unary},
    error::SyntaxError,
};

use super::{Object, ObjectData};

#[derive(Default, Debug, Clone, PartialEq)]
pub enum Value {
    #[default]
    Empty,
    Boolean(bool),
    Number(f64),
    Object(Object),
}

impl Value {
    pub fn truthy(&self) -> bool {
        match self {
            Value::Empty => false,
            Value::Boolean(x) => *x,
            Value::Number(x) => *x > 0.0,
            Value::Object(obj) => !obj.is_empty(),
        }
    }

    pub fn string(string: String) -> Self {
        Value::Object(Object(Box::new(ObjectData::String(string))))
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
            Value::Object(x) => {
                tmp = x.to_string();
                &tmp
            }
        };

        write!(f, "{repr}")
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::string(value.to_string())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::string(value.to_string())
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
    type Output = Result<Value, SyntaxError>;

    fn neg(self) -> Self::Output {
        match self {
            Value::Number(x) => Ok(Value::Number(-x)),
            _ => Err(SyntaxError::IllegalUnaryOperation(self, Unary::Negate))?,
        }
    }
}

impl Add for Value {
    type Output = Result<Value, SyntaxError>;

    fn add(self, rhs: Self) -> Self::Output {
        let lhs = self;
        let val = match (lhs.clone(), rhs.clone()) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),

            (a, b) => Err(SyntaxError::IllegalBinaryOperation(a, b, Binary::Add))?,
        };
        Ok(val)
    }
}

impl Sub for Value {
    type Output = Result<Value, SyntaxError>;

    fn sub(self, rhs: Self) -> Self::Output {
        let val = match (&self, &rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            _ => Err(SyntaxError::IllegalBinaryOperation(
                self,
                rhs,
                Binary::Subtract,
            ))?,
        };
        Ok(val)
    }
}

impl Mul for Value {
    type Output = Result<Value, SyntaxError>;

    fn mul(self, rhs: Self) -> Self::Output {
        let val = match (&self, &rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            _ => Err(SyntaxError::IllegalBinaryOperation(
                self,
                rhs,
                Binary::Multiply,
            ))?,
        };
        Ok(val)
    }
}

impl Div for Value {
    type Output = Result<Value, SyntaxError>;

    fn div(self, rhs: Self) -> Self::Output {
        let val = match (&self, &rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
            _ => Err(SyntaxError::IllegalBinaryOperation(
                self,
                rhs,
                Binary::Divide,
            ))?,
        };
        Ok(val)
    }
}

impl Rem for Value {
    type Output = Result<Value, SyntaxError>;

    fn rem(self, rhs: Self) -> Self::Output {
        let val = match (&self, &rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a % b),
            _ => Err(SyntaxError::IllegalBinaryOperation(
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
