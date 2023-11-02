use repr::error::Error;

pub mod repr;
pub mod vm;

pub type Result<T> = std::result::Result<T, Error>;
