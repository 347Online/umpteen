use umpteen::{error::UmpteenError, value::Value, Runtime};

fn main() -> Result<Value, UmpteenError> {
    let mut vm = Runtime::default();
    vm.run("let x = 10;")
}
