pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (0 .. limit)
        .into_iter()
        .filter(|num| factors.iter().filter(|f| **f != 0).any(|factor| num % factor == 0))
        .sum()
}
