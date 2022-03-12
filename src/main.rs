use std::env;

mod parser;
use parser::*;

mod interpreter;
use interpreter::*;

fn main() -> () {
    let args = env::args().collect::<Vec<String>>();
    let filepath = args.get(1).expect("Please provide a file path");

    let program = std::fs::read_to_string(filepath).unwrap();
    let instructions = parse_tokens(program);

    let mut interpreter = Interpreter::new(instructions);
    interpreter.run();
}
