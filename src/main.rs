use umpteen::{
    repr::{chunk::Chunk, value::Value},
    vm::Vm,
    Result,
};

fn main() -> Result<Value> {
    let mut chunk = Chunk::new();
    chunk.write_arg(256);
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
