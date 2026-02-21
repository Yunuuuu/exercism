pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets = Vec::<char>::with_capacity(string.matches(&['[', '{', '(']).count());
    // let mut brackets = Vec::new();
    for x in string.chars() {
        match x {
            '[' | '{' | '(' => brackets.push(x),
            ']' => {
                if brackets.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if brackets.pop() != Some('{') {
                    return false;
                }
            }
            ')' => {
                if brackets.pop() != Some('(') {
                    return false;
                }
            }
            _ => (),
        }
    }
    brackets.is_empty()
}
