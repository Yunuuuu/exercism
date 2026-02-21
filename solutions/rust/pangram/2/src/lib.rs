pub fn is_pangram(sentence: &str) -> bool {
    let s = sentence.to_lowercase();
    ('a'..='z').all(|b| s.contains(b))
}
