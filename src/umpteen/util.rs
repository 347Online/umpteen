use std::fmt::Display;

use crate::{error::Line, repr::token::Token};

fn report<I: Display>(error: I) {
    eprintln!("ERR: {}", error);
}

pub fn report_line<I: Display>(error: I, line: Line) {
    eprintln!("ERR: {} on line {}", error, line);
}

pub fn report_at<I: Display>(error: I, tk: Token) {
    eprintln!("ERR: {} at {} on line {}", error, tk.lexeme, tk.line)
}
