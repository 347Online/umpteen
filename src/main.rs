use umpteen::{
    bytecode::{Argument, Instruction},
    chunk::Chunk,
    exec, run,
    value::Value,
};

fn sample_programs() {
    let programs = ["", "\n\n\n\n", ";;;;", ",", "25"];
    programs.iter().for_each(|x| exec(x));
}

fn hello_world() {
    exec("print 10.5;");
}

fn let_x_equal_10() {
    exec("let x = 10;");
}

fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.write_val(Value::Number(10.7));
    chunk.write_inst(Instruction::Constant);
    chunk.write_arg(Argument(constant));
    chunk.write_inst(Instruction::Print);
    println!("{:?}", chunk);
    let _ = run(vec![chunk]);
}
