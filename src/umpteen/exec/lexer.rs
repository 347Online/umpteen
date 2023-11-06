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
    chars: Peekable<Enumerate<Chars<'s>>>,
    line: Line,
    finished: bool,
}

impl<'s> Lexer<'s> {
    pub fn new(source: &'s str) -> Self {
        let chars = source.chars().enumerate().peekable();

        Lexer {
            source,
            chars,
            line: Line::new(1),
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
        tokens
    }

    fn peek(&mut self) -> Option<(usize, char)> {
        let (i, c) = self.chars.peek()?;
        Some((*i, *c))
    }
    fn peek_ahead(&mut self, n: usize) -> Option<(usize, char)> {
        let x = self.chars.clone().nth(n)?;

        Some(x)
    }

    fn advance(&mut self) -> Option<(usize, char)> {
        self.chars.next()
    }

    fn scan_token(&mut self) -> Option<Token<'s>> {
        let Some((start, _)) = self.peek() else {
            self.finished = true;
            return None;
        };
        self.line.advance();
        let (i, c) = self.advance().unwrap();

        fn is_identic(c: char) -> bool {
            c == '_' || c.is_alphanumeric()
        }

        macro_rules! lexeme {
            () => {
                &self.source[start..=i]
            };
            ($end:expr) => {
                &self.source[start..=$end]
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
            '=' => token!(Equal),

            '"' => {
                let mut end: usize = 0;
                self.advance();
                while self.peek().is_some() {
                    let (i, c) = self.advance().unwrap();
                    end = i;
                    if c == '"' {
                        break;
                    }
                }
                token!(String, lexeme!(end))
            }

            c if c.is_ascii_digit() => {
                let mut end: usize = i;
                let mut dec = false;
                while self.peek().is_some_and(|(_, c)| {
                    c.is_ascii_digit()
                        || ({
                            if let (false, Some((_, '.')), Some((_, y))) =
                                (dec, self.peek(), self.peek_ahead(1))
                            {
                                y.is_ascii_digit()
                            } else {
                                false
                            }
                        })
                }) {
                    let (i, c) = self.advance().unwrap();
                    if c == '.' {
                        dec = true;
                    }
                    end = i;
                }

                let lx = lexeme!(end);
                token!(Number, lx)
            }

            c if is_identic(c) => {
                let mut end: usize = i;
                while self.peek().is_some_and(|(_, c)| is_identic(c)) {
                    let (i, _) = self.advance().unwrap();
                    end = i;
                }

                let lx = lexeme!(end);

                match lx {
                    "let" => token!(Let, lx),
                    "print" => token!(Print, lx),

                    _ => token!(Identifier, lx),
                }
            }

            _ => todo!(),
        };

        Some(tk)
    }
}
