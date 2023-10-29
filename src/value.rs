pub enum Value {
  Empty,
  Boolean(bool),
  Number(f64),
  String(Box<String>),
}