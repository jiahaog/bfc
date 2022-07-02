use crate::error::Error;
use crate::op::Op;

use std::collections::HashMap;

pub fn parse(inp: &str) -> Result<Vec<Op>, Error> {
    let unopt = inp
        .chars()
        .filter(|char| matches!(char, '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']'))
        .map(|char| {
            use Op::*;

            match char {
                '>' => PtrInc,
                '<' => PtrDec,
                '+' => Inc,
                '-' => Dec,
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
    let result = map_bracket_jumps(opt)?;

    Ok(result)
}

fn optimize(ops: Vec<Op>) -> Vec<Op> {
    use Op::*;

    // Merge consecutive instructions.
    ops.into_iter().fold(vec![], |mut acc, op| {
        if acc.is_empty() {
            acc.push(op);
            return acc;
        }

        let prev = acc.pop().unwrap();

        match (prev, op) {
            (PtrInc, PtrInc) => acc.push(PtrOffset(2)),
            (PtrInc, PtrDec) => (),

            (PtrDec, PtrDec) => acc.push(PtrOffset(-2)),
            (PtrDec, PtrInc) => (),

            (PtrOffset(offset), PtrInc) => acc.push(PtrOffset(offset + 1)),
            (PtrOffset(offset), PtrDec) => acc.push(PtrOffset(offset - 1)),

            (Inc, Inc) => acc.push(Add(2)),
            (Inc, Dec) => (),

            (Dec, Dec) => acc.push(Add(-2)),
            (Dec, Inc) => (),

            (Add(num), Inc) => acc.push(Add(num + 1)),
            (Add(num), Dec) => acc.push(Add(num - 1)),

            // Cases we don't merge for brevity:
            // - Write and Read because it's not common for this to be repeated.
            // - Multiple "higher" level instructions like Add and Add. These
            //   instructions are only inserted in this pass for <=ith elements,
            //   and they shouldn't appear.
            (prev, op) => {
                acc.push(prev);
                acc.push(op);
            }
        };
        acc
    })
}

/// Sets the jump location of each bracket to an offset.
///
/// After doing so, elements should not be removed from `ops`.
fn map_bracket_jumps(ops: Vec<Op>) -> Result<Vec<Op>, Error> {
    let jump_table = bracket_jump_table(&ops)?;

    let result = ops
        .into_iter()
        .enumerate()
        .map(|(i, op)| {
            use Op::*;

            match op {
                BracketLeft => JumpIfZero(jump_table.get(&i).unwrap().1),
                BracketRight => JumpIfNotZero(jump_table.get(&i).unwrap().0),
                x => x,
            }
        })
        .collect();
    Ok(result)
}

/// Returns a mapping of bracket chars to the matching left and right parentheses.
fn bracket_jump_table(chars: &Vec<Op>) -> Result<HashMap<usize, (usize, usize)>, Error> {
    let mut stack = vec![];
    let mut pairs = HashMap::new();

    for (i, op) in chars.iter().enumerate() {
        match op {
            &Op::BracketLeft => stack.push(i),
            &Op::BracketRight => {
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
