use uuid::Uuid;

use crate::{
    error::{MemoryError, ParseError, RuntimeError, UmpteenError},
    repr::{
        ast::{
            expr::Expr,
            ops::{Binary, Unary},
            stmt::Stmt,
        },
        token::Token,
        value::{Object, Value},
    },
};

use super::{
    env::Env,
    lexer::Lexer,
    parse::{Ast, Parser},
};

pub enum Eval {
    Value(Value),
    Variable(String),
}

#[derive(Debug, Default)]
pub struct Interpreter {
    env: Env,
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
            match self.exec(&stmt)? {
                Some(val) => {
                    return_value = val;
                }
                None => break,
            }
        }

        Ok(return_value)
    }

    fn exec(&mut self, stmt: &Stmt) -> Result<Option<Value>, UmpteenError> {
        match stmt {
            Stmt::Declare { name, init } => {
                self.env.declare(name)?;

                if let Some(expr) = init {
                    let value = self.eval(expr)?;
                    self.env.assign(name, None, value)?;
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
                let mem_key = Some(self.env.new_enclosed());
                self.exec_block(statements, mem_key)?;
            }
            Stmt::Condition {
                test,
                then_branch,
                else_branch,
            } => {
                if self.eval(test)?.truthy() {
                    let then_scope = self.env.new_enclosed();
                    self.exec_block(then_branch, Some(then_scope))?;
                } else if let Some(else_branch) = else_branch {
                    let else_scope = self.env.new_enclosed();
                    self.exec_block(else_branch, Some(else_scope))?;
                }
            }
            Stmt::Break => Err(RuntimeError::Break)?,
            Stmt::Continue => Err(RuntimeError::Continue)?,
            Stmt::Loop(body) => {
                let loop_scope = self.env.new_enclosed();
                loop {
                match self.exec_block(body, Some(loop_scope)) {
                    Err(UmpteenError::RuntimeError(RuntimeError::Break)) => break,
                    Err(UmpteenError::RuntimeError(RuntimeError::Continue)) => continue,
                    x => x,
                }?;
            }},
        }

        Ok(Some(Value::Empty))
    }

    fn exec_block(&mut self, statements: &Ast, env_id: Option<Uuid>) -> Result<(), UmpteenError> {
        let mut res = Ok(());
        let previous = self.env.set_current(env_id);

        for stmt in statements {
            match self.exec(stmt) {
                Ok(_) => (),
                Err(err) => {
                    res = Err(err);
                    break;
                }
            }
        }

        self.env.set_current(previous);
        res
    }

    fn eval(&mut self, expr: &Expr) -> Result<Value, UmpteenError> {
        let result = match expr {
            Expr::Literal(value) => value.clone(),
            Expr::List(expressions) => {
                let mut values = vec![];
                for expr in expressions {
                    values.push(self.eval(expr)?);
                }
                Value::Object(Box::new(Object::List(values)))
            }
            Expr::UnOp { expr, op } => {
                let value = self.eval(expr)?;
                match op {
                    Unary::Not => !value,
                    Unary::Negate => (-value)?,
                }
            }
            Expr::BinOp { left, right, op } => {
                let lhs = self.eval(left)?;

                match op {
                    Binary::Add => (lhs + self.eval(right)?)?,
                    Binary::Subtract => (lhs - self.eval(right)?)?,
                    Binary::Multiply => (lhs * self.eval(right)?)?,
                    Binary::Divide => (lhs / self.eval(right)?)?,
                    Binary::Modulo => (lhs / self.eval(right)?)?,
                    Binary::And => {
                        if lhs.truthy() {
                            self.eval(right)?
                        } else {
                            lhs
                        }
                    }
                    Binary::Or => {
                        if lhs.truthy() {
                            lhs
                        } else {
                            self.eval(right)?
                        }
                    }
                    Binary::Equality => Value::Boolean(lhs == self.eval(right)?),
                    Binary::Inequality => Value::Boolean(lhs != self.eval(right)?),
                    Binary::GreaterThan => {
                        let rhs = self.eval(right)?;
                        match (&lhs, &rhs) {
                            (Value::Number(a), Value::Number(b)) => Value::Boolean(a > b),

                            _ => Err(ParseError::IllegalBinaryOperation(lhs, rhs, *op))?,
                        }
                    }
                    Binary::GreaterOrEqual => {
                        let rhs = self.eval(right)?;
                        match (&lhs, &rhs) {
                            (Value::Number(a), Value::Number(b)) => Value::Boolean(a >= b),

                            _ => Err(ParseError::IllegalBinaryOperation(lhs, rhs, *op))?,
                        }
                    }
                    Binary::LessThan => {
                        let rhs = self.eval(right)?;
                        match (&lhs, &rhs) {
                            (Value::Number(a), Value::Number(b)) => Value::Boolean(a < b),

                            _ => Err(ParseError::IllegalBinaryOperation(lhs, rhs, *op))?,
                        }
                    }
                    Binary::LessOrEqual => {
                        let rhs = self.eval(right)?;
                        match (&lhs, &rhs) {
                            (Value::Number(a), Value::Number(b)) => Value::Boolean(a <= b),

                            _ => Err(ParseError::IllegalBinaryOperation(lhs, rhs, *op))?,
                        }
                    }
                }
            }
            Expr::Binding { name, index } => {
                if let Some(expr) = index {
                    let idx = self.eval(expr)?;
                    if let Value::Number(num) = idx {
                        self.env.get(name, Some(num as usize))?
                    } else {
                        Err(MemoryError::CannotIndexWith(name.to_string()))?
                    }
                } else {
                    self.env.get(name, None)?
                }
            }
            Expr::Assign { name, index, expr } => {
                let value = self.eval(expr)?;
                if let Some(expr) = index {
                    let idx = self.eval(expr)?;
                    if let Value::Number(num) = idx {
                        self.env.assign(name, Some(num as usize), value)?;
                        Value::Empty
                    } else {
                        Err(MemoryError::CannotIndexWith(name.to_string()))?
                    }
                } else {
                    self.env.assign(name, None, value)?;
                    Value::Empty
                }
            }
            Expr::Grouping { expr } => self.eval(expr)?,
        };

        Ok(result)
    }
}
