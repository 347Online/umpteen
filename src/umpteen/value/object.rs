#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    String(Box<String>),
}

impl Object {
    pub fn is_empty(&self) -> bool {
        match self {
            Object::String(x) => x.is_empty(),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::String(x) => write!(f, "{}", x),
        }
    }
}
