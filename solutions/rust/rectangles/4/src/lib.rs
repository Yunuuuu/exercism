pub fn count(lines: &[&str]) -> u32 {
    let line_bytes = lines
        .iter()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();
    let mut out: u32 = 0;
    for top_row_index in 0..line_bytes.len() {
        let top_row = unsafe { line_bytes.get_unchecked(top_row_index) };
        'top_left: for left in 0..top_row.len() {
            let top_left_tip = *unsafe { top_row.get_unchecked(left) };
            // skip if it is not a valid rectangle tip
            if top_left_tip != b'+' {
                continue;
            }
            'top_right: for right in (left + 1)..top_row.len() {
                let top_right_tip = *unsafe { top_row.get_unchecked(right) };
                // skip if it is not a valid rectangle tip
                if top_right_tip != b'+' {
                    // skip the whole left tip if it is not a valid rectangle border
                    if top_right_tip != b'-' {
                        continue 'top_left;
                    }
                    continue;
                }
                for bottom_row_index in (top_row_index + 1)..line_bytes.len() {
                    let bottom_row = unsafe { line_bytes.get_unchecked(bottom_row_index) };
                    if let (Some(&bottom_left_tip), Some(&bottom_right_tip)) =
                        (bottom_row.get(left), bottom_row.get(right))
                    {
                        // skip if it is not a valid rectangle tip
                        if bottom_left_tip != b'+' || bottom_right_tip != b'+' {
                            // skip the whole right tip if it is not a valid rectangle border
                            if bottom_left_tip != b'+' && bottom_left_tip != b'|' {
                                continue 'top_right;
                            }
                            if bottom_right_tip != b'+' && bottom_right_tip != b'|' {
                                continue 'top_right;
                            }
                            continue;
                        }
                        if bottom_row[(left + 1)..right]
                            .iter()
                            .all(|&byte| byte == b'-' || byte == b'+')
                        {
                            out += 1;
                        }
                    }
                }
            }
        }
    }
    out
}
