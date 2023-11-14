use crate::{
    boxed,
    error::ParseError,
    repr::{
        ast::{
            expr::Expr,
            ops::{Binary, Unary},
            stmt::Stmt,
        },
        token::{Token, TokenType},
        value::Value,
    },
    util::report_at,
};

pub enum AstNode<'a> {
    Stmt(Stmt<'a>),
    Expr(Expr<'a>),
}

pub type Ast<'a> = Vec<Stmt<'a>>;

macro_rules! catch {
    ($self:ident, $first:tt $(,$rest:tt)*) => {{
        if $self.check(TokenType::$first)$( || $self.check(TokenType::$rest))* {
            $self.advance();
            true
        } else {
            false
        }
    }};
}

macro_rules! op {
    ($self:ident, $kind:ident $(,$tk:tt => $op:tt)+) => {{
        match $self.previous().kind {
            $(
                TokenType::$tk => $kind::$op,
            )+

            _ => unreachable!(),
        }
    }};
}

macro_rules! binop {
    ($self:ident, $next:ident $(,$tk:tt => $op:tt)+) => {{
        let mut expr = $self.$next()?;
        while catch!($self$(,$tk)+) {
            let op = op!($self, Binary$(,$tk => $op)+);
            let right = boxed!($self.$next()?);
            expr = Expr::BinOp {
                left: boxed!(expr),
                right,
                op
            }
        }
        Ok(expr)
    }};
}

macro_rules! literal {
    ($self:ident $(,$tk:tt => $val:tt$(($x:expr))?)+) => {
        match $self.previous().kind {
            $(
                TokenType::$tk => Expr::Literal(Value::$val$(($x))?),
            )+

            _ => unreachable!(),
        }
    };
}

pub struct Parser<'p> {
    tokens: Vec<Token<'p>>,
    index: usize,
}

impl<'p> Parser<'p> {
    pub fn new(tokens: Vec<Token<'p>>) -> Self {
        Parser { tokens, index: 0 }
    }

    pub fn parse(&mut self) -> Ast<'p> {
        let mut ast = vec![];

        while !self.at_end() {
            match self.declaration() {
                Ok(stmt) => ast.push(stmt),
                Err(e) => {
                    report_at(e, self.peek());
                    break;
                }
            }
        }

        // ast.push(Stmt::Exit);

        #[cfg(debug_assertions)]
        dbg!(&ast);

        ast
    }

    fn declaration(&mut self) -> Result<Stmt<'p>, ParseError> {
        if catch!(self, Fnc) {
            return self.declare_fnc();
        }
        if catch!(self, Var) {
            return self.declare_variable(true);
        }
        if catch!(self, Let) {
            return self.declare_variable(false);
        }

        self.statement()
    }

    fn statement(&mut self) -> Result<Stmt<'p>, ParseError> {
        if catch!(self, If) {
            return self.conditional();
        }

        if catch!(self, Loop) {
            return self.repetition();
        }

        if catch!(self, Break) {
            self.consume(TokenType::Semicolon)?;
            return Ok(Stmt::Break);
        }

        if catch!(self, Continue) {
            self.consume(TokenType::Semicolon)?;
            return Ok(Stmt::Continue);
        }

        if catch!(self, Return) {
            if catch!(self, Semicolon) {
                return Ok(Stmt::Return(Expr::Literal(Value::Empty)));
            } else {
                let expr = self.expression()?;
                self.consume(TokenType::Semicolon)?;
                return Ok(Stmt::Return(expr));
            }
        }

        if catch!(self, LeftBrace) {
            return Ok(Stmt::Block(self.block()?));
        }

        let expr = self.expression()?;
        self.consume(TokenType::Semicolon)?;
        Ok(Stmt::Expr(expr))
    }

    fn repetition(&mut self) -> Result<Stmt<'p>, ParseError> {
        self.consume(TokenType::LeftBrace)?;
        let block = self.block()?;
        Ok(Stmt::Loop(block))
    }

    fn conditional(&mut self) -> Result<Stmt<'p>, ParseError> {
        let expr = self.expression()?;

        self.consume(TokenType::LeftBrace)?;
        let then_branch = self.block()?;
        let else_branch = if catch!(self, Else) {
            Some(self.block()?)
        } else {
            None
        };

        Ok(Stmt::Condition {
            test: expr,
            then_branch,
            else_branch,
        })
    }

    fn block(&mut self) -> Result<Ast<'p>, ParseError> {
        let mut statements = vec![];

        while !self.at_end() && !catch!(self, RightBrace) {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    fn declare_variable(&mut self, mutable: bool) -> Result<Stmt<'p>, ParseError> {
        let name = self.consume(TokenType::Identifier)?.lexeme;

        let init = if catch!(self, Equal) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::Semicolon)?;

        // TODO: Do something different for an immutable binding.
        // For now, all bindings are mutable
        Ok(Stmt::Declare { name, init })
    }

    fn declare_fnc(&mut self) -> Result<Stmt<'p>, ParseError> {
        let name = self.consume(TokenType::Identifier)?.lexeme;
        self.consume(TokenType::LeftParen)?;

        let mut first = true;

        let mut params = vec![];

        while !catch!(self, RightParen) {
            if first {
                first = false;
            } else {
                self.consume(TokenType::Comma)?;
            }

            let param = self.consume(TokenType::Identifier)?.lexeme;
            self.consume(TokenType::Colon)?;
            let param_type = self.consume(TokenType::Identifier)?.lexeme;
            params.push((param, param_type));
        }

        println!("Parsed Params: {:?}", params);

        self.consume(TokenType::ThinArrow)?;
        let return_type = self.consume(TokenType::Identifier)?.lexeme;

        println!("Return type: {}", return_type);

        self.consume(TokenType::LeftBrace)?;
        let body = self.block()?;

        println!("Fnc body: {:#?}", body);

        // TODO: Instantiate a function with args and body

        Ok(Stmt::Exit)
    }

    fn expression(&mut self) -> Result<Expr<'p>, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr<'p>, ParseError> {
        let target = self.equality()?;

        if !catch!(
            self,
            Equal,
            PlusEqual,
            MinusEqual,
            StarEqual,
            SlashEqual,
            PercentEqual
        ) {
            return Ok(target);
        }

        let op = self.previous();
        let value = self.assignment()?;

        if let Expr::Binding { name, index } = target.clone() {
            let value = if op.kind == TokenType::Equal {
                value
            } else {
                Expr::BinOp {
                    left: boxed!(target),
                    right: boxed!(value),
                    op: op.kind.try_into().unwrap(),
                }
            };

            Ok(Expr::Assign {
                name,
                index,
                expr: boxed!(value),
            })
        } else {
            Err(ParseError::InvalidAssignmentTarget(op.lexeme.to_owned()))
        }
    }

    fn equality(&mut self) -> Result<Expr<'p>, ParseError> {
        binop!(self, comparison,
            BangEqual => Inequality,
            EqualEqual => Equality
        )
    }

    fn comparison(&mut self) -> Result<Expr<'p>, ParseError> {
        binop!(self, term,
            Greater => GreaterThan,
            GreaterEqual => GreaterOrEqual,
            Less => LessThan,
            LessEqual => LessOrEqual
        )
    }

    fn term(&mut self) -> Result<Expr<'p>, ParseError> {
        binop!(self, factor,
            Plus => Add,
            Minus => Subtract
        )
    }

    fn factor(&mut self) -> Result<Expr<'p>, ParseError> {
        binop!(self, unary,
            Slash => Divide,
            Star => Multiply,
            Percent => Modulo
        )
    }

    fn unary(&mut self) -> Result<Expr<'p>, ParseError> {
        if catch!(self, Bang, Minus) {
            let op = op!(self, Unary,
                Bang => Not,
                Minus => Negate
            );
            Ok(Expr::UnOp {
                expr: boxed!(self.unary()?),
                op,
            })
        } else {
            self.call()
        }
    }

    fn call(&mut self) -> Result<Expr<'p>, ParseError> {
        let mut expr = self.primary()?;

        loop {
            if catch!(self, LeftParen) {
                expr = self.finish_call(expr)?;
                dbg!(&expr);
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr<'p>) -> Result<Expr<'p>, ParseError> {
        let mut args = vec![];

        if !self.check(TokenType::RightParen) {
            loop {
                if args.len() > 255 {
                    // TODO: Constant instead of magic number
                    eprintln!("Too many arguments, limit is 255");
                }
                args.push(self.expression()?);
                if !catch!(self, Comma) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen)?;

        let callee = boxed!(callee);
        Ok(Expr::Call { callee, args })
    }

    fn primary(&mut self) -> Result<Expr<'p>, ParseError> {
        if catch!(self, Identifier) {
            let name = self.previous().lexeme;
            if catch!(self, LeftBracket) {
                let index = Some(boxed!(self.expression()?));
                self.consume(TokenType::RightBracket)?;
                return Ok(Expr::Binding { name, index });
            }
            return Ok(Expr::Binding { name, index: None });
        }

        if catch!(self, Empty, True, False, Number, String) {
            let tk = self.previous();
            let expr = literal!(self,
                True => Boolean(true),
                False => Boolean(false),
                Empty => Empty,
                Number => Number(tk.lexeme.parse()?),
                String => String(boxed!(tk.lexeme.to_owned()))
            );

            return Ok(expr);
        }

        if catch!(self, LeftParen) {
            let expr = boxed!(self.expression()?);
            self.consume(TokenType::RightParen)?;
            return Ok(Expr::Grouping { expr });
        }

        if catch!(self, LeftBracket) {
            let mut list = vec![];

            while !self.check(TokenType::RightBracket) {
                let expr = self.expression()?;
                list.push(expr);
                if !catch!(self, Comma) {
                    break;
                }
            }
            self.consume(TokenType::RightBracket)?;
            return Ok(Expr::List(list));
        }

        Err(ParseError::UnexpectedToken(self.peek().kind))
    }

    fn advance(&mut self) -> Token<'p> {
        if !self.at_end() {
            self.index += 1;
        }
        self.previous()
    }

    fn consume(&mut self, kind: TokenType) -> Result<Token<'p>, ParseError> {
        if self.check(kind) {
            Ok(dbg!(self.advance()))
        } else {
            Err(ParseError::ExpectedToken(kind))
        }
    }

    fn previous(&self) -> Token<'p> {
        self.tokens[self.index.saturating_sub(1)]
    }

    fn check(&self, kind: TokenType) -> bool {
        if self.at_end() {
            false
        } else {
            self.peek().kind == kind
        }
    }

    fn at_end(&self) -> bool {
        self.peek().kind == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.index]
    }
}
