#[derive(Debug)]
pub(crate) enum Op {
    PtrInc,
    PtrDec,
    Inc,
    Dec,
    Write,
    Read,
    JumpIfZero(usize),
    JumpIfNotZero(usize),
}
