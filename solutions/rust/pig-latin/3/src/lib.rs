pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            // Rule 1
            if word.starts_with(['a', 'e', 'i', 'o', 'u'])
                || word.starts_with("xr")
                || word.starts_with("yt")
            {
                format!("{}ay", word)
            // Rule 3: "qu" rule should be applied after consonant sound? 
            // (but the test also ensure "qu" at the very beginning also work)
            // when I remove following condition, test fail.
            } else if word.starts_with("qu") {
                let (left, right) = word.split_at(2);
                format!("{}{}ay", right, left)
            } else {
                // Rule 2
                let (left, right) = word.split_at(translate_consonant(word));
                format!("{}{}ay", right, left)
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn translate_consonant(word: &str) -> usize {
    1 + // Rule 2: check the letter followed by consonant
        translate_consonant_suffix(&word[1..])
}

fn translate_consonant_suffix(word: &str) -> usize {
    if word.starts_with(['a', 'e', 'i', 'o', 'u', 'y']) {
        // Rule 4 or break Rule 2 (no more consonants)
        0
    } else if word.starts_with("qu") {
        // Rule 3: a word starts with a consonant sound followed by "qu"
        2
    } else {
        // Rule 2: multiple consonants
        // remains consonant, add one step and recursively call this function
        1 + translate_consonant_suffix(&word[1..])
    }
}
