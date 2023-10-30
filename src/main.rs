use ump::{
    bytecode::{Argument, Instruction},
    chunk::Chunk,
    exec, run,
    value::Value,
};

fn sample_programs() {
    let programs = ["", "\n\n\n\n", ";;;;", ",", "25", "let x = 10;"];
    programs.iter().for_each(|x| exec(x));
}

fn main() {
    let mut chunk = Chunk::new();
    let st = "Hello world".to_string();
    let constant = chunk.add_constant(Value::String(Box::new(String::from("Hello World"))));
    chunk.write_inst(Instruction::Constant);
    chunk.write_arg(Argument(constant));
    chunk.write_inst(Instruction::Return);
    println!("{:?}", chunk);
    let _ = run(vec![chunk]);
}
