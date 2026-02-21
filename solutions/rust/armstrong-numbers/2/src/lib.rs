pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num.to_string();
    let n_digits = digits.len() as u32;
    digits
        .chars()
        .map(|c| c.to_digit(10).unwrap()) // Should not panic bc `num` is a number
        .map(|d| d.pow(n_digits))
        .try_fold(0, |sum: u32, num: u32| sum.checked_add(num))
        .is_some_and(|num_sum| num_sum == num)
}