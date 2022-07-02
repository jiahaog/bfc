use crate::op::Op;

pub fn compile(ops: Vec<Op>) -> String {
    let mut ins = String::new();

    // ESI is the data ptr.
    for (i, op) in ops.into_iter().enumerate() {
        match op {
            Op::PtrInc => ins.push_str("    add esi, 1\n"),
            Op::PtrDec => ins.push_str("    sub esi, 1\n"),
            Op::Inc => ins.push_str("    add byte [data_array+esi], 1\n"),
            Op::Dec => ins.push_str("    sub byte [data_array+esi], 1\n"),
            Op::Write => ins.push_str(
                "
    ; Get ready for SYSCALL_WRITE

    ; ecx needs to be data_array[eax + eax]
    mov ecx, esi ; buf
    add ecx, data_array

    mov edx, 1 ; count
    mov ebx, 1 ; fd STDOUT_FILENO
    mov eax, 4 ; SYSCALL_WRITE
    int 80h
",
            ),
            Op::Read => ins.push_str(&format!(
                "
    ; Get ready for SYSCALL_READ

    mov eax, 3 ; SYSCALL_READ
    mov ebx, 1 ; fd STDIN_FILENO

    ; Point ecx at the element in the data_array
    mov ecx, data_array ; buf
    add ecx, esi

    mov edx, 1 ; count
    int 80h
            "
            )),
            Op::JumpIfZero(dest) => ins.push_str(&format!(
                "
    cmp byte [data_array+esi], 0
    je jump_dest_{}
jump_dest_{}:
            ",
                dest, i,
            )),
            Op::JumpIfNotZero(dest) => ins.push_str(&format!(
                "
    cmp byte [data_array+esi], 0
    jne jump_dest_{}
jump_dest_{}:
            ",
                dest, i,
            )),
        };
    }

    format!(
        "
SECTION .data
data_array times 30000 dw 0

SECTION .text
global  _start

_start:
{}

    mov ebx, 0 ; error_code
    mov eax, 1 ; SYSCALL_EXIT
    int 80h",
        ins
    )
}
