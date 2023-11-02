use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

use crate::repr::token::{Token, TokenType};

pub struct Lexer<'s> {
    source: &'s str,
    chars: Peekable<Enumerate<Chars<'s>>>,
    line: usize,
}

impl<'s> Lexer<'s> {
    pub fn new(source: &'s str) -> Self {
        Lexer {
            source,
            chars: source.chars().enumerate().peekable(),
            line: 1,
        }
    }

    fn peek(&mut self) -> Option<(usize, &char)> {
        let (i, c) = self.chars.peek()?;
        Some((*i, c))
    }

    fn advance(&mut self) -> Option<(usize, char)> {
        self.chars.next()
    }

    fn scan_token(&mut self) -> Option<Token<'s>> {
        let Some((start, _)) = self.peek() else {
            return None;
        };
        let (i, c) = self.advance().unwrap();

        macro_rules! token {
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
                self.scan_token()?
            }
            ';' => token!(Semicolon),
            '=' => token!(Equal),
            c if c.is_whitespace() => self.scan_token()?,
            _ => todo!(),
        };
        dbg!(&tk);
        Some(tk)
    }

    pub fn scan(mut self) -> Vec<Token<'s>> {
        let mut tokens = vec![];
        while let Some(token) = self.scan_token() {
            tokens.push(token);
        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;

    #[test]
    fn test_lex_semi() {
        let source = ";=     

       123

           ;;;";
        dbg!(&source);
        let lexer = Lexer::new(source);
        let tokens = lexer.scan();
        dbg!(tokens);
    }
}
