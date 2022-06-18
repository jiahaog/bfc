use std::io::{stdin, stdout, Read, Write};

fn main() {
    let ops = parse("+[,.]").unwrap();

    run(&mut stdin(), &mut stdout(), ops).unwrap();
}

const DEBUGGING: bool = false;

fn parse(inp: &str) -> Result<Vec<Op>, Error> {
    inp.chars()
        .filter(|char| matches!(char, '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']'))
        .map(|char| char.try_into())
        .collect()
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
            Op::JumpFwd => {
                if *current_data == 0 {
                    let mut matching_count = 1 as usize;

                    pc += ops[(pc + 1)..]
                        .iter()
                        .position(|op| {
                            if op == &Op::JumpFwd {
                                matching_count += 1;
                                false
                            } else if op == &Op::JumpBwd {
                                matching_count -= 1;
                                matching_count == 0
                            } else {
                                false
                            }
                        })
                        .unwrap();
                }
            }
            Op::JumpBwd => {
                if *current_data != 0 {
                    let mut matching_count = 1 as usize;

                    pc = ops[..pc]
                        .iter()
                        .rposition(|op| {
                            if op == &Op::JumpBwd {
                                matching_count += 1;
                                false
                            } else if op == &Op::JumpFwd {
                                matching_count -= 1;
                                matching_count == 0
                            } else {
                                false
                            }
                        })
                        .unwrap();
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
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Op {
    PtrInc,
    PtrDec,
    Inc,
    Dec,
    Write,
    Read,
    JumpFwd,
    JumpBwd,
}

impl TryFrom<char> for Op {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Op::*;

        match value {
            '>' => Ok(PtrInc),
            '<' => Ok(PtrDec),
            '+' => Ok(Inc),
            '-' => Ok(Dec),
            '.' => Ok(Write),
            ',' => Ok(Read),
            '[' => Ok(JumpFwd),
            ']' => Ok(JumpBwd),
            unknown => Err(Error::InvalidChar(unknown)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::read_to_string, path::PathBuf};

    const CARGO_DIR: &str = env!("CARGO_MANIFEST_DIR");

    fn interpret(path: &str) {
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
        interpret("examples/hello_world.bf");
    }

    #[test]
    fn bizzfuzz() {
        interpret("examples/bizzfuzz.bf");
    }

    // TODO: Get find some way to terminate these programs.
    // #[test]
    // fn cat() {
    //     interpret("examples/cat.bf");
    // }
}
