use itertools::izip;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use tools::{DnaNucleotide, DNA, FASTA};

type ProfileMatrix = HashMap<DnaNucleotide, Vec<usize>>;

pub fn generate_profile_matrix(seqs: FASTA) -> ProfileMatrix {
    let dna_seqs: Vec<DNA> = seqs.values().cloned().collect();
    let length = dna_seqs[0].len();
    (0..length)
        .map(|i| {
            let position: DNA = dna_seqs.iter().map(|seq| &seq[i]).cloned().collect();
            position
        })
        .fold(ProfileMatrix::new(), |mut acc, position| {
            for nucleotide in DnaNucleotide::iter() {
                acc.entry(nucleotide)
                    .or_insert(Vec::new())
                    .push(position.iter().filter(|base| base == &&nucleotide).count());
            }
            acc
        })
}

pub fn get_consensus_string(profile_matrix: &ProfileMatrix) -> DNA {
    izip!(
        profile_matrix.get(&DnaNucleotide::A).unwrap(),
        profile_matrix.get(&DnaNucleotide::T).unwrap(),
        profile_matrix.get(&DnaNucleotide::C).unwrap(),
        profile_matrix.get(&DnaNucleotide::G).unwrap()
    )
    .map(|(a, t, c, g)| {
        let h: HashMap<DnaNucleotide, &usize> = HashMap::from([
            (DnaNucleotide::A, a),
            (DnaNucleotide::T, t),
            (DnaNucleotide::C, c),
            (DnaNucleotide::G, g),
        ]);
        h.iter()
            .max_by(|a, b| a.1.cmp(b.1))
            .map(|(k, _v)| k)
            .unwrap()
            .to_owned()
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use tools::{read_fasta, StringParsable};

    use super::*;

    #[test]
    fn test_generate_profile_matrix() {
        let data = read_fasta("rosalind_cons.txt");
        let answer: ProfileMatrix = ProfileMatrix::from([
            (DnaNucleotide::A, vec![5, 1, 0, 0, 5, 5, 0, 0]),
            (DnaNucleotide::C, vec![0, 0, 1, 4, 2, 0, 6, 1]),
            (DnaNucleotide::G, vec![1, 1, 6, 3, 0, 1, 0, 0]),
            (DnaNucleotide::T, vec![1, 5, 0, 0, 0, 1, 1, 6]),
        ]);
        let test_answer = generate_profile_matrix(data);
        assert_eq!(test_answer, answer);
    }

    #[test]
    fn test_get_consensus_string() {
        let data = read_fasta("rosalind_cons.txt");
        let profile_matrix = generate_profile_matrix(data);
        let test_answer = get_consensus_string(&profile_matrix);
        assert_eq!(test_answer, DNA::parse_string("ATGCAACT"));
    }

    #[test]
    fn test_answer() {
        let data = read_fasta("rosalind_cons(1).txt");
        let profile_matrix = generate_profile_matrix(data);
        let consensus_string = get_consensus_string(&profile_matrix);
        println!("{}", consensus_string.to_string());
        for nucleotide in DnaNucleotide::iter() {
            let freqs = profile_matrix.get(&nucleotide).unwrap();
            print!("{}:", nucleotide);
            for b in freqs {
                print!(" {b}");
            }
            println!();
        }
    }
}
