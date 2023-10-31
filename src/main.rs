use umpteen::{chunk::Chunk, instr::Instruction, run, value::Value, Result};

fn main() -> Result<()> {
    let mut chunk = Chunk::new();
    let constant = chunk.write_val(Value::Number(10.7));
    chunk.write_instr(Instruction::Constant);
    chunk.write_byte(constant);
    chunk.write_instr(Instruction::Print);
    println!("{:?}", chunk);
    run(vec![chunk])
}

#[cfg(test)]
mod tests {
    use umpteen::{exec, Result};

    #[test]
    fn test_hello_world() -> Result<()> {
        exec("print 10;")
    }

    #[test]
    fn test_let_x_equal_10() -> Result<()> {
        exec("let x = 10;")
    }
}
