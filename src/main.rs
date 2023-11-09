use umpteen::{repl, run_file};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        run_file(&args[1]);
    } else {
        repl();
    }
}