use umpteen::{error::UmpteenError, value::Value, Runtime};

fn main() -> Result<Value, UmpteenError> {
    let mut vm = Runtime::default();
    vm.load_source("let x = 10;")?;
    vm.run()
}
