use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

use crate::repr::token::{Token, TokenType};

pub struct Lexer<'s> {
    source: &'s str,
    chars: Peekable<Enumerate<Chars<'s>>>,
    line: usize,
    finished: bool,
}

impl<'s> Lexer<'s> {
    pub fn new(source: &'s str) -> Self {
        let chars = source.chars().enumerate().peekable();

        Lexer {
            source,
            chars,
            line: 1,
            finished: false,
        }
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
        let (i, c) = self.advance().unwrap();

        macro_rules! token {
            ($t:tt, $e:expr) => {{
                let lx = &self.source[start..=$e];
                dbg!(&lx);
                Token::new(TokenType::$t, lx, self.line)
            }};
            ($t:tt) => {{
                let lx = &self.source[start..=i];
                dbg!(&lx);
                Token::new(TokenType::$t, lx, self.line)
            }};
        }
        dbg!(c);
        let tk = match c {
            '\n' => {
                self.line += 1;
                return None;
            }
            c if c.is_whitespace() => return None,

            ';' => token!(Semicolon),
            '=' => token!(Equal),

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

                token!(Number, end)
            }

            c if is_identic(c) => {
                let mut end: usize = i;
                while self.peek().is_some_and(|(_, c)| is_identic(c)) {
                    let (i, _) = self.advance().unwrap();
                    end = i;
                }

                token!(Identifier, end)
            }

            _ => todo!(),
        };
        Some(tk)
    }

    pub fn scan(mut self) -> Vec<Token<'s>> {
        let mut tokens = vec![];
        while !self.finished {
            if let Some(token) = self.scan_token() {
                dbg!(&token);
                tokens.push(token);
            }
        }
        tokens
    }
}

fn is_identic(c: char) -> bool {
    c == '_' || c.is_alphanumeric()
}

#[cfg(test)]
mod tests {
    use super::Lexer;

    #[test]
    fn test_lex() {
        let source = "let x = 10;";
        dbg!(&source);
        let lexer = Lexer::new(source);
        let tokens = lexer.scan();
        dbg!(&tokens, tokens.len());
    }
}
