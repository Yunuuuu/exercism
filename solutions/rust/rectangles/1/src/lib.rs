pub fn count(lines: &[&str]) -> u32 {
    let line_bytes = lines
        .into_iter()
        .map(|line| line.as_bytes().into_iter().collect::<Vec<&u8>>())
        .collect::<Vec<Vec<&u8>>>();
    let mut out: u32 = 0;
    for top_row_index in 0..line_bytes.len() {
        let top_row = unsafe { line_bytes.get_unchecked(top_row_index) };
        for top_left in 0..top_row.len() {
            let top_left_tip = unsafe { top_row.get_unchecked(top_left) };
            // skip if it is not a valid rectangle tip
            if **top_left_tip != b'+' {
                continue;
            }
            for top_right in (top_left + 1)..top_row.len() {
                let top_right_tip = unsafe { top_row.get_unchecked(top_right) };
                // skip if it is not a valid rectangle tip or the midpoint is not valid rectangle border
                if **top_right_tip != b'+'
                    || !top_row[(top_left + 1)..top_right]
                        .iter()
                        .all(|byte| **byte == b'+' || **byte == b'-')
                {
                    continue;
                }
                for bottom_row_index in (top_row_index + 1)..line_bytes.len() {
                    let bottom_row = unsafe { line_bytes.get_unchecked(bottom_row_index) };
                    for bottom_left in 0..bottom_row.len() {
                        let bottom_left_tip = unsafe { bottom_row.get_unchecked(bottom_left) };
                        // skip if it is not a valid rectangle tip
                        if **bottom_left_tip != b'+' || top_left != bottom_left {
                            continue;
                        }
                        // skip if it is not a valid rectangle border
                        if !((top_row_index + 1)..bottom_row_index)
                            .into_iter()
                            .all(|row_index| {
                                let byte = unsafe {
                                    line_bytes.get_unchecked(row_index).get_unchecked(top_left)
                                };
                                **byte == b'+' || **byte == b'|'
                            })
                        {
                            continue;
                        }
                        for bottom_right in (bottom_left + 1)..bottom_row.len() {
                            let bottom_right_tip =
                                unsafe { bottom_row.get_unchecked(bottom_right) };
                            if **bottom_right_tip != b'+'
                                || top_right != bottom_right
                                || !bottom_row[(bottom_left + 1)..bottom_right]
                                    .iter()
                                    .all(|byte| **byte == b'+' || **byte == b'-')
                            {
                                continue;
                            }
                            // skip if it is not a valid rectangle border
                            if !((top_row_index + 1)..bottom_row_index).into_iter().all(
                                |row_index| {
                                    let byte = unsafe {
                                        line_bytes.get_unchecked(row_index).get_unchecked(top_right)
                                    };
                                    **byte == b'+' || **byte == b'|'
                                },
                            ) {
                                continue;
                            }
                            out += 1;
                        }
                    }
                }
            }
        }
    }
    out
}
