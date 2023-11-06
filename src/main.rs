use umpteen::{exec::runtime::Runtime, error::UmpteenError};

fn main() {
    let mut vm = Runtime::default();
    match vm.run("100") {
        Ok(value) => println!("Result: {}", value),
        Err(e) => UmpteenError::report(e),
    }
}
