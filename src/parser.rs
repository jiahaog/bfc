use crate::error::Error;
use crate::op::Op;

use std::collections::HashMap;

/// Internal "IR" for the parser.
///
/// This is a superset of `crate::op::Op` to reduce the API surface of the latter.
#[derive(Debug)]
enum ParseOp {
    PtrOffset(i64),
    Add(i8),
    Write,
    Read,
    BracketLeft,
    BracketRight,
}

pub fn parse(inp: &str) -> Result<Vec<Op>, Error> {
    let unopt = inp
        .chars()
        .filter(|char| matches!(char, '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']'))
        .map(|char| {
            use ParseOp::*;

            match char {
                '>' => PtrOffset(1),
                '<' => PtrOffset(-1),
                '+' => Add(1),
                '-' => Add(-1),
                '.' => Write,
                ',' => Read,
                '[' => BracketLeft,
                ']' => BracketRight,
                x => panic!(
                    "Unexpected char '{}' received which should have been filtered out.",
                    x
                ),
            }
        })
        .collect();

    let opt = optimize(unopt);
    let result = to_opcodes(opt)?;

    Ok(result)
}

fn optimize(ops: Vec<ParseOp>) -> Vec<ParseOp> {
    use ParseOp::*;

    // Merge consecutive instructions.
    ops.into_iter().fold(vec![], |mut acc, op| {
        if acc.is_empty() {
            acc.push(op);
            return acc;
        }

        let prev = acc.pop().unwrap();

        match (prev, op) {
            (PtrOffset(left), PtrOffset(right)) => acc.push(PtrOffset(left + right)),
            (Add(left), Add(right)) => acc.push(Add(left + right)),

            // Cases we don't merge for brevity:
            // - Write and Read because it's not common for this to be repeated.
            (prev, op) => {
                acc.push(prev);
                acc.push(op);
            }
        };
        acc
    })
}

/// Converts the parser "IR" into `crate::op::Op`.
///
/// This is done by setting the jump location of each bracket to an offset.
///
/// After doing so, ordering of elements in the returned vector should not be
/// changed.
fn to_opcodes(ops: Vec<ParseOp>) -> Result<Vec<Op>, Error> {
    let jump_table = bracket_jump_table(&ops)?;

    let result = ops
        .into_iter()
        .enumerate()
        .map(|(i, op)| {
            use ParseOp::*;

            match op {
                PtrOffset(x) => Op::PtrOffset(x),
                Add(x) => Op::Add(x),
                Write => Op::Write,
                Read => Op::Read,
                BracketLeft => Op::JumpIfZero(jump_table.get(&i).unwrap().1),
                BracketRight => Op::JumpIfNotZero(jump_table.get(&i).unwrap().0),
            }
        })
        .collect();
    Ok(result)
}

/// Returns a mapping of bracket chars to the matching left and right parentheses.
fn bracket_jump_table(ops: &Vec<ParseOp>) -> Result<HashMap<usize, (usize, usize)>, Error> {
    let mut stack = vec![];
    let mut pairs = HashMap::new();

    for (i, op) in ops.iter().enumerate() {
        match op {
            &ParseOp::BracketLeft => stack.push(i),
            &ParseOp::BracketRight => {
                if let Some(open_index) = stack.pop() {
                    assert!(open_index < i);
                    let pair = (open_index, i);
                    pairs.insert(open_index, pair);
                    pairs.insert(i, pair);
                } else {
                    return Err(Error::BracketMismatch);
                }
            }
            _ => (),
        }
    }

    if stack.is_empty() {
        Ok(pairs)
    } else {
        Err(Error::BracketMismatch)
    }
}
