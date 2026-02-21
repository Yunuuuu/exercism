use std::collections::HashSet;

/// Determines whether the supplied string is a valid ISBN number
pub fn check(word: &str) -> bool {
    let mut seen = HashSet::new();
    word.to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| seen.insert(c))
}