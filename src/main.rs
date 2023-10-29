use ump::lex;

fn main() {
    let source = "\n\n\n\n";
    match lex(source) {
        Ok(tokens) => println!("Tokens: {:?}", tokens),
        Err(e) => println!("{}", e),
    }
}
