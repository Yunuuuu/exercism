pub fn encode(source: &str) -> String {
    let mut it = source.chars().peekable();
    let mut acc = String::new();
    let mut count: u32 = 0;
    while let Some(ch) = it.next() {
        count += 1;
        // if next letter is the same with current, do next loop.
        if it.peek() == Some(&ch) {
            continue;
        }
        if count > 1 {
            acc.push_str(&count.to_string());
        }
        count = 0;
        acc.push(ch);
    }
    return acc;
}

pub fn decode(source: &str) -> String {
    let mut it = source.chars();
    let mut acc = String::new();
    let mut count: u32 = 0;
    while let Some(ch) = it.next() {
        if ch.is_ascii_digit() {
            count = count * 10 + ch.to_digit(10).unwrap();
        } else if count > 0 {
            acc.push_str(&ch.to_string().repeat(count as usize));
            count = 0;
        } else {
            acc.push(ch);
        }
    }
    return acc;
}
