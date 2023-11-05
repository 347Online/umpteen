use umpteen::{value::Value, Runtime, error::UmpteenError};

fn main() -> Result<Value, UmpteenError> {
    let mut vm = Runtime::new();
    vm.load_source(r#"print "Hello World";"#)?;
    vm.run()
}
