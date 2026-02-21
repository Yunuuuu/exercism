use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let mut word_vec: Vec<char> = word_lower.chars().collect();
    word_vec.sort();
    possible_anagrams
        .into_iter()
        .filter(|candidate| {
            let other_lower = candidate.to_lowercase();
            let mut other_vec = other_lower.chars().collect::<Vec<char>>();
            other_vec.sort();
            word_lower != other_lower && other_vec == word_vec
        })
        .cloned()
        .collect()
}
