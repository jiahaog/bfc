use std::{
    collections::HashMap,
    env,
    fs::read_to_string,
    io::{stdin, stdout, Read, Write},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Unexpected number of args provided. Should only receive one arg as a path to a .bf file");
    }

    let program = read_to_string(args.get(1).unwrap()).unwrap();

    let ops = parse(&program).unwrap();

    run(&mut stdin(), &mut stdout(), ops).unwrap();
}

const DEBUGGING: bool = false;

fn parse(inp: &str) -> Result<Vec<Op>, Error> {
    let chars: Vec<char> = inp
        .chars()
        .filter(|char| matches!(char, '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']'))
        .collect();

    let jump_table = bracket_jump_table(&chars)?;

    chars
        .into_iter()
        .enumerate()
        .map(|(i, char)| {
            use Op::*;

            match char {
                '>' => Ok(PtrInc),
                '<' => Ok(PtrDec),
                '+' => Ok(Inc),
                '-' => Ok(Dec),
                '.' => Ok(Write),
                ',' => Ok(Read),
                '[' => Ok(JumpIfZero(jump_table.get(&i).unwrap().1)),
                ']' => Ok(JumpIfNotZero(jump_table.get(&i).unwrap().0)),
                unknown => Err(Error::InvalidChar(unknown)),
            }
        })
        .collect()
}

// Returns a mapping of bracket chars to the matching left and right parentheses.
fn bracket_jump_table(chars: &Vec<char>) -> Result<HashMap<usize, (usize, usize)>, Error> {
    let mut stack = vec![];
    let mut pairs = HashMap::new();

    for (i, char) in chars.iter().enumerate() {
        if char == &'[' {
            stack.push(i);
        } else if char == &']' {
            if let Some(open_index) = stack.pop() {
                assert!(open_index < i);
                let pair = (open_index, i);
                pairs.insert(open_index, pair);
                pairs.insert(i, pair);
            } else {
                return Err(Error::BracketMismatch);
            }
        }
    }

    if stack.is_empty() {
        Ok(pairs)
    } else {
        Err(Error::BracketMismatch)
    }
}

fn run(reader: &mut impl Read, writer: &mut impl Write, ops: Vec<Op>) -> Result<(), Error> {
    let mut reader = reader.bytes();

    let mut data = [0 as u8; 30000];
    let mut data_ptr = 0;
    let mut pc = 0;

    while let Some(op) = ops.get(pc) {
        if DEBUGGING {
            println!("========================================");
            println!("data {:?}", data);
            println!(
                "data  {}^{}",
                std::iter::repeat("   ").take(data_ptr).collect::<String>(),
                data_ptr
            );
            println!("ins {:?}", ops);
            println!(
                "ins {}^{}",
                std::iter::repeat(" ").take(pc).collect::<String>(),
                pc
            );
        }

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

#[derive(Debug)]
enum Error {
    InvalidChar(char),
    IoError(std::io::Error),
    BracketMismatch,
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

#[derive(Debug)]
enum Op {
    PtrInc,
    PtrDec,
    Inc,
    Dec,
    Write,
    Read,
    JumpIfZero(usize),
    JumpIfNotZero(usize),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::read_to_string, path::PathBuf};

    const CARGO_DIR: &str = env!("CARGO_MANIFEST_DIR");

    // Runs a .bf file at `path` and compares the stdout against `${path}.stdout`.
    fn test_bf(path: &str) {
        let inp_path: PathBuf = [CARGO_DIR, path].iter().collect();
        let stdin_path: PathBuf = [CARGO_DIR, &format!("{}.stdin", path)].iter().collect();
        let expected_stdout_path: PathBuf =
            [CARGO_DIR, &format!("{}.stdout", path)].iter().collect();

        let inp = read_to_string(inp_path).unwrap();
        let stdin = dbg!(read_to_string(stdin_path).unwrap_or_else(|_| "".to_string()));

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
