use std::fmt::Display;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum Value {
    #[default]
    Empty,
    Boolean(bool),
    Number(f64),
    String(Box<String>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tmp;

        let repr = match self {
            Value::Empty => "<Empty>",
            Value::Boolean(x) => {
                tmp = x.to_string();
                &tmp
            }
            Value::Number(x) => {
                tmp = x.to_string();
                &tmp
            }
            Value::String(x) => x,
        };

        write!(f, "{repr}")
    }
}

impl From<Option<Value>> for Value {
    fn from(value: Option<Value>) -> Self {
        value.unwrap_or(Value::Empty)
    }
}
