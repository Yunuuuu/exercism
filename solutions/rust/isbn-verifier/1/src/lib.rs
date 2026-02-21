/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut count = 0;
    let mut ans = 0;
    for c in isbn.chars() {
        if count >= 10 {
            return false;
        } else if c.is_numeric() {
            ans += c.to_digit(10).unwrap() * (10 - count);
            count += 1;
        } else if c == 'X' {
            ans += 10;
            count += 1;
        } else if c == '-' {
            continue;
        } else {
            return false;
        }
    }
    (count == 10) && (ans % 11 == 0)
}