pub mod umpteen {
    pub mod error;
    pub mod exec;
    pub mod repr;
    pub mod util;
}
pub use umpteen::*;

use rustyline::error::ReadlineError;
use umpteen::exec::interpreter::Interpreter;

pub fn prompt() {
    let version = env!("CARGO_PKG_VERSION");
    println!("Umpteen v{} — 2023", version);
}

pub fn repl() {
    let mut rl = rustyline::DefaultEditor::new().unwrap();

    let mut interpreter = Interpreter::new();
    prompt();

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => match interpreter.run(&line) {
                Ok(result) => println!("{}", result),
                Err(e) => eprintln!("{}", e),
            },
            Err(e) => {
                if let ReadlineError::Eof = e {
                    break;
                }
            }
        }
    }
}
