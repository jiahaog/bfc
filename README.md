# bfc

[Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) is an minimal and esoteric language.

For example, the following is a Hello World program. See the link above for more details on how it works, it's actually quite simple.

```bf
++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
```

This package implements a simple compiler for Brainfuck. The compiler generates Intel x86 Assembly, uses `nasm` to assemble and then `ld` to link the result into a x86 ELF executable.

## Dependencies

- [`nasm`](https://www.nasm.us/) assembler
- `ld` - usually already installed
- The Rust Toolchain

Other than the above, no other dependencies are used.

## Usage

### Compiler

```sh
# Compiles examples/hello_world.bf, writes the binary to `hello_world` in the 
# same directory.
$ cargo run --release --quiet examples/hello_world.bf hello_world

$ ./hello_world
Hello World!
```

### Interpreter

An intepreter is also included:

```sh
$ cargo run --release --quiet --bin vm examples/hello_world.bf
Hello World!
```

### Testing

The [`examples/`](examples) directory contains example programs (not written by me) and the expected stdout after execution. This can also be tested programmatically with the following command.

```sh
$ cargo test
```

## Benchmarks

Here is a comparison between the interpreter and the compiled executable for `mandelbrot.bf`.

```sh
$ hyperfine 'cargo run --release --quiet --bin vm examples/mandelbrot.bf' 'cargo run --release --quiet examples/mandelbrot.bf target/mandelbrot && target/mandelbrot'
Benchmark 1: cargo run --release --quiet --bin vm examples/mandelbrot.bf
  Time (mean ± σ):      4.894 s ±  0.022 s    [User: 4.889 s, System: 0.005 s]
  Range (min … max):    4.878 s …  4.950 s    10 runs
 
Benchmark 2: cargo run --release --quiet examples/mandelbrot.bf target/mandelbrot && target/mandelbrot
  Time (mean ± σ):     776.3 ms ±   7.0 ms    [User: 770.5 ms, System: 5.9 ms]
  Range (min … max):   771.4 ms … 792.9 ms    10 runs
 
Summary
  'cargo run --release --quiet examples/mandelbrot.bf target/mandelbrot && target/mandelbrot' ran
    6.30 ± 0.06 times faster than 'cargo run --release --quiet --bin vm examples/mandelbrot.bf'
```

Seems pretty decent, though I didn't put in much effort in optimizations beyond creating a jump table for jump instructions and compressing consecutive instructions. See [`parser.rs`](src/parser.rs) for details.

## Resources

This was mainly an adventure in learning about compilers and assembly since I have no experience in systems programming. I was inspired by the following:

- [Adventures in JIT compilation](https://eli.thegreenplace.net/2017/adventures-in-jit-compilation-part-1-an-interpreter/)
