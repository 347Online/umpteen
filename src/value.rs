#[derive(Clone, Copy)]
pub enum Value<'v> {
    Empty,
    Boolean(bool),
    Number(f64),
    String(&'v String),
}
