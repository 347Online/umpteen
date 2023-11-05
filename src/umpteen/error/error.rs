use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    UnexpectedEof,
    CorruptedChunk,
    UnexpectedToken(char),
    InvalidInstruction(u8),
    WrongNumberArguments(usize, usize, String),
    IllegalDeclare,
    Runtime(RuntimeError),
}

impl std::error::Error for Error {}
impl std::error::Error for RuntimeError {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tmp: String;

        let desc = match self {
            Error::CorruptedChunk => "encountered corrupted chunk",
            Error::UnexpectedEof => "unexpected end of file",
            Error::IllegalDeclare => "illegal declaration",

            Error::UnexpectedToken(c) => {
                tmp = format!("unexpected token `{}`", c);
                &tmp
            }

            Error::InvalidInstruction(byte) => {
                tmp = format!("invalid Instruction `{}`", byte);
                &tmp
            }

            Error::WrongNumberArguments(exp, got, call) => {
                tmp = format!(
                    "wrong number of arguments for {}, expected {} but got {}",
                    call, exp, got
                );
                &tmp
            }

            Error::Runtime(r) => {
                tmp = format!("{}", r);
                &tmp
            }
        };
        write!(f, "{desc}")
    }
}

impl From<RuntimeError> for Error {
    fn from(value: RuntimeError) -> Self {
        Error::Runtime(value)
    }
}

#[derive(Debug)]
pub enum RuntimeError {
    Illegal, // TODO: Make this better
    OutOfBoundsMemoryAccess,
    StackMissingValue,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            RuntimeError::Illegal => "illegal operation",
            RuntimeError::OutOfBoundsMemoryAccess => "out of bounds memory access",
            RuntimeError::StackMissingValue => "popped when stack was empty",
        };
        write!(f, "{}", desc)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Line(usize, usize);

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.1 == 0 {
            write!(f, "{}", self.0)
        } else {
            write!(f, "{}:{}", self.0, self.1)
        }
    }
}

pub fn report(e: Error) {
    eprintln!("ERR: {e}");
}

pub fn report_line(e: Error, line: usize, col: usize) {
    eprintln!("ERR: {} on line {}:{}", e, line, col);
}
