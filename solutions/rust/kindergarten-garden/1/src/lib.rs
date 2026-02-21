use std::ops::{Add, Mul};

const STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let pos = STUDENTS
        .into_iter()
        .position(|s| student == s)
        .expect("Unknown student")
        .mul(2);
    diagram
        .lines()
        .flat_map(|line| {
            line[pos..= pos.add(1)].chars().map(|c| match c {
                'G' => "grass",
                'C' => "clover",
                'R' => "radishes",
                _ => "violets",
            })
        })
        .collect()
}
