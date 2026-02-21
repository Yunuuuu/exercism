pub fn reply(message: &str) -> &str {
    let msg = message.trim();
    if msg.ends_with('?') {
        if is_yelling(msg.trim_end_matches("?")) {
            "Calm down, I know what I'm doing!"
        } else {
            "Sure."
        }
    } else if is_yelling(msg) {
        "Whoa, chill out!"
    } else if msg.is_empty() {
        "Fine. Be that way!"
    } else {
        "Whatever."
    }
}

fn is_yelling(message: &str) -> bool {
    let mut letters = message.chars().filter(|x| x.is_alphabetic());
    letters.clone().count() > 0 && letters.all(|c| c.is_uppercase())
}