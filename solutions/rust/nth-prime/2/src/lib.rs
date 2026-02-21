pub fn nth(n: u32) -> u32 {
    (2..)
        .filter(|x| is_prime(*x))
        .nth(n as usize)
        .expect("Cannot find anything")
}

fn is_prime(x: u32) -> bool {
    match x {
        0 | 1 => false,
        2 | 3 => true,
        _ => (2..x).all(|ii| (x % ii) > 0),
    }
}
