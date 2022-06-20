use bfc::*;

use std::{
    env,
    fs::read_to_string,
    io::{stdin, stdout},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Unexpected number of args provided. Should only receive one arg as a path to a .bf file");
    }

    let program = read_to_string(args.get(1).unwrap()).unwrap();

    let ops = parser::parse(&program).unwrap();

    vm::run(&mut stdin(), &mut stdout(), ops).unwrap();
}
