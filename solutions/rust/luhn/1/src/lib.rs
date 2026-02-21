use std::ops::{Add, Mul, Not, SubAssign};

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.trim();
    code.len().gt(& 1)
        && code
            .chars()
            .filter(|c| c.is_whitespace().not())
            .rev()
            .enumerate()
            .try_fold(0, |sum, (i, value)| -> Option<u32> {
                value.to_digit(10).map(|x| {
                    if i.rem_euclid(2).eq(&1) {
                        let mut val = x.mul(2);
                        if val.gt(&9) {
                            val.sub_assign(9);
                        }
                        sum.add(val)
                    } else {
                        sum.add(x)
                    }
                })
            })
            .map_or(false, |x| x.rem_euclid(10).eq(&0))
}
