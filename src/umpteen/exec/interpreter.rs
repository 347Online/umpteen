use crate::{
    error::{RuntimeError, UmpteenError, ParseError},
    repr::{
        ast::{
            expr::Expr,
            ops::{Binary, Unary},
            stmt::Stmt,
        },
        token::Token,
        value::Value,
    },
};

use super::{
    env::Memory,
    lexer::Lexer,
    parse::{Ast, Parser},
};

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

    pub fn run(&mut self, src: &str) -> Result<Value, UmpteenError> {
        let tokens = Self::scan(src);
        let ast = Self::parse(tokens)?;
        self.interpret(ast)
    }

    fn scan(src: &str) -> Vec<Token> {
        let lexer = Lexer::new(src);
        lexer.scan()
    }

    fn parse(tokens: Vec<Token>) -> Result<Ast, UmpteenError> {
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        Ok(ast)
    }

    fn interpret(&mut self, ast: Ast) -> Result<Value, UmpteenError> {
        let mut return_value = Value::Empty;

        for stmt in ast {
            match self.exec(stmt)? {
                Some(val) => {
                    return_value = val;
                }
                None => break,
            }
        }

        Ok(return_value)
    }

    fn exec(&mut self, stmt: Stmt) -> Result<Option<Value>, UmpteenError> {
        match stmt {
            Stmt::Declare { name, init } => {
                self.mem.declare(name)?;

                if let Some(expr) = init {
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
                return Ok(Some(self.eval(expr)?));
            }
            Stmt::Empty => (),
            Stmt::Exit => return Ok(None),
            Stmt::Block(statements) => {
                for stmt in statements {
                    self.exec(stmt)?;
                }
            }
            Stmt::Condition {
                test,
                then_branch,
                else_branch,
            } => {
                if self.eval(test)?.truthy() {
                    self.exec(*then_branch)?;
                } else if let Some(else_branch) = else_branch {
                    self.exec(*else_branch)?;
                }
            }
            Stmt::Break => Err(RuntimeError::Break)?,
            Stmt::Continue => Err(RuntimeError::Continue)?,
            Stmt::Loop(block) => loop {
                match self.exec(*block.clone()) {
                    Err(UmpteenError::RuntimeError(RuntimeError::Break)) => break,
                    Err(UmpteenError::RuntimeError(RuntimeError::Continue)) => continue,
                    x => x,
                }?;
            },
        }

        Ok(Some(Value::Empty))
    }

    fn eval(&mut self, expr: Expr) -> Result<Value, UmpteenError> {
        let result = match expr {
            Expr::Literal(value) => value,
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
                    Binary::Equality => Value::Boolean(lhs == self.eval(*right)?),
                    Binary::Inequality => Value::Boolean(lhs != self.eval(*right)?),
                    Binary::GreaterThan => {
                        let rhs = self.eval(*right)?;
                        match (&lhs, &rhs) {
                            (Value::Number(a), Value::Number(b)) => {
                                Value::Boolean(a > b)
                            }

                            _ => Err(ParseError::IllegalBinaryOperation(lhs, rhs, op))?
                        }
                    },
                    Binary::GreaterOrEqual => {
                        let rhs = self.eval(*right)?;
                        match (&lhs, &rhs) {
                            (Value::Number(a), Value::Number(b)) => {
                                Value::Boolean(a >= b)
                            }

                            _ => Err(ParseError::IllegalBinaryOperation(lhs, rhs, op))?
                        }
                    },
                    Binary::LessThan => {
                        let rhs = self.eval(*right)?;
                        match (&lhs, &rhs) {
                            (Value::Number(a), Value::Number(b)) => {
                                Value::Boolean(a < b)
                            }

                            _ => Err(ParseError::IllegalBinaryOperation(lhs, rhs, op))?
                        }
                    },
                    Binary::LessOrEqual => {
                        let rhs = self.eval(*right)?;
                        match (&lhs, &rhs) {
                            (Value::Number(a), Value::Number(b)) => {
                                Value::Boolean(a <= b)
                            }

                            _ => Err(ParseError::IllegalBinaryOperation(lhs, rhs, op))?
                        }
                    },
                }
            }
            Expr::Binding { name } => self.mem.get(name)?,
            Expr::Assign { name, expr } => {
                let value = self.eval(*expr)?;
                self.mem.assign(name, value)?;
                Value::Empty
            }
            Expr::Grouping { expr } => self.eval(*expr)?,
        };

        Ok(result)
    }
}
