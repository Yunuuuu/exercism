/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        if is_palindrome(value) {
            Some(Palindrome(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
       self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut out = Vec::new();
    let mut value;
    for x in min..=max {
        for y in min..=max {
            value = x * y;
            if is_palindrome(value) {
                out.push(value)
            }
        }
    }
    if out.is_empty() {
        return None;
    }
    out.sort();
    let min = out.first().cloned().unwrap();
    let max = out.last().cloned().unwrap();
    Some((Palindrome(min), Palindrome(max)))
}

fn is_palindrome(value: u64) -> bool {
    if value % 10 == 0 {
        return false;
    };
    let mut reverse = 0;
    let mut r = value;
    while r > 0 {
        reverse = reverse * 10 + r % 10;
        r /= 10;
    }
    value == reverse
}

