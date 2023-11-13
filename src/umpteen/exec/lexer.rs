use std::{iter::Peekable, str::Chars};

use crate::{
    error::Line,
    repr::token::{Token, TokenType},
    util::report_line,
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
        self.line.newline();
        tokens.push(Token::new(TokenType::Eof, "<EOF>", self.line));

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
        self.line.advance();
        self.offset += 1;
        self.chars.next()
    }

    fn scan_token(&mut self) -> Option<Token<'s>> {
        if self.peek().is_none() {
            self.finished = true;
            return None;
        }

        let start = self.offset;

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
            ($start:expr, $end:expr) => {
                &self.source[$start..$end]
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

            '(' => token!(LeftParen),
            ')' => token!(RightParen),
            '{' => token!(LeftBrace),
            '}' => token!(RightBrace),
            ';' => token!(Semicolon),
            '+' => token!(Plus),
            '-' => token!(Minus),
            '*' => token!(Asterisk),
            '/' => token!(Slash),
            '%' => token!(Percent),

            '>' => {
                if self.peek() == Some('=') {
                    self.advance();
                    token!(GreaterEqual)
                } else {
                    token!(Greater)
                }
            },
            '<' => {
                if self.peek() == Some('=') {
                    self.advance();
                    token!(LessEqual)
                } else {
                    token!(Less)
                }
            },
            '=' => {
                if self.peek() == Some('=') {
                    self.advance();
                    token!(EqualEqual)
                } else {
                    token!(Equal)
                }
            },
            '!' => {
                if self.peek() == Some('=') {
                    self.advance();
                    token!(BangEqual)
                } else {
                    token!(Bang)
                }
            },
            '&' if self.peek() == Some('&') => {
                self.advance();
                token!(And)
            },
            '|' if self.peek() == Some('|') => {
                self.advance();
                token!(Or)
            },

            '"' => {
                let mut end: usize = self.offset;
                while self.peek().is_some() {
                    end = self.offset;
                    let c = self.advance().unwrap();
                    if c == '"' {
                        break;
                    }
                }
                token!(String, lexeme!(start + 1, end))
            }

            c if c.is_ascii_digit() => {
                let mut end = self.offset;
                macro_rules! digits {
                    () => {
                        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                            self.advance().unwrap();
                            end = self.offset;
                        }
                    };
                }

                digits!();

                // Matches a decimal point and at least one additional digit
                if matches!((self.peek(), self.peek_ahead(1)), (Some('.'), Some(c)) if c.is_ascii_digit())
                {
                    self.advance(); // Skip the decimal point
                    digits!()
                }

                let lx = lexeme!(end);
                token!(Number, lx)
            }

            c if is_identic(c) => {
                let mut end: usize = self.offset;
                while self.peek().is_some_and(is_identic) {
                    self.advance().unwrap();
                    end = self.offset;
                }

                let lx = lexeme!(end);

                match lx {
                    "Empty" => token!(Empty, lx),
                    "true" => token!(True, lx),
                    "false" => token!(False, lx),

                    "var" => token!(Var, lx),
                    "let" => token!(Let, lx),
                    "if" => token!(If, lx),
                    "else" => token!(Else, lx),
                    "loop" => token!(Loop, lx),
                    "break" => token!(Break, lx),
                    "continue" => token!(Continue, lx),
                    "print" => token!(Print, lx), // TODO: Re-implement as a function

                    _ => token!(Identifier, lx),
                }
            }

            c => {
                report_line(format!("Unexpected Symbol `{}`", c), self.line);
                None?
            }
        };

        Some(tk)
    }
}
