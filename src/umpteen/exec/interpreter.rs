use crate::{
    error::UmpteenError,
    repr::{
        ast::{
            expr::Expr,
            ops::{Binary, Unary},
            stmt::Stmt,
        },
        value::Value,
    },
};

use super::{env::Memory, parse::Ast};

pub enum Eval {
    Value(Value),
    Variable(String),
}

#[derive(Debug, Default)]
pub struct Interpreter {
    mem: Memory,
}

impl Interpreter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn interpret(&mut self, ast: Ast) -> Result<Value, UmpteenError> {
        let mut return_value = Value::Empty;

        for stmt in ast {
            match stmt {
                Stmt::Declare(name, maybe_expr) => {
                    self.mem.declare(name)?;

                    if let Some(expr) = maybe_expr {
                        let value = self.eval(expr)?;
                        self.mem.assign(name, value)?;
                    }
                }
                Stmt::Expr(expr) => {
                    self.eval(expr)?;
                }
                Stmt::Print(expr) => {
                    let value = self.eval(expr)?;
                    println!("{}", value);
                }
                Stmt::Return(expr) => {
                    return_value = self.eval(expr)?;
                    break;
                }
                Stmt::Exit => break,
            }
        }

        Ok(return_value)
    }

    fn eval(&mut self, expr: Expr) -> Result<Value, UmpteenError> {
        let result = match expr {
            Expr::Value(value) => value,
            Expr::UnOp { expr, op } => {
                let value = self.eval(*expr)?;
                match op {
                    Unary::Not => !value,
                    Unary::Negate => (-value)?,
                }
            }
            Expr::BinOp { left, right, op } => {
                let lhs = self.eval(*left)?;

                match op {
                    Binary::Add => (lhs + self.eval(*right)?)?,
                    Binary::Subtract => (lhs - self.eval(*right)?)?,
                    Binary::Multiply => (lhs * self.eval(*right)?)?,
                    Binary::Divide => (lhs / self.eval(*right)?)?,
                    Binary::Modulo => (lhs / self.eval(*right)?)?,
                    Binary::And => {
                        if lhs.truthy() {
                            self.eval(*right)?
                        } else {
                            lhs
                        }
                    }
                    Binary::Or => {
                        if lhs.truthy() {
                            lhs
                        } else {
                            self.eval(*right)?
                        }
                    }
                }
            }
            Expr::Variable { name } => self.mem.get(name)?,
            Expr::Assign { target, expr } => {
                let Expr::Variable { name } = *target else {
                    panic!("Invalid Assignment Target");
                };

                let value = self.eval(*expr)?;
                self.mem.assign(name, value)?;
                Value::Empty
            }
        };

        Ok(result)
    }
}
