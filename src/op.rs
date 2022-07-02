#[derive(Debug)]
pub enum Op {
    PtrOffset(i64),
    // Let's hope we don't overflow.
    Add(i8),
    Write,
    Read,
    JumpIfZero(usize),
    JumpIfNotZero(usize),
}
