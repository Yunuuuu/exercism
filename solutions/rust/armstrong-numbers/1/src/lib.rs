pub fn is_armstrong_number(num: u32) -> bool {
    let num_units: Vec<u32> = num
        .to_string()
        .chars()
        .map(|s| s.to_digit(10).unwrap())
        .collect();

    let mut power: u32 = 1;
    loop {
        let mut sum: u32 = 0;
        for num_unit in &num_units {
            if let Some(v) = sum.checked_add(num_unit.pow(power)) {
                sum = v;
            } else {
                return false;
            }
        }
        if sum < num && sum == 1 { 
            return false;
        } else if sum < num {
            power += 1;
            continue;
        } if sum == num {
            return true;
        } else {
            return false;
        }
    }
}
