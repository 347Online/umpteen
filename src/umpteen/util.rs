use std::fmt::Display;

use crate::error::Line;

pub fn report<E: Display>(error: E) {
    eprintln!("ERR: {}", error);
}

pub fn report_line<E: Display>(error: E, line: Line) {
    eprintln!("ERR: {} on line {}", error, line);
}
