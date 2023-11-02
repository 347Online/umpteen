use repr::error::Error;

pub mod repr;
pub mod exec;

pub type Result<T> = std::result::Result<T, Error>;
