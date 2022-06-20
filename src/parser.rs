use crate::error::Error;
use crate::op::Op;

use std::collections::HashMap;

pub fn parse(inp: &str) -> Result<Vec<Op>, Error> {
    let chars: Vec<char> = inp
        .chars()
        .filter(|char| matches!(char, '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']'))
        .collect();

    let jump_table = bracket_jump_table(&chars)?;

    chars
        .into_iter()
        .enumerate()
        .map(|(i, char)| {
            use Op::*;

            match char {
                '>' => Ok(PtrInc),
                '<' => Ok(PtrDec),
                '+' => Ok(Inc),
                '-' => Ok(Dec),
                '.' => Ok(Write),
                ',' => Ok(Read),
                '[' => Ok(JumpIfZero(jump_table.get(&i).unwrap().1)),
                ']' => Ok(JumpIfNotZero(jump_table.get(&i).unwrap().0)),
                unknown => Err(Error::InvalidChar(unknown)),
            }
        })
        .collect()
}

// Returns a mapping of bracket chars to the matching left and right parentheses.
fn bracket_jump_table(chars: &Vec<char>) -> Result<HashMap<usize, (usize, usize)>, Error> {
    let mut stack = vec![];
    let mut pairs = HashMap::new();

    for (i, char) in chars.iter().enumerate() {
        if char == &'[' {
            stack.push(i);
        } else if char == &']' {
            if let Some(open_index) = stack.pop() {
                assert!(open_index < i);
                let pair = (open_index, i);
                pairs.insert(open_index, pair);
                pairs.insert(i, pair);
            } else {
                return Err(Error::BracketMismatch);
            }
        }
    }

    if stack.is_empty() {
        Ok(pairs)
    } else {
        Err(Error::BracketMismatch)
    }
}
