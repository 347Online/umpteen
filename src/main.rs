use umpteen::{
    repr::{chunk::Chunk, value::Value, Result},
    vm::Vm,
};

fn main() -> Result<Value> {
    let mut chunk = Chunk::new();
    chunk.write_bytes(256_i32.to_be_bytes());
    let num = i32::from_be_bytes(chunk.load_bytes::<4>()?);
    println!("{:?}", num);
    println!("{:?}", chunk);
    let mut vm = Vm::new();
    vm.write_chunk(chunk);
    vm.exec()
}

#[cfg(test)]
mod tests {
    use umpteen::Umpteen;

    #[test]
    fn test_hello_world() {
        let _ = Umpteen::run("print 10;").unwrap();
    }

    #[test]
    fn test_let_x_equal_10() {
        let _ = Umpteen::run("let x = 10;").unwrap();
    }
}
