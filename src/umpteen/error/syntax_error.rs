use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum SyntaxError {
    UnexpectedSymbol(char),
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tmp: String;
        let desc = match self {
            SyntaxError::UnexpectedSymbol(c) => {
                tmp = format!("unexpected symbol `{}`", c);
                &tmp
            }
        };
        write!(f, "{}", desc)
    }
}

impl Error for SyntaxError {}
