use std::{env, fs, path::PathBuf, process::Command};

const CARGO_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[test]
fn hello_world() {
    test_bfc("examples/hello_world.bf")
}

#[test]
fn print_twenty_three() {
    test_bfc("examples/print_twenty_three.bf")
}

#[test]
fn bizzfuzz() {
    test_bfc("examples/bizzfuzz.bf");
}

#[test]
fn mandelbrot() {
    test_bfc("examples/mandelbrot.bf");
}

// TODO: Write a test that uses stdin.
// TODO: Consolidate VM implementation and compiler implementation and use a single set of tests.

fn test_bfc(path: &str) {
    let inp_path: PathBuf = [CARGO_DIR, path].iter().collect();
    let expected_stdout_path: PathBuf = [CARGO_DIR, &format!("{}.stdout", path)].iter().collect();

    let expected_output = fs::read_to_string(expected_stdout_path).unwrap();

    let path_filename = inp_path.file_name().unwrap();

    let temp_dir = env::temp_dir()
        .join("bfc_integration_tests")
        .join(path_filename);
    fs::create_dir_all(temp_dir.clone()).unwrap();

    let compiled_path = temp_dir.join("compiled");

    bfc::toolchain::run(inp_path, compiled_path.clone()).unwrap();

    let output = Command::new(compiled_path).output().unwrap();

    let stdout = std::str::from_utf8(&output.stdout).unwrap().to_string();

    assert_eq!(stdout, expected_output);
}
