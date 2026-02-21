pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut out: Vec<(usize, usize)> = Vec::new();
    for (row_num, row) in input.iter().enumerate() {
        for (col_num, val) in row.iter().enumerate() {
            if row.iter().all(|x| x <= val)
                && input
                    .iter()
                    .map(|r| r[col_num])
                    .all(|col_value| col_value >= *val)
            {
                out.push((row_num, col_num));
            }
        }
    }
    out
}
