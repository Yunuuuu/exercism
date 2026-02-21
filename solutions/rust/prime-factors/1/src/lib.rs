pub fn factors(mut n: u64) -> Vec<u64> {
    let mut out = Vec::<u64>::new();
    let mut factors: u64 = 2;
    while n > 1 {
        while n % factors == 0 {
            n /= factors;
            out.push(factors)
        }
        factors += 1
    }
    out
}
