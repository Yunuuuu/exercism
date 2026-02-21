pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            if word.starts_with(['a', 'e', 'i', 'o', 'u'])
                || word.starts_with("xr")
                || word.starts_with("yt")
            {
                format!("{}ay", word)
            // rule 3: "qu" rule should be applied only after consonant sound ?
            // but when I move "qu" rule into function `translate_consonant_suffix`,
            // test failure
            } else if word.starts_with("qu") {
                let (left, right) = word.split_at(2);
                format!("{}{}ay", right, left)
            } else {
                let (left, right) = word.split_at(translate_consonant(word));
                format!("{}{}ay", right, left)
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn translate_consonant(word: &str) -> usize {
    1 + translate_consonant_suffix(&word[1..])
}

fn translate_consonant_suffix(word: &str) -> usize {
    if word.starts_with(['a', 'e', 'i', 'o', 'u'])
        || word.starts_with("xr")
        || word.starts_with("yt")
        || word.starts_with('y')
    {
        0
    } else if word.starts_with("qu") {
        2
    } else {
        1 + translate_consonant_suffix(&word[1..])
    }
}