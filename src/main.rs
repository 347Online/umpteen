use umpteen::{
    error::UmpteenError,
    exec::{interpreter::Interpreter, lexer::Lexer, parse::Parser},
    repr::value::Value,
    util::report,
};

fn main() {
    let src = "let x = \"\"";

    match run(src) {
        Ok(value) => println!("Result: {}", value),
        Err(e) => report(e),
    }
}

fn run(src: &str) -> Result<Value, UmpteenError> {
    let lexer = Lexer::new(src);
    let tokens = lexer.scan();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;

    let mut interpreter = Interpreter::new();
    interpreter.interpret(ast)
}
