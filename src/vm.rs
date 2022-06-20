use crate::error::Error;
use crate::op::Op;

use std::io::{Read, Write};

pub fn run(reader: &mut impl Read, writer: &mut impl Write, ops: Vec<Op>) -> Result<(), Error> {
    let mut reader = reader.bytes();

    let mut data = [0 as u8; 30000];
    let mut data_ptr = 0;
    let mut pc = 0;

    while let Some(op) = ops.get(pc) {
        let current_data = &mut data[data_ptr];

        match op {
            Op::PtrInc => data_ptr += 1,
            Op::PtrDec => data_ptr -= 1,
            // For some reason tests like bizzfuzz fail in debug mode (where
            // overflows result in panics).
            Op::Inc => *current_data = current_data.wrapping_add(1),
            Op::Dec => *current_data = current_data.wrapping_sub(1),
            Op::Write => {
                write!(writer, "{}", *current_data as char)?;
            }
            Op::Read => {
                if let Some(byte) = reader.next() {
                    *current_data = byte.unwrap();
                }
            }
            Op::JumpIfZero(i) => {
                if *current_data == 0 {
                    pc = *i;
                }
            }
            Op::JumpIfNotZero(i) => {
                if *current_data != 0 {
                    pc = *i;
                }
            }
        };
        pc += 1;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    use std::{fs::read_to_string, path::PathBuf};

    const CARGO_DIR: &str = env!("CARGO_MANIFEST_DIR");

    // Runs a .bf file at `path` and compares the stdout against `${path}.stdout`.
    fn test_bf(path: &str) {
        let inp_path: PathBuf = [CARGO_DIR, path].iter().collect();
        let stdin_path: PathBuf = [CARGO_DIR, &format!("{}.stdin", path)].iter().collect();
        let expected_stdout_path: PathBuf =
            [CARGO_DIR, &format!("{}.stdout", path)].iter().collect();

        let inp = read_to_string(inp_path).unwrap();
        let stdin = read_to_string(stdin_path).unwrap_or_else(|_| "".to_string());

        let expected_output = read_to_string(expected_stdout_path).unwrap();

        let ops = parse(&inp).unwrap();

        let mut result = vec![];

        run(&mut stdin.as_bytes(), &mut result, ops).unwrap();

        let stdout = std::str::from_utf8(&result).unwrap().to_string();

        assert_eq!(stdout, expected_output);
    }

    #[test]
    fn hello_world() {
        test_bf("examples/hello_world.bf");
    }

    #[test]
    fn bizzfuzz() {
        test_bf("examples/bizzfuzz.bf");
    }

    // TODO: Get find some way to terminate programs that read from stdin.
    // #[test]
    // fn cat() {
    //     test_bf("examples/cat.bf");
    // }

    // This test is slow.
    #[test]
    #[ignore]
    fn mandelbrot() {
        test_bf("examples/mandelbrot.bf");
    }
}
