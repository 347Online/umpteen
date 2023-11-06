use umpteen::{exec::runtime::Runtime, error::UmpteenError};

fn main() {
    let mut vm = Runtime::default();
    match vm.run("let x = 10; print x;") {
        Ok(value) => println!("Result: {}", value),
        Err(e) => UmpteenError::report(e),
    }
}
