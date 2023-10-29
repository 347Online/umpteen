use std::fmt::Display;

pub type UmpResult<T> = Result<T, UmpError>;

#[derive(Debug)]
pub enum UmpErrorType {
    Unknown,
    UnexpectedEof,
    InvalidOpcode(u8),
}

impl Display for UmpErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tmp;
        let desc = match self {
            Self::Unknown => "Unknown error",
            Self::UnexpectedEof => "Unexpected end of file",
            Self::InvalidOpcode(byte) => {
                tmp = format!("Invalid Opcode `{byte}`");
                &tmp
            }
        };

        write!(f, "{}", desc)
    }
}

#[derive(Debug)]
pub struct UmpError {
    kind: UmpErrorType,
    line: usize,
}

impl UmpError {
    pub fn new(kind: UmpErrorType, line: usize) -> Self {
        Self { kind, line }
    }

    pub fn unknown(line: usize) -> Self {
        Self::new(UmpErrorType::Unknown, line)
    }

    pub fn invalid_opcode(byte: u8) -> Self {
        Self::new(UmpErrorType::InvalidOpcode(byte), 0)
    }
}

impl Display for UmpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.line == 0 {
            write!(f, "Error encountered: {}", self.kind)
        } else {
            write!(
                f,
                "Error encountered on line [{}]: {}",
                self.line, self.kind
            )
        }
    }
}
