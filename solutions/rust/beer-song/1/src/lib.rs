fn verse_beer(n: u32) -> String {
    let n_bottle = match n {
        0 => "no more bottles".to_string(),
        1 => "1 bottle".to_string(),
        _ => format!("{} bottles", n),
    };
    format!("{} of beer", n_bottle)
}

fn verse_pass_beer(n: u32) -> String {
    format!(
        "{}, {} on the wall.",
        match n {
            0 => "Go to the store and buy some more".to_string(),
            1 => "Take it down and pass it around".to_string(),
            _ => "Take one down and pass it around".to_string(),
        },
        match n {
            0 => verse_beer(99),
            _ => verse_beer(n - 1),
        }
    )
}

fn str_to_title(s: &mut str) {
    if let Some(r) = s.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
}

fn verse_current_beer(n: u32) -> String {
    let mut beer = verse_beer(n);
    let beer2 = beer.clone();
    str_to_title(&mut beer);
    format!("{} on the wall, {}.", beer, beer2)
}

pub fn verse(n: u32) -> String {
    format!("{}\n{}\n", verse_current_beer(n), verse_pass_beer(n))
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<_>>()
        .join("\n")
}
