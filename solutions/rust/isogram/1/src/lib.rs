use std::collections::HashSet;

/// Determines whether the supplied string is a valid ISBN number
pub fn check(candidate: &str) -> bool {
    let mut total = HashSet::new();
    for ch in candidate.to_ascii_lowercase().chars() {
        if ch == '-' || ch == ' ' {
            continue;
        } else if total.contains(&ch) {
            return false;
        } else {
            total.insert(ch);
        }
    }
    return true;
}
