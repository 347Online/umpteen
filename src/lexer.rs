use crate::repr::token::{Token, TokenType};

pub struct Lexer<'s> {
    source: &'s str,
    tokens: Vec<Token<'s>>,
    line: usize,
}
impl<'s> Lexer<'s> {
    pub fn new(source: &'s str) -> Self {
        Lexer {
            source,
            tokens: vec![],
            line: 1,
        }
    }

    pub fn scan(mut self) -> Vec<Token<'s>> {
        let source = &self.source;
        let mut src = self.source.char_indices().peekable();

        while let Some((i, c)) = src.peek() {
            match c {
                '\n' => self.line += 1,

                _ if c.is_ascii_whitespace() => (),

                ';' => {
                    let lx = &source[*i..*i];
                    let tk = Token::new(TokenType::Semicolon, lx, self.line);
                    self.tokens.push(tk);
                }
                _ => todo!(),
            }
            src.next();
        }
        self.tokens
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;

    #[test]
    fn test_lex_semi() {
        let source = "    ; \n\n  ; ";
        let lexer = Lexer::new(source);
        let tokens = lexer.scan();
        dbg!(tokens);
    }
}
