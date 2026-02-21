pub struct PascalsTriangle {
    inner: Vec<Vec<u32>>
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        if row_count == 0 {
            return PascalsTriangle { inner: Vec::new() };
        }
        let mut out: Vec<Vec<u32>> = Vec::with_capacity(row_count as usize);
        let mut row_value: Vec<u32>;
        for row in 1..=row_count {
            row_value = match row {
                1 => [1].iter().copied().collect(),
                2 => [1, 1].iter().copied().collect(),
                _ => {
                    [1].iter().copied()
                        .chain(out[out.len() - 1].windows(2).map(|chunk| chunk.iter().sum()))
                        .chain([1].iter().copied())
                        .collect()
                }
            };
            out.push(row_value);
        }
        PascalsTriangle { inner: out }

    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.inner.clone()
    }
}
