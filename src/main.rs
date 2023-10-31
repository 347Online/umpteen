use umpteen::{Result, Umpteen};

fn main() -> Result<()> {
    Umpteen::exec("let x = 10;")
}

#[cfg(test)]
mod tests {
    use umpteen::{chunk::Chunk, instr::Instruction, value::Value, Umpteen};

    #[test]
    fn test_hello_world() -> umpteen::Result<()> {
        Umpteen::exec("print 10;")
    }

    #[test]
    fn test_hand_assembled_chunk() -> umpteen::Result<()> {
        let mut chunk = Chunk::new();
        let constant = chunk.write_val(Value::Number(10.7));
        chunk.write_instr(Instruction::Constant);
        chunk.write_byte(constant);
        chunk.write_instr(Instruction::Print);
        println!("{:?}", chunk);
        Umpteen::run(vec![chunk])
    }
}
