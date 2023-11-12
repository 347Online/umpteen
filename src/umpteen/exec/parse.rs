use crate::{
    error::{Line, ParseError},
    repr::{
        ast::{
            expr::Expr,
            ops::{Binary, Unary},
            stmt::Stmt,
        },
        token::{Token, TokenType},
        value::Value,
    }, util::report_at,
};

pub enum AstNode<'a> {
    Stmt(Stmt<'a>),
    Expr(Expr<'a>),
}

pub type Ast<'a> = Vec<Stmt<'a>>;

macro_rules! catch {
    ($self:ident, $first:tt $(,$rest:tt)*) => {
        if $self.check(TokenType::$first)$( || $self.check(TokenType::$rest))* {
            $self.advance();
            true
        } else {
            false
        }
    };
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

        loop {
            let stmt = match self.statement() {
                Ok(stmt) => stmt,
                Err(e) => {
                    report_at(e, self.peek());
                    break;
                }
            };
            ast.push(stmt);
            if self.index == self.tokens.len() {
                break;
            }
        }

        #[cfg(debug_assertions)]
        dbg!(&ast);

        ast
    }

    fn statement(&mut self) -> Result<Stmt<'p>, ParseError> {
        if catch!(self, Print) {
            return self.print();
        }

        if catch!(self, Eof) {
            return Ok(Stmt::Exit);
        }

        let expr = self.expression()?;
        self.consume(TokenType::Semicolon)?;
        Ok(Stmt::Expr(expr))
    }

    fn print(&mut self) -> Result<Stmt<'p>, ParseError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon)?;
        Ok(Stmt::Print(value))
    }

    fn expression(&mut self) -> Result<Expr<'p>, ParseError> {
        self.equality()
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
            todo!()
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
            Ok(Expr::Grouping { expr })
        } else {
            Err(ParseError::ExpectedExpression)
        }
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
        self.tokens[self.index - 1]
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
        // self.tokens.get(self.index).copied().unwrap_or(Token {
        //     kind: TokenType::Eof,
        //     lexeme: "<EOF>",
        //     line: self.line,
        // })
    }
}
