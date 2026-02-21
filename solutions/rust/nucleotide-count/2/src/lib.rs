use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut counts = nucleotide_counts(dna)?;
    counts.remove(&nucleotide).ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut out: HashMap<char, usize> = ['A', 'C', 'G', 'T'].iter().map(|n| (*n, 0)).collect();
    for ch in dna.chars() {
        out.get_mut(&ch).map(|count| *count += 1).ok_or(ch)?
    }
    Ok(out)
}
