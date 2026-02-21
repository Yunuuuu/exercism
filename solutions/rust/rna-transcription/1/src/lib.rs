#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    strand: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    strand: String,
}

const DNA_BASE: [char; 4] = ['A', 'T', 'G', 'C'];
const RNA_BASE: [char; 4] = ['A', 'U', 'G', 'C'];

fn new_sequence(sequence: &str, candidate: [char; 4]) -> Result<String, usize> {
    sequence
        .chars()
        .position(|ch| !candidate.contains(&ch))
        .map_or(Ok(sequence.to_string()), |pos| Err(pos))
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        new_sequence(dna, DNA_BASE).map(|seq| Self { strand: seq })
    }

    pub fn into_rna(self) -> Rna {
        Rna {
            strand: self
                .strand
                .chars()
                .map(|ch| match ch {
                    'A' => 'U',
                    'T' => 'A',
                    'G' => 'C',
                    'C' => 'G',
                    _ => unreachable!()
                })
                .collect::<String>(),
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        new_sequence(rna, RNA_BASE).map(|seq| Self { strand: seq })
    }
}
