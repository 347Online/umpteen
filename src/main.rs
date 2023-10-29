use ump::exec;

fn main() {
    let programs = ["", "\n\n\n\n", ";;;;", ",", "let x = 10;"];

    programs.iter().for_each(|x| exec(x));
}
