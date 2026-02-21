use itertools::Itertools;
use std::collections::{HashMap, HashSet};

// The core idea is to transform equation before permutations testing, to calculate it as quickly as possible.
// The simplest data model for testing would be just a list of letter factors (coefficiens),
//     so that factors multiplied by letter values and summized will give 0 for correct solution.
// For that, we should got through the equation, sum and remember factors per letter,
//     determined by the letter position in the word (x1, x10, x100, etc).
// After "==" we should change the sign of the factors, and it's convenient to parse reversed input string.
// Sorting letters/factors also impact on performance, as I've found.
// Additionally, we have to check found solution against "no zero first" rule,
//     so we need to know which letters occurs at the first position in an encoded number.
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut sign = 1;
    let mut coefficiens = HashMap::new();
    let mut none_zero = HashSet::new();
    let mut letter_group: Vec<char> = Vec::new();

    let mut it = input
        .chars()
        .filter(|ch| !ch.is_ascii_whitespace())
        .peekable();
    // parse coefficiens for each letter ----------------------
    while let Some(ch) = it.next() {
        match ch {
            '=' | '-' => {
                sign = -1;
            }
            '+' => {
                sign = 1;
            }
            'A'..='Z' => {
                if letter_group.is_empty() {
                    none_zero.insert(ch);
                }
                letter_group.push(ch);
            }
            _ => panic!("wrong equation"),
        }
        // check if next one is the next group
        match it.peek() {
            Some('=') | Some('-') | Some('+') | None => {
                for (pos, ch) in letter_group.iter().rev().enumerate() {
                    let value = coefficiens.entry(*ch).or_insert(0);
                    *value += sign * 10_i64.pow(pos as u32);
                }
                letter_group.clear();
            }
            _ => (),
        }
    }

    let (keys, coefficiens): (Vec<char>, Vec<i64>) = coefficiens.into_iter().unzip();
    // find the position of the first letter which should be the non-zero keys.
    let non_zero_pos = keys
        .iter()
        .enumerate()
        .filter_map(|(pos, key)| {
            if none_zero.contains(key) {
                Some(pos)
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();

    (0..=9).permutations(coefficiens.len()).find_map(|perm| {
        if non_zero_pos.iter().any(|&pos| perm[pos] == 0) {
            return None;
        } else if perm
            .iter()
            .enumerate()
            .fold(0, |sum, (pos, value)| sum + value * coefficiens[pos])
            .eq(&0)
        {
            return Some(
                perm.iter()
                    .enumerate()
                    .map(|(pos, v)| (keys[pos], *v as u8))
                    .collect(),
            );
        } else {
            return None;
        }
    })
}
