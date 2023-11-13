use crate::{
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
            let right = Box::new($self.$next()?);
            expr = Expr::BinOp {
                left: Box::new(expr),
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

        ast.push(Stmt::Exit);

        #[cfg(debug_assertions)]
        dbg!(&ast);

        ast
    }

    fn declaration(&mut self) -> Result<Stmt<'p>, ParseError> {
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

        if catch!(self, Print) {
            return self.print();
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

        if catch!(self, LeftBrace) {
            return self.block();
        }

        let expr = self.expression()?;
        self.consume(TokenType::Semicolon)?;
        Ok(Stmt::Expr(expr))
    }

    fn repetition(&mut self) -> Result<Stmt<'p>, ParseError> {
        let block = Box::new(self.block()?);
        Ok(Stmt::Loop(block))
    }

    fn conditional(&mut self) -> Result<Stmt<'p>, ParseError> {
        let expr = self.expression()?;

        let then_branch = Box::new(self.block()?);
        let else_branch = if catch!(self, Else) {
            if catch!(self, If) {
                Some(Box::new(self.conditional()?))
            } else {
                Some(Box::new(self.block()?))
            }
        } else {
            None
        };

        Ok(Stmt::Condition {
            test: expr,
            then_branch,
            else_branch,
        })
    }

    fn block(&mut self) -> Result<Stmt<'p>, ParseError> {
        let mut statements = vec![];

        while !self.check(TokenType::RightBrace) && !self.at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace)?;

        Ok(Stmt::Block(statements))
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

    fn print(&mut self) -> Result<Stmt<'p>, ParseError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon)?;
        Ok(Stmt::Print(value))
    }

    fn expression(&mut self) -> Result<Expr<'p>, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr<'p>, ParseError> {
        let expr = self.equality()?;

        if catch!(self, Equal) {
            let equals = self.previous();
            let value = self.assignment()?;

            if let Expr::Binding { name } = expr {
                let expr = Box::new(value);
                return Ok(Expr::Assign { name, expr });
            }

            report_at("Invalid Assignment Target", equals);
        }

        Ok(expr)
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
            Asterisk => Multiply,
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
                expr: Box::new(self.unary()?),
                op,
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr<'p>, ParseError> {
        if catch!(self, Identifier) {
            let name = self.previous().lexeme;
            return Ok(Expr::Binding { name });
        }

        if catch!(self, Empty, True, False, Number, String) {
            let tk = self.previous();
            let expr = literal!(self,
                True => Boolean(true),
                False => Boolean(false),
                Empty => Empty,
                Number => Number(tk.lexeme.parse()?),
                String => String(Box::new(tk.lexeme.to_owned()))
            );

            return Ok(expr);
        }

        if catch!(self, LeftParen) {
            let expr = Box::new(self.expression()?);
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

        Err(ParseError::UnexpectedToken(self.previous().kind))
    }

    fn advance(&mut self) -> Token<'p> {
        if !self.at_end() {
            self.index += 1;
        }
        self.previous()
    }

    fn consume(&mut self, kind: TokenType) -> Result<Token<'p>, ParseError> {
        if self.check(kind) {
            Ok(self.advance())
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
