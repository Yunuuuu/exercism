pub fn nth(n: u32) -> u32 {
    let mut num = 1;
    'outer:
    for _ in 0..=n {
        loop {
            num += 1;
            if is_prime(num) {
                continue 'outer;
            }
        }
    }
    return num;
}

fn is_prime(x: u32) -> bool {
    match x {
        0 | 1 => false,
        2 | 3 => true,
        _ => (2..x).all(|ii| (x % ii) > 0),
    }
}
