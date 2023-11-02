use repr::error::Error;

pub mod exec;
pub mod repr;

pub type Result<T> = std::result::Result<T, Error>;
