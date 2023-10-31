use umpteen::{Result, Umpteen};

fn main() -> Result<()> {
    Umpteen::run("let x = 10;")
}

#[cfg(test)]
mod tests {
    use umpteen::{chunk::Chunk, instr::Instruction, value::Value, Umpteen};

    #[test]
    fn test_hello_world() -> umpteen::Result<()> {
        Umpteen::run("print 10;")
    }

    #[test]
    fn test_hand_assembled_chunk() -> umpteen::Result<()> {
        let mut chunk = Chunk::new();
        let constant = chunk.write_val(Value::Number(10.7));
        chunk.write_instr(Instruction::U8);
        chunk.write_byte(constant as u8);
        chunk.write_instr(Instruction::Print);
        println!("{:?}", chunk);
        chunk.exec()
    }
}
