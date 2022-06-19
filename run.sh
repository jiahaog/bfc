# !/bin/bash

set -euo pipefail

readonly BF_PROGRAM="$1"

readonly TMP_DIR=target/bf
mkdir -p "$TMP_DIR"

cargo run --bin=bfc -- "$BF_PROGRAM" > "$TMP_DIR/bf.asm"
nasm -f elf "$TMP_DIR/bf.asm"
ld -m elf_i386 "$TMP_DIR/bf.o" -o "$TMP_DIR/bf"


$TMP_DIR/bf
