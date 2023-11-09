use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

use crate::{
    error::Line,
    repr::token::{Token, TokenType},
};

pub struct Lexer<'s> {
    source: &'s str,
    chars: Peekable<Chars<'s>>,
    line: Line,
    offset: usize,
    finished: bool,
}

impl<'s> Lexer<'s> {
    pub fn new(source: &'s str) -> Self {
        let chars = source.chars().peekable();

        Lexer {
            source,
            chars,
            line: Line::new(1),
            offset: 0,
            finished: false,
        }
    }

    pub fn scan(mut self) -> Vec<Token<'s>> {
        let mut tokens = vec![];
        while !self.finished {
            if let Some(token) = self.scan_token() {
                tokens.push(token);
            }
        }

        #[cfg(debug_assertions)]
        dbg!(&tokens);

        tokens
    }

    fn peek(&mut self) -> Option<char> {
        let c = self.chars.peek()?;
        Some(*c)
    }
    fn peek_ahead(&mut self, n: usize) -> Option<char> {
        let x = self.chars.clone().nth(n)?;

        Some(x)
    }

    fn advance(&mut self) -> Option<char> {
        self.offset += 1;
        self.chars.next()
    }

    fn scan_token(&mut self) -> Option<Token<'s>> {
        if self.peek().is_none() {
            self.finished = true;
            return None;
        }

        let start = self.offset;

        self.line.advance();
        let c = self.advance().unwrap();

        fn is_identic(c: char) -> bool {
            c == '_' || c.is_alphanumeric()
        }

        macro_rules! lexeme {
            () => {
                &self.source[start..self.offset]
            };
            ($end:expr) => {
                &self.source[start..$end]
            };
        }

        macro_rules! token {
            ($t:tt, $lx:expr) => {
                Token::new(TokenType::$t, $lx, self.line)
            };
            ($t:tt) => {
                Token::new(TokenType::$t, lexeme!(), self.line)
            };
        }

        let tk = match c {
            '\n' => {
                self.line.newline();
                return None;
            }
            c if c.is_whitespace() => return None,

            ';' => token!(Semicolon),
            '!' => token!(Bang),
            '+' => token!(Plus),
            '-' => token!(Minus),
            '*' => token!(Asterisk),
            '/' => token!(Slash), // TODO: Comments
            '%' => token!(Percent),
            '=' => token!(Equal),

            '"' => {
                let mut end: usize = self.offset;
                while self.peek().is_some() {
                    let c = self.advance().unwrap();
                    end = self.offset;
                    if c == '"' {
                        break;
                    }
                }
                token!(String, lexeme!(end))
            }

            c if c.is_ascii_digit() => {
                let mut end: usize = self.offset;
                let mut dec = false;
                while self.peek().is_some_and(|x| {
                    c.is_ascii_digit()
                        || ({
                            if let (false, Some('.'), Some(y)) =
                                (dec, self.peek(), self.peek_ahead(1))
                            {
                                y.is_ascii_digit()
                            } else {
                                false
                            }
                        })
                }) {
                    let c = self.advance().unwrap();
                    if c == '.' {
                        dec = true;
                    }
                    end = self.offset;
                }

                let lx = lexeme!(end);
                token!(Number, lx)
            }

            c if is_identic(c) => {
                let mut end: usize = self.offset;
                while self.peek().is_some_and(|c| is_identic(c)) {
                    self.advance().unwrap();
                    end = self.offset;
                }

                let lx = lexeme!(end);

                match lx {
                    "Empty" => token!(Empty, lx),
                    "true" => token!(True, lx),
                    "false" => token!(False, lx),

                    "let" => token!(Let, lx),
                    "print" => token!(Print, lx), // TODO: Re-implement as a function

                    _ => token!(Identifier, lx),
                }
            }

            _ => todo!(),
        };

        Some(tk)
    }
}
