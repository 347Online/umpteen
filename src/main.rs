use umpteen::{
    error::UmpteenError, exec::interpreter::Interpreter, repr::value::Value, util::report,
};

fn main() {
    let src = "let x = \"Hello World\"; print x;";

    match run(src) {
        Ok(value) => println!("Result: {}", value),
        Err(e) => report(e),
    }
}

fn run(src: &str) -> Result<Value, UmpteenError> {
    let mut interpreter = Interpreter::new();
    interpreter.run(src)
}
