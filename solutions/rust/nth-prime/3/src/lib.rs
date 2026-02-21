pub fn nth(n: u32) -> u32 {
    (2..)
        .filter(|x| is_prime(*x))
        .nth(n as usize)
        .expect("Cannot find anything")
}

fn is_prime(n: u32) -> bool {
    match n {
        0 | 1 => false,
        2 | 3 => true,
        _ => {
            (2 ..= (n as f64).sqrt() as u32).all(|ii| (n % ii) > 0)
        },
    }
}
