use umpteen::{Runtime, report};

fn main() {
    let mut vm = Runtime::new();
    match vm.run() {
        Ok(value) => println!("Result: {}", value),
        Err(e) => report(e),
    }
}
