use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectData {
    String(String),
    SomethingElse,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Object(pub Box<ObjectData>);

impl Object {
    pub fn is_empty(&self) -> bool {
        match self.0.as_ref() {
            ObjectData::String(x) => x.is_empty(),
            ObjectData::SomethingElse => todo!(),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.as_ref() {
            ObjectData::String(x) => write!(f, "{}", x),
            ObjectData::SomethingElse => todo!(),
        }
    }
}
