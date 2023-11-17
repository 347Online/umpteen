use std::fmt::Display;

use crate::{error::Line, repr::token::Token};

pub fn report_line<I: Display>(error: I, line: Line) {
    eprintln!("ERR: {} on line {}", error, line);
}

pub fn report_at<I: Display>(error: I, tk: Token) {
    eprintln!("ERR: {} at `{}` on line {}", error, tk.lexeme, tk.line)
}

pub fn unescape(escaped: &str) -> String {
    let mut output = String::with_capacity(escaped.len());
    let mut chars = escaped.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' {
            let Some(c) = chars.next() else {
                output.push('\\');
                break;
            };

            match c {
                'n' => output.push('\n'),
                't' => output.push('\t'),
                'r' => output.push('\r'),
                '"' => output.push('"'),
                '\'' => output.push('\''),
                '\\' => output.push('\\'),

                c => return format!("Unsupported Escape Sequence \\{}", c),
            }
        } else {
            output.push(c);
        }
    }

    output
}

#[macro_export]
macro_rules! boxed {
    ($e:expr) => {
        Box::new($e)
    };
}
