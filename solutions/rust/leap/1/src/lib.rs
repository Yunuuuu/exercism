pub fn is_leap_year(year: u64) -> bool {
    if year.rem_euclid(100).eq(&0) {
        year.rem_euclid(400).eq(&0)
    } else {
        year.rem_euclid(4).eq(&0)
    }
}
