use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut candidates: HashSet<char> = ('a'..='z').collect();
    sentence.to_ascii_lowercase().chars().for_each(|ch| {
        candidates.remove(&ch);
    });
    println!("{candidates:?}");
    candidates.is_empty()
}