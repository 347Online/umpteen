use self::error::Error;

pub mod chunk;
pub mod error;
pub mod instr;
pub mod token;
pub mod value;

pub type Result<T> = std::result::Result<T, Error>;
