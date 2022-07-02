#[derive(Debug)]
pub enum Op {
    PtrOffset(i64),
    PtrAdd(usize),
    PtrSub(usize),
    // Let's hope we don't overflow.
    Add(i8),
    PtrInc,
    PtrDec,
    Inc,
    Dec,
    Write,
    Read,
    BracketLeft,
    BracketRight,
    JumpIfZero(usize),
    JumpIfNotZero(usize),
}
