use std::ops::Not;

pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(&[' ', '-', '_'][..])
        .flat_map(|s| {
            let all_upper = s.chars().all(|ch| ch.is_uppercase());
            s.char_indices().filter_map(move |(i, ch)| {
                if i.eq(&0) || (all_upper.not() && ch.is_uppercase()) {
                    return Some(ch);
                } else {
                    return None;
                }
            })
        })
        .collect::<String>()
        .to_uppercase()
}
