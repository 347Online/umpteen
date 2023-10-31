use umpteen::{Result, Umpteen, value::Value};

fn main() -> Result<Value> {
    Umpteen::run("let x = 10;")
}

#[cfg(test)]
mod tests {
    use umpteen::{chunk::Chunk, instr::Instruction, value::Value, Umpteen};

    #[test]
    fn test_hello_world() -> umpteen::Result<Value> {
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
        let val = chunk.exec();

        Ok(())
    }
}
