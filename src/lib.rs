#![feature(let_chains, box_patterns)]

pub mod umpteen {
    pub mod error;
    pub mod exec;
    pub mod repr;
    pub(crate) mod util;
}

pub(crate) use umpteen::util;
pub use umpteen::{error, exec, repr};

use rustyline::error::ReadlineError;
use umpteen::{error::UmpteenError, exec::interpreter::Interpreter, repr::value::Value};

pub fn repl() {
    let mut rl = rustyline::DefaultEditor::new().unwrap();
    let _ = rl.load_history("umpteen_history");

    let mut umpteen = Interpreter::new();
    prompt();

    let mut interrupt = false;

    loop {
        let readline = rl.readline("> ");

        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(&line);
                handle(umpteen.run(&line))
            }
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

    let _ = rl.save_history("umpteen_history");
}

pub fn run_file(path: &str) {
    let code = match std::fs::read_to_string(path) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let mut umpteen = Interpreter::new();
    handle(umpteen.run(&code));
}

fn prompt() {
    let version = env!("CARGO_PKG_VERSION");
    println!("Umpteen v{} — 2023", version);
}

fn handle(result: Result<Value, UmpteenError>) {
    match result {
        Ok(value) => {
            if value != Value::Empty {
                println!("{}", value);
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}
