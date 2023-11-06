use umpteen::{repr::value::Value, error::UmpteenError, exec::runtime::Runtime};

fn main() -> Result<Value, UmpteenError> {
    let mut vm = Runtime::default();
    vm.run("let x = 10;")
}
