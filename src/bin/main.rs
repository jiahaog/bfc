use std::env;
use std::path::Path;

use bfc::toolchain;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Unexpected number of args provided. Usage: bfc infile outfile");
    }

    let program_path = Path::new(args.get(1).unwrap()).to_owned();
    let output_path = Path::new(args.get(2).unwrap()).to_owned();

    toolchain::run(program_path, output_path).unwrap();
}
