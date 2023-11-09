pub mod umpteen {
    pub mod error;
    pub mod exec;
    pub mod repr;
    pub mod util;
}
pub use umpteen::*;

use rustyline::{config::Configurer, error::ReadlineError};
use umpteen::{exec::interpreter::Interpreter, repr::value::Value};

fn prompt() {
    let version = env!("CARGO_PKG_VERSION");
    println!("Umpteen v{} — 2023", version);
}

pub fn repl() {
    let mut rl = rustyline::DefaultEditor::new().unwrap();
    let _ = rl.load_history("umpteen_history.txt");
    rl.set_auto_add_history(true);

    let mut interpreter = Interpreter::new();
    prompt();

    let mut interrupt = false;

    loop {
        let readline = rl.readline("> ");

        match readline {
            Ok(line) => match interpreter.run(&line) {
                Ok(x) => {
                    if x != Value::Empty {
                        println!("{}", x);
                    }
                }
                Err(e) => eprintln!("{}", e),
            },
            Err(ReadlineError::Interrupted) => {
                if interrupt {
                    break;
                } else {
                    interrupt = true;
                    println!("Ctrl + D to exit, or press Ctrl + C again");
                    continue;
                }
            }
            Err(ReadlineError::Eof) => break,

            _ => (),
        }

        interrupt = false;
    }

    let _ = rl.save_history("umpteen_history.txt");
}
