static NEIGBOURHOOD_OFFSETS: &'static [(i32, i32)] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len() as i32;
    minefield
        .into_iter()
        .enumerate()
        .map(|(h, &field)| {
            let width = field.len() as i32;
            let h = h as i32;
            field.as_bytes()
                .into_iter()
                .enumerate()
                .map(|(w, &s)| {
                    let w = w as i32;
                    if s == b'*' {
                        return '*';
                    } else {
                        match NEIGBOURHOOD_OFFSETS
                            .into_iter()
                            .map(|&(ox, oy)| (w + ox, h + oy))
                            .filter(|&(x, y)| (x >= 0 && x < width) && (y >= 0 && y < height))
                            .filter(|&(x, y)| minefield[y as usize].as_bytes()[x as usize ] == b'*')
                            .count()
                        {
                            0 => ' ',
                            n => (n as u8 + '0' as u8) as char,
                        }
                    }
                })
                .collect::<String>()
        })
        .collect()
}
