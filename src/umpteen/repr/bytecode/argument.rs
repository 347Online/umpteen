#[derive(Debug)]
pub enum Arg {
    Address(usize),
}

impl From<usize> for Arg {
    fn from(value: usize) -> Self {
        Arg::Address(value)
    }
}
