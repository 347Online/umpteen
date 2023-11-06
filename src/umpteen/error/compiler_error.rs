use std::fmt::Display;

#[derive(Debug)]
pub enum CompilerError {
    CorruptedChunk,
    InvalidInstruction(u8),
    WrongNumberArguments(usize, usize, String),
    IllegalDeclare,
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tmp: String;
        let desc = match self {
            CompilerError::CorruptedChunk => "encountered corrupted chunk",
            CompilerError::InvalidInstruction(byte) => {
                tmp = format!("invalid Instruction `{}`", byte);
                &tmp
            }
            CompilerError::IllegalDeclare => "illegal declaration",

            CompilerError::WrongNumberArguments(exp, got, call) => {
                tmp = format!(
                    "wrong number of arguments for {}, expected {} but got {}",
                    call, exp, got
                );
                &tmp
            }
        };
        write!(f, "{}", desc)
    }
}
