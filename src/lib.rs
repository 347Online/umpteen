use std::{iter::Peekable, str::Chars};

use repr::{
    error::Error,
    token::TokenType,
    token::{print_tokens, Token},
    value::Value,
};

pub mod repr;
pub mod vm;

pub type Result<T> = std::result::Result<T, Error>;
