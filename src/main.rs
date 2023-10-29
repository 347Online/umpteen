use ump::{error::UmpResult, lex, token::print_tokens};

fn try_lexing() -> UmpResult<()> {
    let sources = vec!["", "\n\n\n\n", ";;;;", ","];

    for src in sources {
        let _ = print_tokens(lex(src)?);
    }

    Ok(())
}

fn main() {
    if let Err(e) = try_lexing() {
        println!("{}", e)
    }
}
