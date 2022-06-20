use crate::compiler;
use crate::error::Error;
use crate::parser;

use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

pub fn run(program_path: PathBuf, output_path: PathBuf) -> Result<(), Error> {
    let program = fs::read_to_string(program_path)?;

    let ops = parser::parse(&program)?;

    let asm = compiler::compile(ops);

    let temp_dir = env::temp_dir().join("bfc");
    fs::create_dir_all(temp_dir.clone())?;

    let file_name = output_path.file_name().unwrap();

    let asm_file_path = temp_dir.join(file_name).clone();
    let mut asm_file = fs::File::create(asm_file_path.clone())?;
    write!(asm_file, "{}", asm)?;

    let object_file_path = temp_dir.join(format!("{}.o", file_name.to_str().unwrap()));

    assemble(&asm_file_path, &object_file_path)?;

    link(&object_file_path, &output_path)?;

    Ok(())
}

fn assemble(asm_path: &PathBuf, output_path: &PathBuf) -> Result<(), Error> {
    Command::new("nasm")
        .args([
            "-f",
            "elf",
            "-o",
            output_path.to_str().unwrap(),
            asm_path.to_str().unwrap(),
        ])
        .status()?;
    Ok(())
}

fn link(object_path: &PathBuf, output_path: &PathBuf) -> Result<(), Error> {
    Command::new("ld")
        .args([
            "-m",
            "elf_i386",
            "-o",
            output_path.to_str().unwrap(),
            object_path.to_str().unwrap(),
        ])
        .status()?;
    Ok(())
}
