use std::fmt::Display;

pub type UmpResult<T> = Result<T, UmpError>;

#[derive(Debug)]
pub enum UmpErrorType {
    Unknown,
    UnexpectedEof,
}

impl Display for UmpErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            Self::Unknown => "Unknown error",
            Self::UnexpectedEof => "Unexpected end of file",
        };

        write!(f, "{}", desc)
    }
}

#[derive(Debug)]
pub struct UmpError {
    message: String,
    kind: UmpErrorType,
    line: u32,
}

impl UmpError {
    pub fn new(message: &str, kind: UmpErrorType, line: u32) -> Self {
        Self {
            message: message.to_string(),
            kind,
            line,
        }
    }

    pub fn unkown() -> Self {
        Self::new("An unknown error occurred", UmpErrorType::Unknown, 0)
    }
}

impl Display for UmpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error encountered at [{}]: {}\n{}",
            self.line, self.kind, self.message
        )
    }
}
