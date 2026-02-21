use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !(nucleotide == 'A' || nucleotide == 'C' || nucleotide == 'G' || nucleotide == 'T') {
        return Err(nucleotide);
    }
    let mut out = 0;
    for ch in dna.chars() {
        if ch == nucleotide {
            out += 1;
        } else if ch == 'A' || ch == 'C' || ch == 'G' || ch == 'T' {
            continue;
        } else {
            return Err(ch);
        }
    }
    Ok(out)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut out: HashMap<char, usize> = ['A', 'C', 'G', 'T'].iter().map(|n| (*n, 0)).collect();
    for ch in dna.chars() {
        out.get_mut(&ch).map(|count| *count += 1).ok_or(ch)?
    }
    Ok(out)
}
